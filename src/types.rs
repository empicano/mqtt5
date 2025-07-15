use num_enum::TryFromPrimitive;
use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;
use pyo3::PyResult;
use std::fmt;

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

macro_rules! int_enum {
    ( $name:ident { $($field:ident = $value:expr),* $(,)? } ) => {
        #[pyclass(eq, str, rename_all = "SCREAMING_SNAKE_CASE", module = "mqtt5")]
        #[derive(Copy, Clone, PartialEq, Eq, TryFromPrimitive)]
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
    };
}

int_enum! {
    QoS {
        AtMostOnce = 0,
        AtLeastOnce = 1,
        ExactlyOnce = 2,
    }
}

int_enum! {
    RetainHandling {
        SendAlways = 0,
        SendIfSubscriptionNotExists = 1,
        SendNever = 2,
    }
}
