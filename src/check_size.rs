use crate::io::VariableByteInteger;
use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;
use pyo3::types::{PyBytes, PyList, PyString};
use pyo3::PyResult;
use std::marker::PhantomData;

const MAX_FIELD_LENGTH: usize = 65535;

pub trait CheckSize {
    fn check_size(&self, py: Python) -> PyResult<()>;
}

impl CheckSize for VariableByteInteger {
    fn check_size(&self, _py: Python) -> PyResult<()> {
        if self.value() >= 1 << 28 {
            return Err(PyValueError::new_err(
                "Variable byte integer must be < 2^28",
            ));
        }
        Ok(())
    }
}

impl CheckSize for Py<PyString> {
    fn check_size(&self, py: Python) -> PyResult<()> {
        if self.bind(py).to_str()?.len() > MAX_FIELD_LENGTH {
            return Err(PyValueError::new_err("String must not exceed 65535 bytes"));
        }
        Ok(())
    }
}

impl CheckSize for Py<PyBytes> {
    fn check_size(&self, py: Python) -> PyResult<()> {
        if self.bind(py).as_bytes().len() > MAX_FIELD_LENGTH {
            return Err(PyValueError::new_err(
                "Binary data must not exceed 65535 bytes",
            ));
        }
        Ok(())
    }
}

impl<T: CheckSize> CheckSize for Option<T> {
    fn check_size(&self, py: Python) -> PyResult<()> {
        if let Some(v) = self {
            v.check_size(py)?;
        }
        Ok(())
    }
}

impl<A: CheckSize, B: CheckSize> CheckSize for (A, B) {
    fn check_size(&self, py: Python) -> PyResult<()> {
        self.0.check_size(py)?;
        self.1.check_size(py)?;
        Ok(())
    }
}

pub struct CheckedList<'a, T>(Option<&'a Py<PyList>>, PhantomData<fn() -> T>);

impl<'a, T> CheckedList<'a, T> {
    pub fn new(list: Option<&'a Py<PyList>>) -> Self {
        Self(list, PhantomData)
    }
}

impl<T> CheckSize for CheckedList<'_, T>
where
    T: CheckSize,
    for<'py, 'a> T: FromPyObject<'py, 'a>,
    for<'py, 'a> <T as FromPyObject<'py, 'a>>::Error: Into<PyErr>,
{
    fn check_size(&self, py: Python) -> PyResult<()> {
        let Some(list) = self.0 else { return Ok(()) };
        for item in list.bind(py).iter() {
            let t: T = item.extract().map_err(Into::into)?;
            t.check_size(py)?;
        }
        Ok(())
    }
}

pub type UserProperties<'a> = CheckedList<'a, (Py<PyString>, Py<PyString>)>;
pub type Patterns<'a> = CheckedList<'a, Py<PyString>>;
pub type SubscriptionIds<'a> = CheckedList<'a, VariableByteInteger>;
