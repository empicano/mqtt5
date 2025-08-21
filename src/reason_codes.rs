use crate::io::{Cursor, Readable, Writable};
use crate::types::py_int_enum;
use num_enum::TryFromPrimitive;
use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;
use pyo3::PyResult;
use std::fmt;

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
