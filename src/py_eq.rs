use pyo3::prelude::*;
use pyo3::types::{PyBytes, PyList, PyString};

pub trait PyEq {
    fn py_eq(&self, other: &Self) -> bool;
}

impl PyEq for Py<PyString> {
    fn py_eq(&self, other: &Self) -> bool {
        Python::attach(|py| self.bind(py).as_any().eq(other.bind(py)).unwrap_or(false))
    }
}

impl PyEq for Py<PyBytes> {
    fn py_eq(&self, other: &Self) -> bool {
        Python::attach(|py| self.bind(py).as_any().eq(other.bind(py)).unwrap_or(false))
    }
}

impl PyEq for Py<PyList> {
    fn py_eq(&self, other: &Self) -> bool {
        Python::attach(|py| self.bind(py).as_any().eq(other.bind(py)).unwrap_or(false))
    }
}

impl<T: PyEq> PyEq for Option<T> {
    fn py_eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Some(a), Some(b)) => a.py_eq(b),
            (None, None) => true,
            _ => false,
        }
    }
}
