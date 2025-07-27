use core::str;
use pyo3::exceptions::{PyIndexError, PyValueError};
use pyo3::prelude::*;
use pyo3::types::{PyByteArray, PyBytes, PyString, PyStringMethods};
use pyo3::PyResult;
use std::fmt;

pub struct Cursor<'a> {
    pub buffer: &'a mut [u8],
    pub index: usize,
}

impl<'a> Cursor<'a> {
    pub fn new(buffer: &'a Bound<'_, PyByteArray>, index: usize) -> Self {
        Self {
            buffer: unsafe { buffer.as_bytes_mut() },
            index,
        }
    }

    /// Ensures that the buffer has at least the given number of bytes available.
    pub fn require(&self, length: usize) -> PyResult<()> {
        let available = self.buffer.len() - self.index;
        if available < length {
            return Err(PyIndexError::new_err(format!(
                "Insufficient bytes: {available} < {length}"
            )));
        }
        Ok(())
    }
}

#[derive(Clone, Copy, PartialEq, Eq, FromPyObject, IntoPyObject)]
pub struct VariableByteInteger(u32);

impl VariableByteInteger {
    pub fn new(value: u32) -> Self {
        assert!(value < 1 << 28);
        Self(value)
    }

    pub fn value(self) -> u32 {
        self.0
    }
}

impl fmt::Display for VariableByteInteger {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

pub trait Readable {
    fn read(cursor: &mut Cursor<'_>) -> PyResult<Self>
    where
        Self: Sized;
}

impl Readable for u8 {
    fn read(cursor: &mut Cursor<'_>) -> PyResult<Self> {
        cursor.require(1)?;
        let result = cursor.buffer[cursor.index];
        cursor.index += 1;
        Ok(result)
    }
}

impl Readable for u16 {
    fn read(cursor: &mut Cursor<'_>) -> PyResult<Self> {
        cursor.require(2)?;
        let result = u16::from_be_bytes(
            cursor.buffer[cursor.index..cursor.index + 2]
                .try_into()
                .unwrap(),
        );
        cursor.index += 2;
        Ok(result)
    }
}

impl Readable for u32 {
    fn read(cursor: &mut Cursor<'_>) -> PyResult<Self> {
        cursor.require(4)?;
        let result = u32::from_be_bytes(
            cursor.buffer[cursor.index..cursor.index + 4]
                .try_into()
                .unwrap(),
        );
        cursor.index += 4;
        Ok(result)
    }
}

impl Readable for bool {
    fn read(cursor: &mut Cursor<'_>) -> PyResult<Self> {
        cursor.require(1)?;
        let byte = cursor.buffer[cursor.index];
        cursor.index += 1;
        match byte {
            0 => Ok(false),
            1 => Ok(true),
            _ => Err(PyValueError::new_err("Malformed bytes")),
        }
    }
}

impl Readable for VariableByteInteger {
    fn read(cursor: &mut Cursor<'_>) -> PyResult<Self> {
        let mut multiplier = 1;
        let mut result = 0;
        for _ in 0..4 {
            cursor.require(1)?;
            result += (cursor.buffer[cursor.index] & 0x7f) as u32 * multiplier;
            multiplier *= 128;
            cursor.index += 1;
            if (cursor.buffer[cursor.index - 1] & 0x80) == 0 {
                return Ok(VariableByteInteger(result));
            }
        }
        Err(PyValueError::new_err("Malformed bytes"))
    }
}

// TODO: Remove, replaced with PyBytes
impl Readable for Vec<u8> {
    fn read(cursor: &mut Cursor<'_>) -> PyResult<Self> {
        let length = u16::read(cursor)? as usize;
        cursor.require(length)?;
        let result = cursor.buffer[cursor.index..cursor.index + length].to_vec();
        cursor.index += length;
        Ok(result)
    }
}

// TODO: Remove, replaced with PyString
impl Readable for String {
    fn read(cursor: &mut Cursor<'_>) -> PyResult<Self> {
        let value = Vec::<u8>::read(cursor)?;
        String::from_utf8(value).map_err(|_| PyValueError::new_err("Malformed bytes"))
    }
}

impl Readable for Py<PyBytes> {
    fn read(cursor: &mut Cursor<'_>) -> PyResult<Self> {
        let length = u16::read(cursor)? as usize;
        cursor.require(length)?;
        let result = Python::with_gil(|py| {
            PyBytes::new(py, &cursor.buffer[cursor.index..cursor.index + length]).unbind()
        });
        cursor.index += length;
        Ok(result)
    }
}

impl Readable for Py<PyString> {
    fn read(cursor: &mut Cursor<'_>) -> PyResult<Self> {
        let length = u16::read(cursor)? as usize;
        cursor.require(length)?;
        let result = Python::with_gil(|py| {
            PyString::new(py, unsafe {
                str::from_utf8_unchecked(&cursor.buffer[cursor.index..cursor.index + length])
            })
            .unbind()
        });
        cursor.index += length;
        Ok(result)
    }
}

pub trait Writable {
    fn write(&self, cursor: &mut Cursor<'_>);
    fn nbytes(&self) -> usize;
}

impl Writable for u8 {
    fn write(&self, cursor: &mut Cursor<'_>) {
        cursor.buffer[cursor.index] = *self;
        cursor.index += 1;
    }

    fn nbytes(&self) -> usize {
        1
    }
}

impl Writable for u16 {
    fn write(&self, cursor: &mut Cursor<'_>) {
        let bytes = self.to_be_bytes();
        cursor.buffer[cursor.index..cursor.index + 2].copy_from_slice(&bytes);
        cursor.index += 2;
    }

    fn nbytes(&self) -> usize {
        2
    }
}

impl Writable for u32 {
    fn write(&self, cursor: &mut Cursor<'_>) {
        let bytes = self.to_be_bytes();
        cursor.buffer[cursor.index..cursor.index + 4].copy_from_slice(&bytes);
        cursor.index += 4;
    }

    fn nbytes(&self) -> usize {
        4
    }
}

impl Writable for bool {
    fn write(&self, cursor: &mut Cursor<'_>) {
        cursor.buffer[cursor.index] = if *self { 1 } else { 0 };
        cursor.index += 1;
    }

    fn nbytes(&self) -> usize {
        1
    }
}

impl Writable for VariableByteInteger {
    fn write(&self, cursor: &mut Cursor<'_>) {
        let mut remainder = self.0;
        for _ in 0..self.nbytes() {
            let mut byte = (remainder & 0x7F) as u8;
            remainder >>= 7;
            if remainder > 0 {
                byte |= 0x80;
            }
            cursor.buffer[cursor.index] = byte;
            cursor.index += 1;
        }
    }

    fn nbytes(&self) -> usize {
        match self.0 {
            0..=127 => 1,
            128..=16383 => 2,
            16384..=2097151 => 3,
            2097152..=268435455 => 4,
            _ => unreachable!(),
        }
    }
}

impl Writable for &[u8] {
    fn write(&self, cursor: &mut Cursor<'_>) {
        let length = self.len();
        (length as u16).write(cursor);
        cursor.buffer[cursor.index..cursor.index + length].copy_from_slice(self);
        cursor.index += length;
    }

    fn nbytes(&self) -> usize {
        self.len() + 2
    }
}

impl Writable for &str {
    fn write(&self, cursor: &mut Cursor<'_>) {
        self.as_bytes().write(cursor);
    }

    fn nbytes(&self) -> usize {
        self.len() + 2
    }
}

impl Writable for Py<PyBytes> {
    fn write(&self, cursor: &mut Cursor<'_>) {
        Python::with_gil(|py| {
            self.bind(py).as_bytes().write(cursor);
        })
    }

    fn nbytes(&self) -> usize {
        Python::with_gil(|py| self.bind(py).as_bytes().nbytes())
    }
}

impl Writable for &Bound<'_, PyBytes> {
    fn write(&self, cursor: &mut Cursor<'_>) {
        self.as_bytes().write(cursor);
    }

    fn nbytes(&self) -> usize {
        self.as_bytes().nbytes()
    }
}

impl Writable for Py<PyString> {
    fn write(&self, cursor: &mut Cursor<'_>) {
        Python::with_gil(|py| {
            self.bind(py).to_str().unwrap().write(cursor);
        })
    }

    fn nbytes(&self) -> usize {
        Python::with_gil(|py| self.bind(py).to_str().unwrap().nbytes())
    }
}

impl Writable for &Bound<'_, PyString> {
    fn write(&self, cursor: &mut Cursor<'_>) {
        self.to_str().unwrap().write(cursor);
    }
    fn nbytes(&self) -> usize {
        self.to_str().unwrap().nbytes()
    }
}

impl<T: Writable> Writable for Option<T> {
    fn write(&self, cursor: &mut Cursor<'_>) {
        if let Some(ref value) = self {
            value.write(cursor);
        }
    }

    fn nbytes(&self) -> usize {
        match self {
            Some(value) => value.nbytes(),
            None => 0,
        }
    }
}
