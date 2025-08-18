use crate::io::{Cursor, Readable, Writable};
use num_enum::TryFromPrimitive;
use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;
use pyo3::PyResult;
use std::fmt;

macro_rules! reason_code {
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

        impl Readable for $name {
            fn read(cursor: &mut Cursor<'_>) -> PyResult<Self> {
                cursor.require(1)?;
                let result = cursor.buffer[cursor.index];
                cursor.index += 1;
                Self::new(result)
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

        impl fmt::Display for $name {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                write!(f, "{}", *self as u8)
            }
        }
    };
}

reason_code! {
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
        BadAuthMethod = 140,
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

reason_code! {
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

reason_code! {
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

reason_code! {
    PubCompReasonCode {
        Success = 0,
        PacketIdNotFound = 146,
    }
}

reason_code! {
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

reason_code! {
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
