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

    /// Returns the number of bytes left to read/write.
    pub fn len(&self) -> usize {
        self.buffer.len() - self.index
    }

    /// Asserts that the buffer has at least the given number of bytes available.
    pub fn assert(&self, length: usize) -> PyResult<()> {
        if self.len() < length {
            return Err(PyIndexError::new_err(format!(
                "Insufficient bytes: {} < {}",
                self.len(),
                length
            )));
        }
        Ok(())
    }
}

#[derive(Clone, Copy, PartialEq, Eq, FromPyObject, IntoPyObject)]
pub struct VariableByteInteger(pub u32);

impl VariableByteInteger {
    pub fn new(value: u32) -> Self {
        assert!(value < 1 << 28);
        Self(value)
    }

    pub fn get(self) -> u32 {
        self.0
    }
}

impl fmt::Display for VariableByteInteger {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

pub trait Readable {
    fn read<'a>(cursor: &mut Cursor<'a>) -> PyResult<Self>
    where
        Self: Sized;
}

impl Readable for u8 {
    fn read<'a>(cursor: &mut Cursor<'a>) -> PyResult<Self> {
        if cursor.len() < 1 {
            return Err(PyIndexError::new_err("Insufficient bytes"));
        }
        let result = cursor.buffer[cursor.index];
        cursor.index += 1;
        Ok(result)
    }
}

impl Readable for u16 {
    fn read<'a>(cursor: &mut Cursor<'a>) -> PyResult<Self> {
        if cursor.len() < 2 {
            return Err(PyIndexError::new_err("Insufficient bytes"));
        }
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
    fn read<'a>(cursor: &mut Cursor<'a>) -> PyResult<Self> {
        if cursor.len() < 4 {
            return Err(PyIndexError::new_err("Insufficient bytes"));
        }
        let result = u32::from_be_bytes(
            cursor.buffer[cursor.index..cursor.index + 4]
                .try_into()
                .unwrap(),
        );
        cursor.index += 4;
        Ok(result)
    }
}

impl Readable for VariableByteInteger {
    fn read<'a>(cursor: &mut Cursor<'a>) -> PyResult<Self> {
        let mut multiplier = 1;
        let mut result = 0;
        for _ in 0..4 {
            if cursor.len() < 1 {
                return Err(PyIndexError::new_err("Insufficient bytes"));
            }
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
    fn read<'a>(cursor: &mut Cursor<'a>) -> PyResult<Self> {
        let length = u16::read(cursor)? as usize;
        if cursor.len() < length {
            return Err(PyIndexError::new_err("Insufficient bytes"));
        }
        let result = cursor.buffer[cursor.index..cursor.index + length].to_vec();
        cursor.index += length;
        Ok(result)
    }
}

// TODO: Remove, replaced with PyString
impl Readable for String {
    fn read<'a>(cursor: &mut Cursor<'a>) -> PyResult<Self> {
        let value = Vec::<u8>::read(cursor)?;
        String::from_utf8(value).map_err(|_| PyValueError::new_err("Malformed bytes"))
    }
}

impl Readable for Py<PyBytes> {
    fn read<'a>(cursor: &mut Cursor<'a>) -> PyResult<Self> {
        let length = u16::read(cursor)? as usize;
        if cursor.len() < length {
            return Err(PyIndexError::new_err("Insufficient bytes"));
        }
        let result = Python::with_gil(|py| {
            PyBytes::new(py, &cursor.buffer[cursor.index..cursor.index + length]).unbind()
        });
        cursor.index += length;
        Ok(result)
    }
}

impl Readable for Py<PyString> {
    fn read<'a>(cursor: &mut Cursor<'a>) -> PyResult<Self> {
        let length = u16::read(cursor)? as usize;
        if cursor.len() < length {
            return Err(PyIndexError::new_err("Insufficient bytes"));
        }
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
    fn write<'a>(&self, cursor: &mut Cursor<'a>);
    fn size(&self) -> usize;
}

impl Writable for u8 {
    fn write<'a>(&self, cursor: &mut Cursor<'a>) {
        cursor.buffer[cursor.index] = *self;
        cursor.index += 1;
    }

    fn size(&self) -> usize {
        1
    }
}

impl Writable for u16 {
    fn write<'a>(&self, cursor: &mut Cursor<'a>) {
        let bytes = self.to_be_bytes();
        cursor.buffer[cursor.index..cursor.index + 2].copy_from_slice(&bytes);
        cursor.index += 2;
    }

    fn size(&self) -> usize {
        2
    }
}

impl Writable for u32 {
    fn write<'a>(&self, cursor: &mut Cursor<'a>) {
        let bytes = self.to_be_bytes();
        cursor.buffer[cursor.index..cursor.index + 4].copy_from_slice(&bytes);
        cursor.index += 4;
    }

    fn size(&self) -> usize {
        4
    }
}

impl Writable for VariableByteInteger {
    fn write<'a>(&self, cursor: &mut Cursor<'a>) {
        let mut remainder = self.0;
        for _ in 0..self.size() {
            let mut byte = (remainder & 0x7F) as u8;
            remainder >>= 7;
            if remainder > 0 {
                byte |= 0x80;
            }
            cursor.buffer[cursor.index] = byte;
            cursor.index += 1;
        }
    }

    fn size(&self) -> usize {
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
    fn write<'a>(&self, cursor: &mut Cursor<'a>) {
        let length = self.len();
        (length as u16).write(cursor);
        cursor.buffer[cursor.index..cursor.index + length].copy_from_slice(self);
        cursor.index += length;
    }

    fn size(&self) -> usize {
        self.len() + 2
    }
}

impl Writable for &str {
    fn write<'a>(&self, cursor: &mut Cursor<'a>) {
        self.as_bytes().write(cursor);
    }

    fn size(&self) -> usize {
        self.len() + 2
    }
}

impl Writable for Py<PyBytes> {
    fn write<'a>(&self, cursor: &mut Cursor<'a>) {
        Python::with_gil(|py| {
            self.bind(py).as_bytes().write(cursor);
        })
    }

    fn size(&self) -> usize {
        Python::with_gil(|py| self.bind(py).as_bytes().size())
    }
}

impl Writable for &Bound<'_, PyBytes> {
    fn write<'a>(&self, cursor: &mut Cursor<'a>) {
        self.as_bytes().write(cursor);
    }

    fn size(&self) -> usize {
        self.as_bytes().size()
    }
}

impl Writable for Py<PyString> {
    fn write<'a>(&self, cursor: &mut Cursor<'a>) {
        Python::with_gil(|py| {
            self.bind(py).to_str().unwrap().write(cursor);
        })
    }

    fn size(&self) -> usize {
        Python::with_gil(|py| self.bind(py).to_str().unwrap().size())
    }
}

impl Writable for &Bound<'_, PyString> {
    fn write<'a>(&self, cursor: &mut Cursor<'a>) {
        self.to_str().unwrap().write(cursor);
    }
    fn size(&self) -> usize {
        self.to_str().unwrap().size()
    }
}

impl<T: Writable> Writable for Option<T> {
    fn write<'a>(&self, cursor: &mut Cursor<'a>) {
        if let Some(ref value) = self {
            value.write(cursor);
        }
    }

    fn size(&self) -> usize {
        match self {
            Some(value) => value.size(),
            None => 0,
        }
    }
}
