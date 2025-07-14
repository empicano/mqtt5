use crate::io::{Cursor, Readable, VariableByteInteger, Writable};
use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;
use pyo3::types::{PyBytes, PyString};
use pyo3::PyResult;

// Helper trait for cloning property fields that may contain Python objects
pub trait CloneWithGil {
    fn clone_with_gil(&self, py: Python) -> Self;
}

impl CloneWithGil for u8 {
    fn clone_with_gil(&self, _py: Python) -> Self {
        *self
    }
}

impl CloneWithGil for u16 {
    fn clone_with_gil(&self, _py: Python) -> Self {
        *self
    }
}

impl CloneWithGil for u32 {
    fn clone_with_gil(&self, _py: Python) -> Self {
        *self
    }
}

impl CloneWithGil for String {
    fn clone_with_gil(&self, _py: Python) -> Self {
        self.clone()
    }
}

impl CloneWithGil for Vec<u8> {
    fn clone_with_gil(&self, _py: Python) -> Self {
        self.clone()
    }
}

impl CloneWithGil for VariableByteInteger {
    fn clone_with_gil(&self, _py: Python) -> Self {
        *self
    }
}

impl CloneWithGil for Py<PyBytes> {
    fn clone_with_gil(&self, py: Python) -> Self {
        self.clone_ref(py)
    }
}

impl CloneWithGil for Py<PyString> {
    fn clone_with_gil(&self, py: Python) -> Self {
        self.clone_ref(py)
    }
}

impl<T: CloneWithGil> CloneWithGil for Option<T> {
    fn clone_with_gil(&self, py: Python) -> Self {
        self.as_ref().map(|x| x.clone_with_gil(py))
    }
}

pub trait PyEq {
    fn equals(&self, other: &Self) -> bool;
}

impl PyEq for u8 {
    fn equals(&self, other: &Self) -> bool {
        self == other
    }
}

impl PyEq for u16 {
    fn equals(&self, other: &Self) -> bool {
        self == other
    }
}

impl PyEq for u32 {
    fn equals(&self, other: &Self) -> bool {
        self == other
    }
}

impl PyEq for VariableByteInteger {
    fn equals(&self, other: &Self) -> bool {
        self == other
    }
}

impl PyEq for Py<PyBytes> {
    fn equals(&self, other: &Self) -> bool {
        Python::with_gil(|py| self.bind(py).as_any().eq(other.bind(py)).unwrap_or(false))
    }
}

impl PyEq for Py<PyString> {
    fn equals(&self, other: &Self) -> bool {
        Python::with_gil(|py| self.bind(py).as_any().eq(other.bind(py)).unwrap_or(false))
    }
}

impl<T: PyEq> PyEq for Option<T> {
    fn equals(&self, other: &Self) -> bool {
        match (self, other) {
            (Some(a), Some(b)) => a.equals(b),
            (None, None) => true,
            _ => false,
        }
    }
}

macro_rules! properties {
    (
        $name:ident {
            $($field:ident: $property_type:ty = $property_id:expr),* $(,)?
        }
    ) => {
        #[pyclass(eq, get_all)]
        pub struct $name {
            $(pub $field: Option<$property_type>,)*
        }

        #[pymethods]
        impl $name {
            #[new]
            #[pyo3(signature = (*, $($field=None),*))]
            fn new($($field: Option<$property_type>),*) -> Self {
                Self {
                    $($field,)*
                }
            }
        }

        impl Clone for $name {
            fn clone(&self) -> Self {
                Python::with_gil(|py| Self {
                    $(
                        $field: self.$field.clone_with_gil(py),
                    )*
                })
            }
        }

        impl PartialEq for $name {
            fn eq(&self, other: &Self) -> bool {
                $(self.$field.equals(&other.$field) &&)* true
            }
        }

        impl Default for $name {
            fn default() -> Self {
                Self {
                    $($field: None,)*
                }
            }
        }

        impl Readable for $name {
            fn read<'a>(cursor: &mut Cursor<'a>) -> PyResult<Self> {
                let length = VariableByteInteger::read(cursor)?.get() as usize;
                let start = cursor.index;
                let mut instance = Self::default();
                while cursor.index - start < length {
                    let id = VariableByteInteger::read(cursor)?.get();
                    match id {
                        $(
                            $property_id => {
                                let value = <$property_type>::read(cursor)?;
                                instance.$field = Some(value);
                            }
                        )*
                        _ => {
                            return Err(PyValueError::new_err(format!("Invalid property id: {}", id)));
                        }
                    }
                }
                Ok(instance)
            }
        }

        impl Writable for $name {
            fn write<'a>(&self, cursor: &mut Cursor<'a>) {
                let mut size = 0;
                $(
                    if let Some(ref value) = self.$field {
                        size += VariableByteInteger($property_id).size() + value.size();
                    }
                )*
                VariableByteInteger(size as u32).write(cursor);
                $(
                    if let Some(ref value) = self.$field {
                        VariableByteInteger($property_id).write(cursor);
                        value.write(cursor);
                    }
                )*
            }

            fn size(&self) -> usize {
                let mut size = 0;
                $(
                    if let Some(ref value) = self.$field {
                        size += VariableByteInteger($property_id).size() + value.size();
                    }
                )*
                size + VariableByteInteger(size as u32).size()
            }
        }
    };
}

properties! {
    WillProperties {
        payload_format_indicator: u8 = 0x01,
        message_expiry_interval: u32 = 0x02,
        content_type: Py<PyString> = 0x03,
        response_topic: Py<PyString> = 0x08,
        correlation_data: Py<PyBytes> = 0x09,
        will_delay_interval: u32 = 0x18,
    }
}

properties! {
    ConnectProperties {
        session_expiry_interval: u32 = 0x11,
        authentication_method: Py<PyString> = 0x15,
        authentication_data: Py<PyBytes> = 0x16,
        request_problem_information: u8 = 0x17,
        request_response_information: u8 = 0x19,
        receive_maximum: u16 = 0x21,
        topic_alias_maximum: u16 = 0x22,
        maximum_packet_size: u32 = 0x27,
    }
}

properties! {
    ConnAckProperties {
        session_expiry_interval: u32 = 0x11,
        assigned_client_id: Py<PyString> = 0x12,
        server_keep_alive: u16 = 0x13,
        authentication_method: Py<PyString> = 0x15,
        authentication_data: Py<PyBytes> = 0x16,
        response_information: Py<PyString> = 0x1A,
        server_reference: Py<PyString> = 0x1C,
        reason_string: Py<PyString> = 0x1F,
        receive_maximum: u16 = 0x21,
        topic_alias_maximum: u16 = 0x22,
        maximum_qos: u8 = 0x24,
        retain_available: u8 = 0x25,
        maximum_packet_size: u32 = 0x27,
        wildcard_subscription_available: u8 = 0x28,
        subscription_id_available: u8 = 0x29,
        shared_subscription_available: u8 = 0x2A,
    }
}

properties! {
    PublishProperties {
        payload_format_indicator: u8 = 0x01,
        message_expiry_interval: u32 = 0x02,
        content_type: Py<PyString> = 0x03,
        response_topic: Py<PyString> = 0x08,
        correlation_data: Py<PyBytes> = 0x09,
        subscription_id: VariableByteInteger = 0x0B,
        topic_alias: u16 = 0x23,
    }
}

properties! {
    PubAckProperties {
        reason_string: Py<PyString> = 0x1F,
    }
}

properties! {
    PubRecProperties {
        reason_string: Py<PyString> = 0x1F,
    }
}

properties! {
    PubCompProperties {
        reason_string: Py<PyString> = 0x1F,
    }
}

properties! {
    SubscribeProperties {
        subscription_id: VariableByteInteger = 0x0B,
    }
}

properties! {
    SubAckProperties {
        reason_string: Py<PyString> = 0x1F,
    }
}

properties! {
    DisconnectProperties {
        session_expiry_interval: u32 = 0x11,
        server_reference: Py<PyString> = 0x1C,
        reason_string: Py<PyString> = 0x1F,
    }
}
