use crate::io::{Cursor, Readable, Writable};
use num_enum::TryFromPrimitive;
use pyo3::exceptions::{PyIndexError, PyValueError};
use pyo3::prelude::*;
use pyo3::PyResult;

macro_rules! reason_code {
    ( $name:ident { $($field:ident / $py:literal = $value:expr),* $(,)? } ) => {
        #[pyclass(frozen, eq)]
        #[derive(Copy, Clone, PartialEq, Eq, TryFromPrimitive)]
        #[repr(u8)]
        pub enum $name {
            $(#[pyo3(name = $py)] $field = $value,)*
        }

        #[pymethods]
        impl $name {
            #[new]
            pub fn new(value: u8) -> PyResult<Self> {
                Self::try_from(value).map_err(|e| PyValueError::new_err(e.to_string()))
            }
        }

        impl Readable for $name {
            fn read<'a>(cursor: &mut Cursor<'a>) -> PyResult<Self> {
                if cursor.len() < 1 {
                    return Err(PyIndexError::new_err("Insufficient bytes"));
                }
                let result = cursor.buffer[cursor.index];
                cursor.index += 1;
                Self::new(result)
            }
        }

        impl Writable for $name {
            fn write<'a>(&self, cursor: &mut Cursor<'a>) {
                cursor.buffer[cursor.index] = *self as u8;
                cursor.index += 1;
            }

            fn size(&self) -> usize {
                1
            }
        }
    };
}

reason_code! {
    ConnAckReasonCode {
        Success / "SUCCESS" = 0,
        UnspecifiedError / "UNSPECIFIED_ERROR" = 128,
        MalformedPacket / "MALFORMED_PACKET" = 129,
        ProtocolError / "PROTOCOL_ERROR" = 130,
        ImplementationSpecificError / "IMPLEMENTATION_SPECIFIC_ERROR" = 131,
        UnsupportedProtocolVersion / "UNSUPPORTED_PROTOCOL_VERSION" = 132,
        ClientIdNotValid / "CLIENT_ID_NOT_VALID" = 133,
        BadUserNameOrPassword / "BAD_USER_NAME_OR_PASSWORD" = 134,
        NotAuthorized / "NOT_AUTHORIZED" = 135,
        ServerUnavailable / "SERVER_UNAVAILABLE" = 136,
        ServerBusy / "SERVER_BUSY" = 137,
        Banned / "BANNED" = 138,
        BadAuthenticationMethod / "BAD_AUTHENTICATION_METHOD" = 140,
        TopicNameInvalid / "TOPIC_NAME_INVALID" = 144,
        PacketTooLarge / "PACKET_TOO_LARGE" = 149,
        QuotaExceeded / "QUOTA_EXCEEDED" = 151,
        PayloadFormatInvalid / "PAYLOAD_FORMAT_INVALID" = 153,
        RetainNotSupported / "RETAIN_NOT_SUPPORTED" = 154,
        QualityNotSupported / "QUALITY_NOT_SUPPORTED" = 155,
        UseAnotherServer / "USE_ANOTHER_SERVER" = 156,
        ServerMoved / "SERVER_MOVED" = 157,
        ConnectionRateExceeded / "CONNECTION_RATE_EXCEEDED" = 159,
    }
}

reason_code! {
    PubAckReasonCode {
        Success / "SUCCESS" = 0,
        NoMatchingSubscribers/"NO_MATCHING_SUBSCRIBERS" = 16,
        UnspecifiedError / "UNSPECIFIED_ERROR" = 128,
        ImplementationSpecificError / "IMPLEMENTATION_SPECIFIC_ERROR" = 131,
        NotAuthorized / "NOT_AUTHORIZED" = 135,
        TopicNameInvalid / "TOPIC_NAME_INVALID" = 144,
        PacketIdInUse / "PACKET_ID_IN_USE" = 145,
        QuotaExceeded / "QUOTA_EXCEEDED" = 151,
        PayloadFormatInvalid / "PAYLOAD_FORMAT_INVALID" = 153,
    }
}

reason_code! {
    PubRecReasonCode {
        Success / "SUCCESS" = 0,
        NoMatchingSubscribers/ "NO_MATCHING_SUBSCRIBERS" = 16,
        UnspecifiedError / "UNSPECIFIED_ERROR"= 128,
        ImplementationSpecificError / "IMPLEMENTATION_SPECIFIC_ERROR" = 131,
        NotAuthorized / "NOT_AUTHORIZED" = 135,
        TopicNameInvalid / "TOPIC_NAME_INVALID" = 144,
        PacketIdInUse / "PACKET_ID_IN_USE" = 145,
        QuotaExceeded / "QUOTA_EXCEEDED" = 151,
        PayloadFormatInvalid / "PAYLOAD_FORMAT_INVALID" = 153,
    }
}

reason_code! {
    PubCompReasonCode {
        Success / "SUCCESS" = 0,
        PacketIdNotFound / "PACKET_ID_NOT_FOUND" = 146,
    }
}

reason_code! {
    SubAckReasonCode {
        GrantedQoSAtMostOnce / "GRANTED_QOS_AT_MOST_ONCE" = 0,
        GrantedQoSAtLeastOnce / "GRANTED_QOS_AT_LEAST_ONCE" = 1,
        GrantedQoSExactlyOnce / "GRANTED_QOS_EXACTLY_ONCE" = 2,
        UnspecifiedError / "UNSPECIFIED_ERROR" = 128,
        ImplementationSpecificError / "IMPLEMENTATION_SPECIFIC_ERROR" = 131,
        NotAuthorized / "NOT_AUTHORIZED" = 135,
        TopicFilterInvalid / "TOPIC_FILTER_INVALID" = 143,
        PacketIdInUse / "PACKET_ID_IN_USE" = 145,
        QuotaExceeded / "QUOTA_EXCEEDED" = 151,
        SharedSubscriptionsNotSupported / "SHARED_SUBSCRIPTIONS_NOT_SUPPORTED" = 158,
        SubscriptionIdsNotSupported / "SUBSCRIPTION_IDS_NOT_SUPPORTED" = 161,
        WildcardSubscriptionsNotSupported / "WILDCARD_SUBSCRIPTIONS_NOT_SUPPORTED" = 162,
    }
}

reason_code! {
    DisconnectReasonCode {
        NormalDisconnection / "NORMAL_DISCONNECTION" = 0,
        DisconnectWithWillMessage / "DISCONNECT_WITH_WILL_MESSAGE" = 4,
        UnspecifiedError / "UNSPECIFIED_ERROR" = 128,
        MalformedPacket / "MALFORMED_PACKET" = 129,
        ProtocolError / "PROTOCOL_ERROR" = 130,
        ImplementationSpecificError / "IMPLEMENTATION_SPECIFIC_ERROR" = 131,
        NotAuthorized / "NOT_AUTHORIZED" = 135,
        ServerBusy / "SERVER_BUSY" = 137,
        ServerShuttingDown / "SERVER_SHUTTING_DOWN" = 139,
        KeepAliveTimeout / "KEEP_ALIVE_TIMEOUT" = 141,
        SessionTakenOver / "SESSION_TAKEN_OVER" = 142,
        TopicFilterInvalid / "TOPIC_FILTER_INVALID" = 143,
        TopicNameInvalid / "TOPIC_NAME_INVALID" = 144,
        ReceiveMaximumExceeded / "RECEIVE_MAXIMUM_EXCEEDED" = 147,
        TopicAliasInvalid / "TOPIC_ALIAS_INVALID" = 148,
        PacketTooLarge / "PACKET_TOO_LARGE" = 149,
        MessageRateTooHigh / "MESSAGE_RATE_TOO_HIGH" = 150,
        QuotaExceeded / "QUOTA_EXCEEDED" = 151,
        AdministrativeAction / "ADMINISTRATIVE_ACTION" = 152,
        PayloadFormatInvalid / "PAYLOAD_FORMAT_INVALID" = 153,
        RetainNotSupported / "RETAIN_NOT_SUPPORTED" = 154,
        QoSNotSupported / "QOS_NOT_SUPPORTED" = 155,
        UseAnotherServer / "USE_ANOTHER_SERVER" = 156,
        ServerMoved / "SERVER_MOVED" = 157,
        SharedSubscriptionsNotSupported / "SHARED_SUBSCRIPTIONS_NOT_SUPPORTED" = 158,
        ConnectionRateExceeded / "CONNECTION_RATE_EXCEEDED" = 159,
        MaximumConnectTime / "MAXIMUM_CONNECT_TIME" = 160,
        SubscriptionIdsNotSupported / "SUBSCRIPTION_IDS_NOT_SUPPORTED" = 161,
        WildcardSubscriptionsNotSupported / "WILDCARD_SUBSCRIPTIONS_NOT_SUPPORTED" = 162,
    }
}
