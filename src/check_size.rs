use crate::io::VariableByteInteger;
use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;
use pyo3::types::{PyBytes, PyList, PyString};
use pyo3::PyResult;

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
            return Err(PyValueError::new_err("String must be < 65535 bytes"));
        }
        Ok(())
    }
}

impl CheckSize for Py<PyBytes> {
    fn check_size(&self, py: Python) -> PyResult<()> {
        if self.bind(py).as_bytes().len() > MAX_FIELD_LENGTH {
            return Err(PyValueError::new_err("Binary data must be < 65535 bytes"));
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

pub fn check_user_properties_size(py: Python, list: Option<&Py<PyList>>) -> PyResult<()> {
    let Some(list) = list else { return Ok(()) };
    for item in list.bind(py).iter() {
        let (key, value): (Py<PyString>, Py<PyString>) = item.extract()?;
        key.check_size(py)?;
        value.check_size(py)?;
    }
    Ok(())
}

pub fn check_patterns_size(py: Python, list: Option<&Py<PyList>>) -> PyResult<()> {
    let Some(list) = list else { return Ok(()) };
    for item in list.bind(py).iter() {
        let pattern: Py<PyString> = item.extract()?;
        pattern.check_size(py)?;
    }
    Ok(())
}

pub fn check_subscription_ids_size(py: Python, list: Option<&Py<PyList>>) -> PyResult<()> {
    let Some(list) = list else { return Ok(()) };
    for item in list.bind(py).iter() {
        let id: VariableByteInteger = item.extract()?;
        id.check_size(py)?;
    }
    Ok(())
}
