use core::str;
use num_enum::TryFromPrimitive;
use pyo3::exceptions::{PyIndexError, PyValueError};
use pyo3::prelude::*;
use pyo3::types::{PyByteArray, PyBytes, PyString, PyStringMethods};
use pyo3::PyResult;
use std::fmt;

struct Cursor<'a> {
    buffer: &'a mut [u8],
    index: usize,
}

impl<'a> Cursor<'a> {
    pub fn new(buffer: &'a Bound<'_, PyByteArray>, index: usize) -> Self {
        Self {
            buffer: unsafe { buffer.as_bytes_mut() },
            index,
        }
    }

    /// Returns the number of bytes left to read/write.
    pub fn len(&self) -> usize {
        self.buffer.len() - self.index
    }

    /// Asserts that the buffer has at least the given number of bytes available.
    pub fn assert(&self, length: usize) -> PyResult<()> {
        if self.len() < length {
            return Err(PyIndexError::new_err(format!(
                "Insufficient bytes: {} < {}",
                self.len(),
                length
            )));
        }
        Ok(())
    }
}

#[derive(Clone, Copy, PartialEq, Eq, FromPyObject, IntoPyObject)]
pub struct VariableByteInteger(u32);

impl VariableByteInteger {
    pub fn new(value: u32) -> Self {
        assert!(value < 1 << 28);
        Self(value)
    }

    pub fn get(self) -> u32 {
        self.0
    }
}

impl fmt::Display for VariableByteInteger {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

trait Readable {
    fn read<'a>(cursor: &mut Cursor<'a>) -> PyResult<Self>
    where
        Self: Sized;
}

impl Readable for u8 {
    fn read<'a>(cursor: &mut Cursor<'a>) -> PyResult<Self> {
        if cursor.len() < 1 {
            return Err(PyIndexError::new_err("Insufficient bytes"));
        }
        let result = cursor.buffer[cursor.index];
        cursor.index += 1;
        Ok(result)
    }
}

impl Readable for u16 {
    fn read<'a>(cursor: &mut Cursor<'a>) -> PyResult<Self> {
        if cursor.len() < 2 {
            return Err(PyIndexError::new_err("Insufficient bytes"));
        }
        let result = u16::from_be_bytes(
            cursor.buffer[cursor.index..cursor.index + 2]
                .try_into()
                .unwrap(),
        );
        cursor.index += 2;
        Ok(result)
    }
}

impl Readable for u32 {
    fn read<'a>(cursor: &mut Cursor<'a>) -> PyResult<Self> {
        if cursor.len() < 4 {
            return Err(PyIndexError::new_err("Insufficient bytes"));
        }
        let result = u32::from_be_bytes(
            cursor.buffer[cursor.index..cursor.index + 4]
                .try_into()
                .unwrap(),
        );
        cursor.index += 4;
        Ok(result)
    }
}

impl Readable for VariableByteInteger {
    fn read<'a>(cursor: &mut Cursor<'a>) -> PyResult<Self> {
        let mut multiplier = 1;
        let mut result = 0;
        for _ in 0..4 {
            if cursor.len() < 1 {
                return Err(PyIndexError::new_err("Insufficient bytes"));
            }
            result += (cursor.buffer[cursor.index] & 0x7f) as u32 * multiplier;
            multiplier *= 128;
            cursor.index += 1;
            if (cursor.buffer[cursor.index - 1] & 0x80) == 0 {
                return Ok(VariableByteInteger(result));
            }
        }
        Err(PyValueError::new_err("Malformed bytes"))
    }
}

// TODO: Remove, we don't deal with this type anymore
impl Readable for Vec<u8> {
    fn read<'a>(cursor: &mut Cursor<'a>) -> PyResult<Self> {
        let length = u16::read(cursor)? as usize;
        if cursor.len() < length {
            return Err(PyIndexError::new_err("Insufficient bytes"));
        }
        let result = cursor.buffer[cursor.index..cursor.index + length].to_vec();
        cursor.index += length;
        Ok(result)
    }
}

// TODO: Remove, we don't deal with this type anymore
impl Readable for String {
    fn read<'a>(cursor: &mut Cursor<'a>) -> PyResult<Self> {
        let value = Vec::<u8>::read(cursor)?;
        String::from_utf8(value).map_err(|_| PyValueError::new_err("Malformed bytes"))
    }
}

impl Readable for Py<PyBytes> {
    fn read<'a>(cursor: &mut Cursor<'a>) -> PyResult<Self> {
        let length = u16::read(cursor)? as usize;
        if cursor.len() < length {
            return Err(PyIndexError::new_err("Insufficient bytes"));
        }
        let result = Python::with_gil(|py| {
            PyBytes::new(py, &cursor.buffer[cursor.index..cursor.index + length]).unbind()
        });
        cursor.index += length;
        Ok(result)
    }
}

impl Readable for Py<PyString> {
    fn read<'a>(cursor: &mut Cursor<'a>) -> PyResult<Self> {
        let length = u16::read(cursor)? as usize;
        if cursor.len() < length {
            return Err(PyIndexError::new_err("Insufficient bytes"));
        }
        let result = Python::with_gil(|py| {
            PyString::new(py, unsafe {
                str::from_utf8_unchecked(&cursor.buffer[cursor.index..cursor.index + length])
            })
            .unbind()
        });
        cursor.index += length;
        Ok(result)
    }
}

trait Writable {
    fn write<'a>(&self, cursor: &mut Cursor<'a>);
    fn size(&self) -> usize;
}

impl Writable for u8 {
    fn write<'a>(&self, cursor: &mut Cursor<'a>) {
        cursor.buffer[cursor.index] = *self;
        cursor.index += 1;
    }

    fn size(&self) -> usize {
        1
    }
}

impl Writable for u16 {
    fn write<'a>(&self, cursor: &mut Cursor<'a>) {
        let bytes = self.to_be_bytes();
        cursor.buffer[cursor.index..cursor.index + 2].copy_from_slice(&bytes);
        cursor.index += 2;
    }

    fn size(&self) -> usize {
        2
    }
}

impl Writable for u32 {
    fn write<'a>(&self, cursor: &mut Cursor<'a>) {
        let bytes = self.to_be_bytes();
        cursor.buffer[cursor.index..cursor.index + 4].copy_from_slice(&bytes);
        cursor.index += 4;
    }

    fn size(&self) -> usize {
        4
    }
}

impl Writable for VariableByteInteger {
    fn write<'a>(&self, cursor: &mut Cursor<'a>) {
        let mut remainder = self.0;
        for _ in 0..self.size() {
            let mut byte = (remainder & 0x7F) as u8;
            remainder >>= 7;
            if remainder > 0 {
                byte |= 0x80;
            }
            cursor.buffer[cursor.index] = byte;
            cursor.index += 1;
        }
    }

    fn size(&self) -> usize {
        match self.0 {
            0..=127 => 1,
            128..=16383 => 2,
            16384..=2097151 => 3,
            2097152..=268435455 => 4,
            _ => unreachable!(),
        }
    }
}

impl Writable for &[u8] {
    fn write<'a>(&self, cursor: &mut Cursor<'a>) {
        let length = self.len();
        (length as u16).write(cursor);
        cursor.buffer[cursor.index..cursor.index + length].copy_from_slice(self);
        cursor.index += length;
    }

    fn size(&self) -> usize {
        self.len() + 2
    }
}

impl Writable for &str {
    fn write<'a>(&self, cursor: &mut Cursor<'a>) {
        self.as_bytes().write(cursor);
    }

    fn size(&self) -> usize {
        self.len() + 2
    }
}

impl Writable for Py<PyBytes> {
    fn write<'a>(&self, cursor: &mut Cursor<'a>) {
        Python::with_gil(|py| {
            self.bind(py).as_bytes().write(cursor);
        })
    }

    fn size(&self) -> usize {
        Python::with_gil(|py| self.bind(py).as_bytes().size())
    }
}

impl Writable for &Bound<'_, PyBytes> {
    fn write<'a>(&self, cursor: &mut Cursor<'a>) {
        self.as_bytes().write(cursor);
    }

    fn size(&self) -> usize {
        self.as_bytes().size()
    }
}

impl Writable for Py<PyString> {
    fn write<'a>(&self, cursor: &mut Cursor<'a>) {
        Python::with_gil(|py| {
            self.bind(py).to_str().unwrap().write(cursor);
        })
    }

    fn size(&self) -> usize {
        Python::with_gil(|py| self.bind(py).to_str().unwrap().size())
    }
}

impl Writable for &Bound<'_, PyString> {
    fn write<'a>(&self, cursor: &mut Cursor<'a>) {
        self.to_str().unwrap().write(cursor);
    }
    fn size(&self) -> usize {
        self.to_str().unwrap().size()
    }
}

impl<T: Writable> Writable for Option<T> {
    fn write<'a>(&self, cursor: &mut Cursor<'a>) {
        if let Some(ref value) = self {
            value.write(cursor);
        }
    }

    fn size(&self) -> usize {
        match self {
            Some(value) => value.size(),
            None => 0,
        }
    }
}

trait PyEq {
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

// const PROTOCOL_NAME: &[u8] = b"MQTT";
const PROTOCOL_NAME: &str = "MQTT";
const PROTOCOL_VERSION: u8 = 5;

#[derive(PartialEq, Eq, TryFromPrimitive)]
#[repr(u8)]
enum PacketType {
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

macro_rules! reasons {
    ( $name:ident { $($field:ident / $py:literal = $value:expr),* $(,)? } ) => {
        #[pyclass(eq)]
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

reasons! {
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

reasons! {
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

reasons! {
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

reasons! {
    PubCompReasonCode {
        Success / "SUCCESS" = 0,
        PacketIdNotFound / "PACKET_ID_NOT_FOUND" = 146,
    }
}

reasons! {
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

reasons! {
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

// Helper trait for cloning property fields that may contain Python objects
trait CloneWithGil {
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
                let start = cursor.index;
                let length = VariableByteInteger::read(cursor)?.get() as usize;
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

#[pyclass(eq, get_all)]
pub struct Will {
    topic: Py<PyString>,
    payload: Option<Py<PyBytes>>,
    qos: QoS,
    retain: bool,
    properties: WillProperties,
}

#[pymethods]
impl Will {
    #[new]
    #[pyo3(signature = (
        topic,
        payload=None,
        qos=QoS::AtMostOnce,
        retain=false,
        properties=None,
    ))]
    pub fn new(
        topic: &Bound<'_, PyString>,
        payload: Option<&Bound<'_, PyBytes>>,
        qos: QoS,
        retain: bool,
        properties: Option<WillProperties>,
    ) -> Self {
        Self {
            topic: topic.clone().unbind(),
            payload: payload.map(|x| x.clone().unbind()),
            qos,
            retain,
            properties: properties.unwrap_or_default(),
        }
    }
}

impl Clone for Will {
    fn clone(&self) -> Self {
        Python::with_gil(|py| Self {
            topic: self.topic.clone_ref(py),
            payload: self.payload.as_ref().map(|x| x.clone_ref(py)),
            qos: self.qos,
            retain: self.retain,
            properties: self.properties.clone(),
        })
    }
}

impl PartialEq for Will {
    fn eq(&self, other: &Self) -> bool {
        self.topic.equals(&other.topic)
            && self.payload.equals(&other.payload)
            && self.qos == other.qos
            && self.retain == other.retain
            && self.properties == other.properties
    }
}

#[pyclass(eq, get_all)]
pub struct ConnectPacket {
    client_id: Py<PyString>,
    username: Option<Py<PyString>>,
    password: Option<Py<PyString>>,
    clean_start: bool,
    will: Option<Will>,
    keep_alive: u16,
    properties: ConnectProperties,
}

#[pymethods]
impl ConnectPacket {
    #[new]
    #[pyo3(signature = (
        client_id,
        *,
        username=None,
        password=None,
        clean_start=false,
        will=None,
        keep_alive=0,
        properties=None,
    ))]
    pub fn new(
        py: Python,
        client_id: &Bound<'_, PyString>,
        username: Option<&Bound<'_, PyString>>,
        password: Option<&Bound<'_, PyString>>,
        clean_start: bool,
        will: Option<Will>,
        keep_alive: u16,
        properties: Option<ConnectProperties>,
    ) -> PyResult<Self> {
        Ok(Self {
            client_id: client_id.clone().unbind(),
            username: username.map(|x| x.clone().unbind()),
            password: password.map(|x| x.clone().unbind()),
            clean_start,
            will,
            keep_alive,
            properties: properties.unwrap_or_default(),
        })
    }

    #[pyo3(signature = (buffer, /, *, index=0))]
    fn write(&self, py: Python, buffer: &Bound<'_, PyByteArray>, index: usize) -> PyResult<usize> {
        let remaining_length = PROTOCOL_NAME.size()
            + PROTOCOL_VERSION.size()
            + 0u8.size()
            + self.keep_alive.size()
            + self.properties.size()
            + self.client_id.size()
            + self.will.as_ref().map_or(0, |x| {
                x.properties.size()
                    + x.topic.size()
                    + x.payload.as_ref().map_or(0u16.size(), |x| x.size())
            })
            + self.username.size()
            + self.password.size();
        let var = VariableByteInteger(remaining_length as u32);
        let mut cursor = Cursor::new(buffer, index);
        cursor.assert(1 + var.size() + remaining_length)?;

        // [3.1.1] Fixed header
        let first_byte = (PacketType::Connect as u8) << 4;
        first_byte.write(&mut cursor);
        var.write(&mut cursor);

        // [3.1.2] Variable header
        PROTOCOL_NAME.write(&mut cursor);
        PROTOCOL_VERSION.write(&mut cursor);
        let mut flags = (self.clean_start as u8) << 1;
        if let Some(ref will) = self.will {
            flags |= 0x04;
            flags |= (will.qos as u8) << 3;
            flags |= (will.retain as u8) << 5;
        }
        if self.password.is_some() {
            flags |= 0x40;
        }
        if self.username.is_some() {
            flags |= 0x80;
        }
        flags.write(&mut cursor);
        self.keep_alive.write(&mut cursor);
        self.properties.write(&mut cursor);

        // [3.1.3] Payload
        self.client_id.write(&mut cursor);
        if let Some(ref will) = self.will {
            will.properties.write(&mut cursor);
            will.topic.write(&mut cursor);
            if let Some(ref payload) = will.payload {
                payload.write(&mut cursor);
            } else {
                0u16.write(&mut cursor);
            }
        }
        self.username.write(&mut cursor);
        self.password.write(&mut cursor);

        Ok(cursor.index - index)
    }
}

impl ConnectPacket {
    fn read(
        py: Python,
        cursor: &mut Cursor,
        flags: u8,
        remaining_length: VariableByteInteger,
    ) -> PyResult<Py<Self>> {
        if flags != 0x00 {
            return Err(PyValueError::new_err("Malformed bytes"));
        }

        // [3.1.2] Variable header
        if String::read(cursor)? != PROTOCOL_NAME {
            return Err(PyValueError::new_err("Malformed bytes"));
        }
        if u8::read(cursor)? != PROTOCOL_VERSION {
            return Err(PyValueError::new_err("Malformed bytes"));
        }
        let vh_flags = u8::read(cursor)?;
        let clean_start = (vh_flags & 0x02) == 1;
        let keep_alive = u16::read(cursor)?;
        let properties = ConnectProperties::read(cursor)?;

        // [3.1.3] Payload
        let client_id = Py::<PyString>::read(cursor)?;
        let will = if (vh_flags & 0x04) == 1 {
            let properties = WillProperties::read(cursor)?;
            let topic = Py::<PyString>::read(cursor)?;
            let payload = Py::<PyBytes>::read(cursor)?;
            Some(Will {
                topic,
                payload: Some(payload),
                qos: QoS::new(vh_flags & 0x18)?,
                retain: (vh_flags & 0x20) == 1,
                properties,
            })
        } else {
            None
        };
        let username = if (vh_flags & 0x80) == 1 {
            Some(Py::<PyString>::read(cursor)?)
        } else {
            None
        };
        let password = if (vh_flags & 0x40) == 1 {
            Some(Py::<PyString>::read(cursor)?)
        } else {
            None
        };

        // Return Python object
        let packet = Self {
            client_id,
            username,
            password,
            clean_start,
            will,
            keep_alive,
            properties,
        };
        Py::new(py, packet)
    }
}

impl PartialEq for ConnectPacket {
    fn eq(&self, other: &Self) -> bool {
        self.client_id.equals(&other.client_id)
            && self.username.equals(&other.username)
            && self.password.equals(&other.password)
            && self.clean_start == other.clean_start
            && self.will == other.will
            && self.keep_alive == other.keep_alive
            && self.properties == other.properties
    }
}

#[pyclass(eq, get_all)]
pub struct ConnAckPacket {
    session_present: bool,
    reason_code: ConnAckReasonCode,
    properties: ConnAckProperties,
}

#[pymethods]
impl ConnAckPacket {
    #[new]
    #[pyo3(signature = (
        *,
        session_present=false,
        reason_code=ConnAckReasonCode::Success,
        properties=None,
    ))]
    pub fn new(
        py: Python,
        session_present: bool,
        reason_code: ConnAckReasonCode,
        properties: Option<ConnAckProperties>,
    ) -> PyResult<Self> {
        Ok(Self {
            session_present,
            reason_code,
            properties: properties.unwrap_or_default(),
        })
    }

    #[pyo3(signature = (buffer, /, *, index=0))]
    pub fn write(
        &self,
        py: Python,
        buffer: &Bound<'_, PyByteArray>,
        index: usize,
    ) -> PyResult<usize> {
        let remaining_length = 0u8.size() + self.reason_code.size() + self.properties.size();
        let var = VariableByteInteger(remaining_length as u32);
        let mut cursor = Cursor::new(buffer, index);
        cursor.assert(1 + var.size() + remaining_length)?;

        // [3.2.1] Fixed header
        let first_byte = (PacketType::ConnAck as u8) << 4;
        first_byte.write(&mut cursor);
        var.write(&mut cursor);

        // [3.2.2] Variable header
        let vh_flags = self.session_present as u8;
        vh_flags.write(&mut cursor);
        self.reason_code.write(&mut cursor);
        self.properties.write(&mut cursor);

        Ok(cursor.index - index)
    }
}

impl ConnAckPacket {
    fn read(
        py: Python,
        cursor: &mut Cursor,
        flags: u8,
        remaining_length: VariableByteInteger,
    ) -> PyResult<Py<Self>> {
        if flags != 0x00 {
            return Err(PyValueError::new_err("Malformed bytes"));
        }

        // [3.2.2] Variable header
        let vh_flags = u8::read(cursor)?;
        if (vh_flags & 0xfe) != 0 {
            return Err(PyValueError::new_err("Invalid bytes"));
        }
        let session_present = (vh_flags & 0x01) != 0;
        let reason_code = ConnAckReasonCode::read(cursor)?;
        let properties = ConnAckProperties::read(cursor)?;

        // Return Python object
        let packet = Self {
            session_present,
            reason_code,
            properties,
        };
        Py::new(py, packet)
    }
}

impl PartialEq for ConnAckPacket {
    fn eq(&self, other: &Self) -> bool {
        self.session_present == other.session_present
            && self.reason_code == other.reason_code
            && self.properties == other.properties
    }
}

#[pyclass(eq, get_all)]
pub struct PublishPacket {
    topic: Py<PyString>,
    payload: Option<Py<PyBytes>>,
    qos: QoS,
    retain: bool,
    packet_id: Option<u16>,
    duplicate: bool,
    properties: PublishProperties,
}

#[pymethods]
impl PublishPacket {
    #[new]
    #[pyo3(signature = (
        topic,
        *,
        payload=None,
        qos=QoS::AtMostOnce,
        retain=false,
        packet_id=None,
        duplicate=false,
        properties=None,
    ))]
    pub fn new(
        py: Python,
        topic: &Bound<'_, PyString>,
        payload: Option<&Bound<'_, PyBytes>>,
        qos: QoS,
        retain: bool,
        packet_id: Option<u16>,
        duplicate: bool,
        properties: Option<PublishProperties>,
    ) -> PyResult<Self> {
        if packet_id.is_some() && qos == QoS::AtMostOnce {
            return Err(PyValueError::new_err(
                "Packet ID must not be set for QoS.AT_MOST_ONCE",
            ));
        }
        if packet_id.is_none() && (qos == QoS::AtLeastOnce || qos == QoS::ExactlyOnce) {
            return Err(PyValueError::new_err(
                "Packet ID must be set for QoS.AT_LEAST_ONCE and QoS.EXACTLY_ONCE",
            ));
        }
        Ok(Self {
            topic: topic.clone().unbind(),
            qos,
            duplicate,
            retain,
            packet_id,
            properties: properties.unwrap_or_default(),
            payload: payload.map(|x| x.clone().unbind()),
        })
    }

    #[pyo3(signature = (buffer, /, *, index=0))]
    pub fn write(
        &self,
        py: Python,
        buffer: &Bound<'_, PyByteArray>,
        index: usize,
    ) -> PyResult<usize> {
        let payload = self.payload.as_ref().map(|x| x.bind(py).as_bytes());
        let remaining_length = self.topic.size()
            + self.packet_id.size()
            + self.properties.size()
            + payload.map_or(0, |x| x.len());
        let var = VariableByteInteger(remaining_length as u32);
        let mut cursor = Cursor::new(buffer, index);
        cursor.assert(1 + var.size() + remaining_length)?;

        // [3.3.1] Fixed header
        let first_byte = (PacketType::Publish as u8) << 4
            | (self.duplicate as u8) << 3
            | (self.qos as u8) << 1
            | self.retain as u8;
        first_byte.write(&mut cursor);
        var.write(&mut cursor);

        // [3.3.2] Variable header
        self.topic.write(&mut cursor);
        self.packet_id.write(&mut cursor);
        self.properties.write(&mut cursor);

        // [3.3.3] Payload
        if let Some(ref payload) = payload {
            let length = payload.len();
            cursor.buffer[cursor.index..cursor.index + length].copy_from_slice(payload);
            cursor.index += length;
        }

        Ok(cursor.index - index)
    }
}

impl PublishPacket {
    fn read(
        py: Python,
        cursor: &mut Cursor,
        flags: u8,
        remaining_length: VariableByteInteger,
    ) -> PyResult<Py<Self>> {
        let i0 = cursor.index;

        let retain = (flags & 0x01) == 1;
        let qos = QoS::new(flags & 0x06)?;
        let duplicate = (flags & 0x08) == 1;

        // [3.3.2] Variable header
        let topic = Py::<PyString>::read(cursor)?;
        let packet_id = if qos == QoS::AtLeastOnce || qos == QoS::ExactlyOnce {
            Some(u16::read(cursor)?)
        } else {
            None
        };
        let properties = PublishProperties::read(cursor)?;

        // [3.3.3] Payload
        let length = i0 + remaining_length.get() as usize - cursor.index;
        let payload =
            PyBytes::new(py, &cursor.buffer[cursor.index..cursor.index + length]).unbind();
        cursor.index += length;

        // Return Python object
        let packet = Self {
            topic,
            payload: Some(payload),
            qos,
            retain,
            packet_id,
            duplicate,
            properties,
        };
        Py::new(py, packet)
    }
}

impl PartialEq for PublishPacket {
    fn eq(&self, other: &Self) -> bool {
        self.topic.equals(&other.topic)
            && self.payload.equals(&other.payload)
            && self.qos == other.qos
            && self.retain == other.retain
            && self.packet_id == other.packet_id
            && self.duplicate == other.duplicate
            && self.properties == other.properties
    }
}

#[pyclass(eq, get_all)]
pub struct PubAckPacket {
    packet_id: u16,
    reason_code: PubAckReasonCode,
    properties: PubAckProperties,
}

#[pymethods]
impl PubAckPacket {
    #[new]
    #[pyo3(signature = (
        packet_id,
        *,
        reason_code=PubAckReasonCode::Success,
        properties=None,
    ))]
    pub fn new(
        py: Python,
        packet_id: u16,
        reason_code: PubAckReasonCode,
        properties: Option<PubAckProperties>,
    ) -> PyResult<Self> {
        Ok(Self {
            packet_id,
            reason_code,
            properties: properties.unwrap_or_default(),
        })
    }

    #[pyo3(signature = (buffer, /, *, index=0))]
    pub fn write(
        &self,
        py: Python,
        buffer: &Bound<'_, PyByteArray>,
        index: usize,
    ) -> PyResult<usize> {
        let remaining_length =
            self.packet_id.size() + self.reason_code.size() + self.properties.size();
        let var = VariableByteInteger(remaining_length as u32);
        let mut cursor = Cursor::new(buffer, index);
        cursor.assert(1 + var.size() + remaining_length)?;

        // [3.4.1] Fixed header
        let first_byte = (PacketType::PubAck as u8) << 4;
        first_byte.write(&mut cursor);
        var.write(&mut cursor);

        // [3.4.2] Variable header
        self.packet_id.write(&mut cursor);
        self.reason_code.write(&mut cursor);
        self.properties.write(&mut cursor);

        Ok(cursor.index - index)
    }
}

impl PubAckPacket {
    fn read(
        py: Python,
        cursor: &mut Cursor,
        flags: u8,
        remaining_length: VariableByteInteger,
    ) -> PyResult<Py<Self>> {
        if flags != 0x00 {
            return Err(PyValueError::new_err("Malformed bytes"));
        }

        // [3.4.2] Variable header
        let packet_id = u16::read(cursor)?;
        let reason_code = if remaining_length.get() > 2 {
            PubAckReasonCode::read(cursor)?
        } else {
            PubAckReasonCode::Success
        };
        let properties = if remaining_length.get() > 3 {
            Some(PubAckProperties::read(cursor)?)
        } else {
            None
        };

        // Return Python object
        let packet = Self {
            packet_id,
            reason_code,
            properties: properties.unwrap_or_default(),
        };
        Py::new(py, packet)
    }
}

impl PartialEq for PubAckPacket {
    fn eq(&self, other: &Self) -> bool {
        self.packet_id == other.packet_id
            && self.reason_code == other.reason_code
            && self.properties == other.properties
    }
}

#[pyclass(eq, get_all)]
pub struct DisconnectPacket {
    reason_code: DisconnectReasonCode,
    properties: DisconnectProperties,
}

#[pymethods]
impl DisconnectPacket {
    #[new]
    #[pyo3(signature = (
        *,
        reason_code=DisconnectReasonCode::NormalDisconnection,
        properties=None,
    ))]
    pub fn new(
        py: Python,
        reason_code: DisconnectReasonCode,
        properties: Option<DisconnectProperties>,
    ) -> PyResult<Self> {
        Ok(Self {
            reason_code,
            properties: properties.unwrap_or_default(),
        })
    }

    #[pyo3(signature = (buffer, /, *, index=0))]
    pub fn write(
        &self,
        py: Python,
        buffer: &Bound<'_, PyByteArray>,
        index: usize,
    ) -> PyResult<usize> {
        let remaining_length = self.reason_code.size() + self.properties.size();
        let var = VariableByteInteger(remaining_length as u32);
        let mut cursor = Cursor::new(buffer, index);
        cursor.assert(1 + var.size() + remaining_length)?;

        // [3.14.1] Fixed header
        let first_byte = (PacketType::Disconnect as u8) << 4;
        first_byte.write(&mut cursor);
        var.write(&mut cursor);

        // [3.14.2] Variable header
        self.reason_code.write(&mut cursor);
        self.properties.write(&mut cursor);

        Ok(cursor.index - index)
    }
}

impl DisconnectPacket {
    fn read(
        py: Python,
        cursor: &mut Cursor,
        flags: u8,
        remaining_length: VariableByteInteger,
    ) -> PyResult<Py<Self>> {
        if flags != 0x00 {
            return Err(PyValueError::new_err("Malformed bytes"));
        }

        // [3.14.2] Variable header
        let reason_code = DisconnectReasonCode::read(cursor)?;
        let properties = DisconnectProperties::read(cursor)?;

        // Return Python object
        let packet = Self {
            reason_code,
            properties,
        };
        Py::new(py, packet)
    }
}

impl PartialEq for DisconnectPacket {
    fn eq(&self, other: &Self) -> bool {
        self.reason_code == other.reason_code && self.properties == other.properties
    }
}

#[pyfunction]
#[pyo3(signature = (buffer, /, *, index=0))]
fn read(py: Python, buffer: &Bound<'_, PyByteArray>, index: usize) -> PyResult<(PyObject, usize)> {
    // Parse the fixed header
    let mut cursor = Cursor::new(buffer, index);
    let first_byte = u8::read(&mut cursor)?;
    let flags = first_byte & 0x0F;
    let remaining_length = VariableByteInteger::read(&mut cursor)?;
    // Call the read method of the corresponding packet for the remaining bytes
    match PacketType::new(first_byte >> 4)? {
        PacketType::Connect => {
            let packet = ConnectPacket::read(py, &mut cursor, flags, remaining_length)?;
            Ok((packet.into(), cursor.index))
        }
        PacketType::ConnAck => {
            let packet = ConnAckPacket::read(py, &mut cursor, flags, remaining_length)?;
            Ok((packet.into(), cursor.index))
        }
        PacketType::Publish => {
            let packet = PublishPacket::read(py, &mut cursor, flags, remaining_length)?;
            Ok((packet.into(), cursor.index))
        }
        PacketType::PubAck => {
            let packet = PubAckPacket::read(py, &mut cursor, flags, remaining_length)?;
            Ok((packet.into(), cursor.index))
        }
        PacketType::PubRec => Err(PyValueError::new_err("Not implemented")),
        PacketType::PubRel => Err(PyValueError::new_err("Not implemented")),
        PacketType::PubComp => Err(PyValueError::new_err("Not implemented")),
        PacketType::Subscribe => Err(PyValueError::new_err("Not implemented")),
        PacketType::SubAck => Err(PyValueError::new_err("Not implemented")),
        PacketType::Unsubscribe => Err(PyValueError::new_err("Not implemented")),
        PacketType::UnsubAck => Err(PyValueError::new_err("Not implemented")),
        PacketType::PingReq => Err(PyValueError::new_err("Not implemented")),
        PacketType::PingResp => Err(PyValueError::new_err("Not implemented")),
        PacketType::Disconnect => {
            let packet = DisconnectPacket::read(py, &mut cursor, flags, remaining_length)?;
            Ok((packet.into(), cursor.index))
        }
        PacketType::Auth => Err(PyValueError::new_err("Not implemented")),
    }
}

#[pymodule]
fn mqtt5(m: &Bound<'_, PyModule>) -> PyResult<()> {
    // Packets
    m.add_class::<ConnectPacket>()?;
    m.add_class::<ConnAckPacket>()?;
    m.add_class::<PublishPacket>()?;
    m.add_class::<PubAckPacket>()?;
    m.add_class::<DisconnectPacket>()?;
    // Properties
    m.add_class::<ConnectProperties>()?;
    m.add_class::<ConnAckProperties>()?;
    m.add_class::<PublishProperties>()?;
    m.add_class::<PubAckProperties>()?;
    m.add_class::<DisconnectProperties>()?;
    // Reason codes
    m.add_class::<ConnAckReasonCode>()?;
    m.add_class::<PubAckReasonCode>()?;
    m.add_class::<PubRecReasonCode>()?;
    m.add_class::<DisconnectReasonCode>()?;
    // Misc
    m.add_class::<QoS>()?;
    m.add_class::<WillProperties>()?;
    m.add_class::<Will>()?;
    // Functions
    m.add_function(wrap_pyfunction!(read, m)?)?;
    Ok(())
}
