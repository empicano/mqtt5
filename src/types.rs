use crate::io::{ReadCursor, Readable, Writable, WriteCursor};
use num_enum::TryFromPrimitive;
use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;
use pyo3::types::{PyBytes, PyList, PyString};
use pyo3::PyResult;
use std::fmt;

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
    AuthenticationMethod = 21,
    AuthenticationData = 22,
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
        #[pyclass(eq, eq_int, str, rename_all = "SCREAMING_SNAKE_CASE", module = "mqtt5")]
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

            #[getter]
            fn value(&self) -> PyResult<u8> {
                Ok(*self as u8)
            }

            #[getter]
            pub fn name(&self) -> PyResult<String> {
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
                Ok(member_name)
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
            fn read(cursor: &mut ReadCursor<'_>) -> PyResult<Self> {
                cursor.require(1)?;
                let result = Self::new(cursor.buffer[cursor.index])?;
                cursor.index += 1;
                Ok(result)
            }
        }

        impl Writable for $name {
            fn write(&self, cursor: &mut WriteCursor<'_>) {
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

py_int_enum! {
    ConnAckReasonCode {
        Success = 0,
        UnspecifiedError = 128,
        MalformedPacket = 129,
        ProtocolError = 130,
        ImplementationSpecificError = 131,
        UnsupportedProtocolVersion = 132,
        ClientIdNotValid = 133,
        BadUserNameOrPassword = 134,
        NotAuthorized = 135,
        ServerUnavailable = 136,
        ServerBusy = 137,
        Banned = 138,
        BadAuthenticationMethod = 140,
        TopicNameInvalid = 144,
        PacketTooLarge = 149,
        QuotaExceeded = 151,
        PayloadFormatInvalid = 153,
        RetainNotSupported = 154,
        QualityNotSupported = 155,
        UseAnotherServer = 156,
        ServerMoved = 157,
        ConnectionRateExceeded = 159,
    }
}

py_int_enum! {
    PubAckReasonCode {
        Success = 0,
        NoMatchingSubscribers = 16,
        UnspecifiedError = 128,
        ImplementationSpecificError = 131,
        NotAuthorized = 135,
        TopicNameInvalid = 144,
        PacketIdInUse = 145,
        QuotaExceeded = 151,
        PayloadFormatInvalid = 153,
    }
}

py_int_enum! {
    PubRecReasonCode {
        Success = 0,
        NoMatchingSubscribers = 16,
        UnspecifiedError = 128,
        ImplementationSpecificError = 131,
        NotAuthorized = 135,
        TopicNameInvalid = 144,
        PacketIdInUse = 145,
        QuotaExceeded = 151,
        PayloadFormatInvalid = 153,
    }
}

py_int_enum! {
    PubRelReasonCode {
        Success = 0,
        PacketIdNotFound = 146,
    }
}

py_int_enum! {
    PubCompReasonCode {
        Success = 0,
        PacketIdNotFound = 146,
    }
}

py_int_enum! {
    SubAckReasonCode {
        GrantedQosAtMostOnce = 0,
        GrantedQosAtLeastOnce = 1,
        GrantedQosExactlyOnce = 2,
        UnspecifiedError = 128,
        ImplementationSpecificError = 131,
        NotAuthorized = 135,
        TopicFilterInvalid = 143,
        PacketIdInUse = 145,
        QuotaExceeded = 151,
        SharedSubscriptionsNotSupported = 158,
        SubscriptionIdsNotSupported = 161,
        WildcardSubscriptionsNotSupported = 162,
    }
}

py_int_enum! {
    UnsubAckReasonCode {
        Success = 0,
        NoSubscriptionExisted = 17,
        UnspecifiedError = 128,
        ImplementationSpecificError = 131,
        NotAuthorized = 135,
        TopicFilterInvalid = 143,
        PacketIdInUse = 145,
    }
}

py_int_enum! {
    AuthReasonCode {
        Success = 0,
        ContinueAuthentication = 24,
        ReAuthenticate = 25,
    }
}

py_int_enum! {
    DisconnectReasonCode {
        NormalDisconnection = 0,
        DisconnectWithWillMessage = 4,
        UnspecifiedError = 128,
        MalformedPacket = 129,
        ProtocolError = 130,
        ImplementationSpecificError = 131,
        NotAuthorized = 135,
        ServerBusy = 137,
        ServerShuttingDown = 139,
        KeepAliveTimeout = 141,
        SessionTakenOver = 142,
        TopicFilterInvalid = 143,
        TopicNameInvalid = 144,
        ReceiveMaxExceeded = 147,
        TopicAliasInvalid = 148,
        PacketTooLarge = 149,
        MessageRateTooHigh = 150,
        QuotaExceeded = 151,
        AdministrativeAction = 152,
        PayloadFormatInvalid = 153,
        RetainNotSupported = 154,
        QosNotSupported = 155,
        UseAnotherServer = 156,
        ServerMoved = 157,
        SharedSubscriptionsNotSupported = 158,
        ConnectionRateExceeded = 159,
        MaxConnectTime = 160,
        SubscriptionIdsNotSupported = 161,
        WildcardSubscriptionsNotSupported = 162,
    }
}
