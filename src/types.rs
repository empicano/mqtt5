use crate::io::{Cursor, Readable, Writable};
use num_enum::TryFromPrimitive;
use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;
use pyo3::types::{PyBytes, PyString};
use pyo3::PyResult;
use std::fmt;

pub trait PyEq {
    fn py_eq(&self, other: &Self) -> bool;
}

impl PyEq for Py<PyBytes> {
    fn py_eq(&self, other: &Self) -> bool {
        Python::with_gil(|py| self.bind(py).as_any().eq(other.bind(py)).unwrap_or(false))
    }
}

impl PyEq for Py<PyString> {
    fn py_eq(&self, other: &Self) -> bool {
        Python::with_gil(|py| self.bind(py).as_any().eq(other.bind(py)).unwrap_or(false))
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

#[derive(PartialEq, Eq, TryFromPrimitive)]
#[repr(u8)]
pub enum PacketType {
    Connect = 1,
    ConnAck = 2,
    Publish = 3,
    PubAck = 4,
    PubRec = 5,
    PubRel = 6,
    PubComp = 7,
    Subscribe = 8,
    SubAck = 9,
    Unsubscribe = 10,
    UnsubAck = 11,
    PingReq = 12,
    PingResp = 13,
    Disconnect = 14,
    Auth = 15,
}

impl PacketType {
    pub fn new(value: u8) -> PyResult<Self> {
        Self::try_from(value).map_err(|e| PyValueError::new_err(e.to_string()))
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, TryFromPrimitive)]
#[repr(u8)]
pub enum PropertyType {
    PayloadFormatIndicator = 1,
    MessageExpiryInterval = 2,
    ContentType = 3,
    ResponseTopic = 8,
    CorrelationData = 9,
    SubscriptionId = 11,
    SessionExpiryInterval = 17,
    AssignedClientId = 18,
    ServerKeepAlive = 19,
    AuthMethod = 21,
    AuthData = 22,
    RequestProblemInfo = 23,
    WillDelayInterval = 24,
    RequestResponseInfo = 25,
    ResponseInfo = 26,
    ServerReference = 28,
    ReasonStr = 31,
    ReceiveMax = 33,
    TopicAliasMax = 34,
    TopicAlias = 35,
    MaxQoS = 36,
    RetainAvailable = 37,
    UserProperty = 38,
    MaxPacketSize = 39,
    WildcardSubscriptionAvailable = 40,
    SubscriptionIdAvailable = 41,
    SharedSubscriptionAvailable = 42,
}

impl PropertyType {
    pub fn new(value: u8) -> PyResult<Self> {
        Self::try_from(value).map_err(|e| PyValueError::new_err(e.to_string()))
    }
}

macro_rules! py_int_enum {
    ( $name:ident { $($field:ident = $value:expr),* $(,)? } ) => {
        #[pyclass(eq, str, rename_all = "SCREAMING_SNAKE_CASE", module = "mqtt5")]
        #[derive(Clone, Copy, PartialEq, Eq, TryFromPrimitive)]
        #[repr(u8)]
        pub enum $name {
            $($field = $value,)*
        }

        #[pymethods]
        impl $name {
            #[new]
            pub fn new(value: u8) -> PyResult<Self> {
                Self::try_from(value).map_err(|e| PyValueError::new_err(e.to_string()))
            }

            pub fn __repr__(&self) -> String {
                let member_name = match self {
                    $(Self::$field => stringify!($field).to_string(),)*
                }
                .chars()
                .enumerate()
                .flat_map(|(i, c)| {
                    if i > 0 && c.is_uppercase() {
                        vec!['_', c]
                    } else {
                        vec![c.to_ascii_uppercase()]
                    }
                })
                .collect::<String>();
                format!("<{}.{}: {}>", stringify!($name), member_name, *self as u8)
            }
        }

        impl fmt::Display for $name {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                write!(f, "{}", *self as u8)
            }
        }

        impl Readable for $name {
            fn read(cursor: &mut Cursor<'_>) -> PyResult<Self> {
                cursor.require(1)?;
                let result = Self::new(cursor.buffer[cursor.index])?;
                cursor.index += 1;
                Ok(result)
            }
        }

        impl Writable for $name {
            fn write(&self, cursor: &mut Cursor<'_>) {
                cursor.buffer[cursor.index] = *self as u8;
                cursor.index += 1;
            }

            fn nbytes(&self) -> usize {
                1
            }
        }
    };
}

py_int_enum! {
    QoS {
        AtMostOnce = 0,
        AtLeastOnce = 1,
        ExactlyOnce = 2,
    }
}

py_int_enum! {
    RetainHandling {
        SendAlways = 0,
        SendIfSubscriptionNotExists = 1,
        SendNever = 2,
    }
}
