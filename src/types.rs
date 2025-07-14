use num_enum::TryFromPrimitive;
use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;
use pyo3::PyResult;

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

#[pyclass(eq)]
#[derive(Copy, Clone, PartialEq, Eq, TryFromPrimitive)]
#[repr(u8)]
pub enum QoS {
    #[pyo3(name = "AT_MOST_ONCE")]
    AtMostOnce = 0,
    #[pyo3(name = "AT_LEAST_ONCE")]
    AtLeastOnce = 1,
    #[pyo3(name = "EXACTLY_ONCE")]
    ExactlyOnce = 2,
}

impl QoS {
    pub fn new(value: u8) -> PyResult<Self> {
        Self::try_from(value).map_err(|e| PyValueError::new_err(e.to_string()))
    }
}

#[pyclass(eq)]
#[derive(Copy, Clone, PartialEq, Eq, TryFromPrimitive)]
#[repr(u8)]
pub enum RetainHandling {
    #[pyo3(name = "SEND_ALWAYS")]
    SendAlways = 0,
    #[pyo3(name = "SEND_IF_SUBSCRIPTION_NOT_EXISTS")]
    SendIfSubscriptionNotExists = 1,
    #[pyo3(name = "SEND_NEVER")]
    SendNever = 2,
}

impl RetainHandling {
    pub fn new(value: u8) -> PyResult<Self> {
        Self::try_from(value).map_err(|e| PyValueError::new_err(e.to_string()))
    }
}
