use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;
use pyo3::types::PyString;
use pyo3::PyResult;

pub fn check_topic_format(py: Python, topic: &Py<PyString>) -> PyResult<()> {
    let s = topic.bind(py).to_str()?;
    if s.contains('+') || s.contains('#') {
        return Err(PyValueError::new_err("Invalid topic"));
    }
    Ok(())
}

pub fn check_pattern_format(py: Python, pattern: &Py<PyString>) -> PyResult<()> {
    let s = pattern.bind(py).to_str()?;
    let bytes = s.as_bytes();
    for (i, &b) in bytes.iter().enumerate() {
        match b {
            b'#' => {
                if i != bytes.len() - 1 || (i > 0 && bytes[i - 1] != b'/') {
                    return Err(PyValueError::new_err("Invalid topic filter"));
                }
            }
            b'+' => {
                let preceded = i == 0 || bytes[i - 1] == b'/';
                let followed = i == bytes.len() - 1 || bytes[i + 1] == b'/';
                if !preceded || !followed {
                    return Err(PyValueError::new_err("Invalid topic filter"));
                }
            }
            _ => {}
        }
    }
    Ok(())
}
