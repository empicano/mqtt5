use crate::io::{Cursor, Readable, VariableByteInteger, Writable};
use crate::reason_codes::*;
use crate::types::{PacketType, PropertyType, PyEq, QoS, RetainHandling};
use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;
use pyo3::types::{PyByteArray, PyBytes, PyList, PyString};
use pyo3::PyResult;

const PROTOCOL_NAME: &str = "MQTT";
const PROTOCOL_VERSION: u8 = 5;

#[pyclass(frozen, eq, get_all, module = "mqtt5")]
pub struct Will {
    pub topic: Py<PyString>,
    pub payload: Option<Py<PyBytes>>,
    pub qos: QoS,
    pub retain: bool,
    pub payload_format_indicator: u8,
    pub message_expiry_interval: Option<u32>,
    pub content_type: Option<Py<PyString>>,
    pub response_topic: Option<Py<PyString>>,
    pub correlation_data: Option<Py<PyBytes>>,
    pub will_delay_interval: u32,
}

#[pymethods]
impl Will {
    #[new]
    #[pyo3(signature = (
        topic,
        *,
        payload=None,
        qos=QoS::AtMostOnce,
        retain=false,
        payload_format_indicator=0,
        message_expiry_interval=None,
        content_type=None,
        response_topic=None,
        correlation_data=None,
        will_delay_interval=0,
    ))]
    pub fn new(
        topic: &Bound<'_, PyString>,
        payload: Option<&Bound<'_, PyBytes>>,
        qos: QoS,
        retain: bool,
        payload_format_indicator: u8,
        message_expiry_interval: Option<u32>,
        content_type: Option<Py<PyString>>,
        response_topic: Option<Py<PyString>>,
        correlation_data: Option<Py<PyBytes>>,
        will_delay_interval: u32,
    ) -> Self {
        Self {
            topic: topic.clone().unbind(),
            payload: payload.map(|x| x.clone().unbind()),
            qos,
            retain,
            payload_format_indicator,
            message_expiry_interval,
            content_type,
            response_topic,
            correlation_data,
            will_delay_interval,
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
            payload_format_indicator: self.payload_format_indicator,
            message_expiry_interval: self.message_expiry_interval,
            content_type: self.content_type.as_ref().map(|x| x.clone_ref(py)),
            response_topic: self.response_topic.as_ref().map(|x| x.clone_ref(py)),
            correlation_data: self.correlation_data.as_ref().map(|x| x.clone_ref(py)),
            will_delay_interval: self.will_delay_interval,
        })
    }
}

impl PartialEq for Will {
    fn eq(&self, other: &Self) -> bool {
        self.topic.py_eq(&other.topic)
            && self.payload.py_eq(&other.payload)
            && self.qos == other.qos
            && self.retain == other.retain
            && self.payload_format_indicator == other.payload_format_indicator
            && self.message_expiry_interval == other.message_expiry_interval
            && self.content_type.py_eq(&other.content_type)
            && self.response_topic.py_eq(&other.response_topic)
            && self.correlation_data.py_eq(&other.correlation_data)
            && self.will_delay_interval == other.will_delay_interval
    }
}

#[pyclass(frozen, eq, get_all, module = "mqtt5")]
pub struct Subscription {
    pub pattern: Py<PyString>,
    pub maximum_qos: QoS,
    pub no_local: bool,
    pub retain_as_published: bool,
    pub retain_handling: RetainHandling,
}

#[pymethods]
impl Subscription {
    #[new]
    #[pyo3(signature = (
        pattern,
        *,
        maximum_qos=QoS::ExactlyOnce,
        no_local=false,
        retain_as_published=true,
        retain_handling=RetainHandling::SendAlways,
    ))]
    pub fn new(
        pattern: &Bound<'_, PyString>,
        maximum_qos: QoS,
        no_local: bool,
        retain_as_published: bool,
        retain_handling: RetainHandling,
    ) -> Self {
        Self {
            pattern: pattern.clone().unbind(),
            no_local,
            maximum_qos,
            retain_as_published,
            retain_handling,
        }
    }
}

impl PartialEq for Subscription {
    fn eq(&self, other: &Self) -> bool {
        self.pattern.py_eq(&other.pattern)
            && self.maximum_qos == other.maximum_qos
            && self.no_local == other.no_local
            && self.retain_as_published == other.retain_as_published
            && self.retain_handling == other.retain_handling
    }
}

#[pyclass(frozen, eq, get_all, module = "mqtt5")]
pub struct ConnectPacket {
    pub client_id: Py<PyString>,
    pub username: Option<Py<PyString>>,
    pub password: Option<Py<PyString>>,
    pub clean_start: bool,
    pub will: Option<Will>,
    pub keep_alive: u16,
    pub session_expiry_interval: u32,
    pub authentication_method: Option<Py<PyString>>,
    pub authentication_data: Option<Py<PyBytes>>,
    pub request_problem_information: bool,
    pub request_response_information: bool,
    pub receive_maximum: u16,
    pub topic_alias_maximum: u16,
    pub maximum_packet_size: Option<u32>,
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
        session_expiry_interval=0,
        authentication_method=None,
        authentication_data=None,
        request_problem_information=true,
        request_response_information=false,
        receive_maximum=65535,
        topic_alias_maximum=0,
        maximum_packet_size=None,
    ))]
    pub fn new(
        client_id: &Bound<'_, PyString>,
        username: Option<&Bound<'_, PyString>>,
        password: Option<&Bound<'_, PyString>>,
        clean_start: bool,
        will: Option<Will>,
        keep_alive: u16,
        session_expiry_interval: u32,
        authentication_method: Option<Py<PyString>>,
        authentication_data: Option<Py<PyBytes>>,
        request_problem_information: bool,
        request_response_information: bool,
        receive_maximum: u16,
        topic_alias_maximum: u16,
        maximum_packet_size: Option<u32>,
    ) -> PyResult<Self> {
        Ok(Self {
            client_id: client_id.clone().unbind(),
            username: username.map(|x| x.clone().unbind()),
            password: password.map(|x| x.clone().unbind()),
            clean_start,
            will,
            keep_alive,
            session_expiry_interval,
            authentication_method,
            authentication_data,
            request_problem_information,
            request_response_information,
            receive_maximum,
            topic_alias_maximum,
            maximum_packet_size,
        })
    }

    #[pyo3(signature = (buffer, /, *, index=0))]
    fn write(&self, buffer: &Bound<'_, PyByteArray>, index: usize) -> PyResult<usize> {
        let mut properties_nbytes = 0;
        if self.session_expiry_interval != 0 {
            properties_nbytes += 0u8.nbytes() + self.session_expiry_interval.nbytes();
        }
        if let Some(ref authentication_method) = self.authentication_method {
            properties_nbytes += 0u8.nbytes() + authentication_method.nbytes();
        }
        if let Some(ref authentication_data) = self.authentication_data {
            properties_nbytes += 0u8.nbytes() + authentication_data.nbytes();
        }
        if !self.request_problem_information {
            properties_nbytes += 0u8.nbytes() + self.request_problem_information.nbytes();
        }
        if self.request_response_information {
            properties_nbytes += 0u8.nbytes() + self.request_response_information.nbytes();
        }
        if self.receive_maximum != 65535 {
            properties_nbytes += 0u8.nbytes() + self.receive_maximum.nbytes();
        }
        if self.topic_alias_maximum != 0 {
            properties_nbytes += 0u8.nbytes() + self.topic_alias_maximum.nbytes();
        }
        if let Some(ref maximum_packet_size) = self.maximum_packet_size {
            properties_nbytes += 0u8.nbytes() + maximum_packet_size.nbytes();
        }
        let properties_remaining_length = VariableByteInteger::new(properties_nbytes as u32);
        let mut will_properties_nbytes = 0;
        if let Some(ref will) = self.will {
            if will.payload_format_indicator != 0 {
                will_properties_nbytes += 0u8.nbytes() + will.payload_format_indicator.nbytes();
            }
            if let Some(message_expiry_interval) = will.message_expiry_interval {
                will_properties_nbytes += 0u8.nbytes() + message_expiry_interval.nbytes();
            }
            if let Some(ref content_type) = will.content_type {
                will_properties_nbytes += 0u8.nbytes() + content_type.nbytes();
            }
            if let Some(ref response_topic) = will.response_topic {
                will_properties_nbytes += 0u8.nbytes() + response_topic.nbytes();
            }
            if let Some(ref correlation_data) = will.correlation_data {
                will_properties_nbytes += 0u8.nbytes() + correlation_data.nbytes();
            }
            if will.will_delay_interval != 0 {
                will_properties_nbytes += 0u8.nbytes() + will.will_delay_interval.nbytes();
            }
        }
        let will_properties_remaining_length =
            VariableByteInteger::new(will_properties_nbytes as u32);
        let nbytes = PROTOCOL_NAME.nbytes()
            + PROTOCOL_VERSION.nbytes()
            + 0u8.nbytes()
            + self.keep_alive.nbytes()
            + properties_remaining_length.nbytes()
            + properties_nbytes
            + self.client_id.nbytes()
            + self.will.as_ref().map_or(0, |x| {
                will_properties_remaining_length.nbytes()
                    + will_properties_nbytes
                    + x.topic.nbytes()
                    + x.payload.as_ref().map_or(0u16.nbytes(), |x| x.nbytes())
            })
            + self.username.nbytes()
            + self.password.nbytes();
        let remaining_length = VariableByteInteger::new(nbytes as u32);
        let mut cursor = Cursor::new(buffer, index);
        cursor.require(1 + remaining_length.nbytes() + nbytes)?;

        // [3.1.1] Fixed header
        let first_byte = (PacketType::Connect as u8) << 4;
        first_byte.write(&mut cursor);
        remaining_length.write(&mut cursor);

        // [3.1.2] Variable header
        PROTOCOL_NAME.write(&mut cursor);
        PROTOCOL_VERSION.write(&mut cursor);
        let mut packet_flags = (self.clean_start as u8) << 1;
        if let Some(ref will) = self.will {
            packet_flags |= 0x04;
            packet_flags |= (will.qos as u8) << 3;
            packet_flags |= (will.retain as u8) << 5;
        }
        if self.password.is_some() {
            packet_flags |= 0x40;
        }
        if self.username.is_some() {
            packet_flags |= 0x80;
        }
        packet_flags.write(&mut cursor);
        self.keep_alive.write(&mut cursor);

        // [3.1.2.11] Properties
        properties_remaining_length.write(&mut cursor);
        if self.session_expiry_interval != 0 {
            (PropertyType::SessionExpiryInterval as u8).write(&mut cursor);
            self.session_expiry_interval.write(&mut cursor);
        }
        if let Some(ref authentication_method) = self.authentication_method {
            (PropertyType::AuthenticationMethod as u8).write(&mut cursor);
            authentication_method.write(&mut cursor);
        }
        if let Some(ref authentication_data) = self.authentication_data {
            (PropertyType::AuthenticationData as u8).write(&mut cursor);
            authentication_data.write(&mut cursor);
        }
        if !self.request_problem_information {
            (PropertyType::RequestProblemInformation as u8).write(&mut cursor);
            self.request_problem_information.write(&mut cursor);
        }
        if self.request_response_information {
            (PropertyType::RequestResponseInformation as u8).write(&mut cursor);
            self.request_response_information.write(&mut cursor);
        }
        if self.receive_maximum != 65535 {
            (PropertyType::ReceiveMaximum as u8).write(&mut cursor);
            self.receive_maximum.write(&mut cursor);
        }
        if self.topic_alias_maximum != 0 {
            (PropertyType::TopicAliasMaximum as u8).write(&mut cursor);
            self.topic_alias_maximum.write(&mut cursor);
        }
        if let Some(ref maximum_packet_size) = self.maximum_packet_size {
            (PropertyType::MaximumPacketSize as u8).write(&mut cursor);
            maximum_packet_size.write(&mut cursor);
        }

        // [3.1.3] Payload
        self.client_id.write(&mut cursor);
        if let Some(ref will) = self.will {
            will_properties_remaining_length.write(&mut cursor);
            if will.payload_format_indicator != 0 {
                (PropertyType::PayloadFormatIndicator as u8).write(&mut cursor);
                will.payload_format_indicator.write(&mut cursor);
            }
            if let Some(message_expiry_interval) = will.message_expiry_interval {
                (PropertyType::MessageExpiryInterval as u8).write(&mut cursor);
                message_expiry_interval.write(&mut cursor);
            }
            if let Some(ref content_type) = will.content_type {
                (PropertyType::ContentType as u8).write(&mut cursor);
                content_type.write(&mut cursor);
            }
            if let Some(ref response_topic) = will.response_topic {
                (PropertyType::ResponseTopic as u8).write(&mut cursor);
                response_topic.write(&mut cursor);
            }
            if let Some(ref correlation_data) = will.correlation_data {
                (PropertyType::CorrelationData as u8).write(&mut cursor);
                correlation_data.write(&mut cursor);
            }
            if will.will_delay_interval != 0 {
                (PropertyType::WillDelayInterval as u8).write(&mut cursor);
                will.will_delay_interval.write(&mut cursor);
            }
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
    pub fn read(
        py: Python,
        cursor: &mut Cursor,
        flags: u8,
        _remaining_length: VariableByteInteger,
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
        let packet_flags = u8::read(cursor)?;
        let clean_start = (packet_flags & 0x02) != 0;
        let keep_alive = u16::read(cursor)?;

        // [3.1.2.11] Properties
        let mut session_expiry_interval = 0;
        let mut authentication_method = None;
        let mut authentication_data = None;
        let mut request_problem_information = true;
        let mut request_response_information = false;
        let mut receive_maximum = 65535;
        let mut topic_alias_maximum = 0;
        let mut maximum_packet_size = None;
        let properties_remaining_length = VariableByteInteger::read(cursor)?.value() as usize;
        let properties_start_index = cursor.index;
        while cursor.index - properties_start_index < properties_remaining_length {
            match PropertyType::new(u8::read(cursor)?)? {
                PropertyType::SessionExpiryInterval => {
                    session_expiry_interval = u32::read(cursor)?;
                }
                PropertyType::AuthenticationMethod => {
                    authentication_method = Some(Py::<PyString>::read(cursor)?);
                }
                PropertyType::AuthenticationData => {
                    authentication_data = Some(Py::<PyBytes>::read(cursor)?);
                }
                PropertyType::RequestProblemInformation => {
                    request_problem_information = bool::read(cursor)?;
                }
                PropertyType::RequestResponseInformation => {
                    request_response_information = bool::read(cursor)?;
                }
                PropertyType::ReceiveMaximum => {
                    receive_maximum = u16::read(cursor)?;
                }
                PropertyType::TopicAliasMaximum => {
                    topic_alias_maximum = u16::read(cursor)?;
                }
                PropertyType::MaximumPacketSize => {
                    maximum_packet_size = Some(u32::read(cursor)?);
                }
                other => {
                    return Err(PyValueError::new_err(format!(
                        "Invalid property type for ConnectPacket: {:?}",
                        other
                    )));
                }
            }
        }

        // [3.1.3] Payload
        let client_id = Py::<PyString>::read(cursor)?;
        let will = if (packet_flags & 0x04) != 0 {
            let mut payload_format_indicator = 0;
            let mut message_expiry_interval = None;
            let mut content_type = None;
            let mut response_topic = None;
            let mut correlation_data = None;
            let mut will_delay_interval = 0;
            let will_properties_remaining_length =
                VariableByteInteger::read(cursor)?.value() as usize;
            let will_properties_start_index = cursor.index;
            while cursor.index - will_properties_start_index < will_properties_remaining_length {
                match PropertyType::new(u8::read(cursor)?)? {
                    PropertyType::PayloadFormatIndicator => {
                        payload_format_indicator = u8::read(cursor)?;
                    }
                    PropertyType::MessageExpiryInterval => {
                        message_expiry_interval = Some(u32::read(cursor)?);
                    }
                    PropertyType::ContentType => {
                        content_type = Some(Py::<PyString>::read(cursor)?);
                    }
                    PropertyType::ResponseTopic => {
                        response_topic = Some(Py::<PyString>::read(cursor)?);
                    }
                    PropertyType::CorrelationData => {
                        correlation_data = Some(Py::<PyBytes>::read(cursor)?);
                    }
                    PropertyType::WillDelayInterval => {
                        will_delay_interval = u32::read(cursor)?;
                    }
                    other => {
                        return Err(PyValueError::new_err(format!(
                            "Invalid property type for Will: {:?}",
                            other
                        )));
                    }
                }
            }
            let topic = Py::<PyString>::read(cursor)?;
            let payload = Py::<PyBytes>::read(cursor)?;
            Some(Will {
                topic,
                payload: Some(payload),
                qos: QoS::new((packet_flags >> 3) & 0x03)?,
                retain: (packet_flags & 0x20) != 0,
                payload_format_indicator,
                message_expiry_interval,
                content_type,
                response_topic,
                correlation_data,
                will_delay_interval,
            })
        } else {
            None
        };
        let username = if (packet_flags & 0x80) != 0 {
            Some(Py::<PyString>::read(cursor)?)
        } else {
            None
        };
        let password = if (packet_flags & 0x40) != 0 {
            Some(Py::<PyString>::read(cursor)?)
        } else {
            None
        };

        // Return the Python object
        let packet = Self {
            client_id,
            username,
            password,
            clean_start,
            will,
            keep_alive,
            session_expiry_interval,
            authentication_method,
            authentication_data,
            request_problem_information,
            request_response_information,
            receive_maximum,
            topic_alias_maximum,
            maximum_packet_size,
        };
        Py::new(py, packet)
    }
}

impl PartialEq for ConnectPacket {
    fn eq(&self, other: &Self) -> bool {
        self.client_id.py_eq(&other.client_id)
            && self.username.py_eq(&other.username)
            && self.password.py_eq(&other.password)
            && self.clean_start == other.clean_start
            && self.will == other.will
            && self.keep_alive == other.keep_alive
            && self.session_expiry_interval == other.session_expiry_interval
            && self
                .authentication_method
                .py_eq(&other.authentication_method)
            && self.authentication_data.py_eq(&other.authentication_data)
            && self.request_problem_information == other.request_problem_information
            && self.request_response_information == other.request_response_information
            && self.receive_maximum == other.receive_maximum
            && self.topic_alias_maximum == other.topic_alias_maximum
            && self.maximum_packet_size == other.maximum_packet_size
    }
}

#[pyclass(eq, get_all, module = "mqtt5")]
pub struct ConnAckPacket {
    pub session_present: bool,
    pub reason_code: ConnAckReasonCode,
    pub session_expiry_interval: Option<u32>,
    pub assigned_client_id: Option<Py<PyString>>,
    pub server_keep_alive: Option<u16>,
    pub authentication_method: Option<Py<PyString>>,
    pub authentication_data: Option<Py<PyBytes>>,
    pub response_information: Option<Py<PyString>>,
    pub server_reference: Option<Py<PyString>>,
    pub reason_string: Option<Py<PyString>>,
    pub receive_maximum: u16,
    pub topic_alias_maximum: u16,
    pub maximum_qos: QoS,
    pub retain_available: bool,
    pub maximum_packet_size: Option<u32>,
    pub wildcard_subscription_available: bool,
    pub subscription_id_available: bool,
    pub shared_subscription_available: bool,
}

#[pymethods]
impl ConnAckPacket {
    #[new]
    #[pyo3(signature = (
        *,
        session_present=false,
        reason_code=ConnAckReasonCode::Success,
        session_expiry_interval=None,
        assigned_client_id=None,
        server_keep_alive=None,
        authentication_method=None,
        authentication_data=None,
        response_information=None,
        server_reference=None,
        reason_string=None,
        receive_maximum=65535,
        topic_alias_maximum=0,
        maximum_qos=QoS::ExactlyOnce,
        retain_available=true,
        maximum_packet_size=None,
        wildcard_subscription_available=true,
        subscription_id_available=true,
        shared_subscription_available=true,
    ))]
    pub fn new(
        session_present: bool,
        reason_code: ConnAckReasonCode,
        session_expiry_interval: Option<u32>,
        assigned_client_id: Option<Py<PyString>>,
        server_keep_alive: Option<u16>,
        authentication_method: Option<Py<PyString>>,
        authentication_data: Option<Py<PyBytes>>,
        response_information: Option<Py<PyString>>,
        server_reference: Option<Py<PyString>>,
        reason_string: Option<Py<PyString>>,
        receive_maximum: u16,
        topic_alias_maximum: u16,
        maximum_qos: QoS,
        retain_available: bool,
        maximum_packet_size: Option<u32>,
        wildcard_subscription_available: bool,
        subscription_id_available: bool,
        shared_subscription_available: bool,
    ) -> PyResult<Self> {
        Ok(Self {
            session_present,
            reason_code,
            session_expiry_interval,
            assigned_client_id,
            server_keep_alive,
            authentication_method,
            authentication_data,
            response_information,
            server_reference,
            reason_string,
            receive_maximum,
            topic_alias_maximum,
            maximum_qos,
            retain_available,
            maximum_packet_size,
            wildcard_subscription_available,
            subscription_id_available,
            shared_subscription_available,
        })
    }

    #[pyo3(signature = (buffer, /, *, index=0))]
    pub fn write(&self, buffer: &Bound<'_, PyByteArray>, index: usize) -> PyResult<usize> {
        let mut properties_nbytes = 0;
        if let Some(session_expiry_interval) = self.session_expiry_interval {
            properties_nbytes += 0u8.nbytes() + session_expiry_interval.nbytes();
        }
        if let Some(ref assigned_client_id) = self.assigned_client_id {
            properties_nbytes += 0u8.nbytes() + assigned_client_id.nbytes();
        }
        if let Some(server_keep_alive) = self.server_keep_alive {
            properties_nbytes += 0u8.nbytes() + server_keep_alive.nbytes();
        }
        if let Some(ref authentication_method) = self.authentication_method {
            properties_nbytes += 0u8.nbytes() + authentication_method.nbytes();
        }
        if let Some(ref authentication_data) = self.authentication_data {
            properties_nbytes += 0u8.nbytes() + authentication_data.nbytes();
        }
        if let Some(ref response_information) = self.response_information {
            properties_nbytes += 0u8.nbytes() + response_information.nbytes();
        }
        if let Some(ref server_reference) = self.server_reference {
            properties_nbytes += 0u8.nbytes() + server_reference.nbytes();
        }
        if let Some(ref reason_string) = self.reason_string {
            properties_nbytes += 0u8.nbytes() + reason_string.nbytes();
        }
        if self.receive_maximum != 65535 {
            properties_nbytes += 0u8.nbytes() + self.receive_maximum.nbytes();
        }
        if self.topic_alias_maximum != 0 {
            properties_nbytes += 0u8.nbytes() + self.topic_alias_maximum.nbytes();
        }
        if self.maximum_qos != QoS::ExactlyOnce {
            properties_nbytes += 0u8.nbytes() + 0u8.nbytes();
        }
        if !self.retain_available {
            properties_nbytes += 0u8.nbytes() + self.retain_available.nbytes();
        }
        if let Some(ref maximum_packet_size) = self.maximum_packet_size {
            properties_nbytes += 0u8.nbytes() + maximum_packet_size.nbytes();
        }
        if !self.wildcard_subscription_available {
            properties_nbytes += 0u8.nbytes() + self.wildcard_subscription_available.nbytes();
        }
        if !self.subscription_id_available {
            properties_nbytes += 0u8.nbytes() + self.subscription_id_available.nbytes();
        }
        if !self.shared_subscription_available {
            properties_nbytes += 0u8.nbytes() + self.shared_subscription_available.nbytes();
        }
        let properties_remaining_length = VariableByteInteger::new(properties_nbytes as u32);

        let nbytes = 0u8.nbytes()
            + self.reason_code.nbytes()
            + properties_remaining_length.nbytes()
            + properties_nbytes;
        let remaining_length = VariableByteInteger::new(nbytes as u32);
        let mut cursor = Cursor::new(buffer, index);
        cursor.require(1 + remaining_length.nbytes() + nbytes)?;

        // [3.2.1] Fixed header
        let first_byte = (PacketType::ConnAck as u8) << 4;
        first_byte.write(&mut cursor);
        remaining_length.write(&mut cursor);

        // [3.2.2] Variable header
        let packet_flags = self.session_present as u8;
        packet_flags.write(&mut cursor);
        self.reason_code.write(&mut cursor);

        // [3.2.2.3] Properties
        properties_remaining_length.write(&mut cursor);
        if let Some(session_expiry_interval) = self.session_expiry_interval {
            (PropertyType::SessionExpiryInterval as u8).write(&mut cursor);
            session_expiry_interval.write(&mut cursor);
        }
        if let Some(ref assigned_client_id) = self.assigned_client_id {
            (PropertyType::AssignedClientId as u8).write(&mut cursor);
            assigned_client_id.write(&mut cursor);
        }
        if let Some(server_keep_alive) = self.server_keep_alive {
            (PropertyType::ServerKeepAlive as u8).write(&mut cursor);
            server_keep_alive.write(&mut cursor);
        }
        if let Some(ref authentication_method) = self.authentication_method {
            (PropertyType::AuthenticationMethod as u8).write(&mut cursor);
            authentication_method.write(&mut cursor);
        }
        if let Some(ref authentication_data) = self.authentication_data {
            (PropertyType::AuthenticationData as u8).write(&mut cursor);
            authentication_data.write(&mut cursor);
        }
        if let Some(ref response_information) = self.response_information {
            (PropertyType::ResponseInformation as u8).write(&mut cursor);
            response_information.write(&mut cursor);
        }
        if let Some(ref server_reference) = self.server_reference {
            (PropertyType::ServerReference as u8).write(&mut cursor);
            server_reference.write(&mut cursor);
        }
        if let Some(ref reason_string) = self.reason_string {
            (PropertyType::ReasonString as u8).write(&mut cursor);
            reason_string.write(&mut cursor);
        }
        if self.receive_maximum != 65535 {
            (PropertyType::ReceiveMaximum as u8).write(&mut cursor);
            self.receive_maximum.write(&mut cursor);
        }
        if self.topic_alias_maximum != 0 {
            (PropertyType::TopicAliasMaximum as u8).write(&mut cursor);
            self.topic_alias_maximum.write(&mut cursor);
        }
        if self.maximum_qos != QoS::ExactlyOnce {
            (PropertyType::MaximumQoS as u8).write(&mut cursor);
            (self.maximum_qos as u8).write(&mut cursor);
        }
        if !self.retain_available {
            (PropertyType::RetainAvailable as u8).write(&mut cursor);
            self.retain_available.write(&mut cursor);
        }
        if let Some(maximum_packet_size) = self.maximum_packet_size {
            (PropertyType::MaximumPacketSize as u8).write(&mut cursor);
            maximum_packet_size.write(&mut cursor);
        }
        if !self.wildcard_subscription_available {
            (PropertyType::WildcardSubscriptionAvailable as u8).write(&mut cursor);
            self.wildcard_subscription_available.write(&mut cursor);
        }
        if !self.subscription_id_available {
            (PropertyType::SubscriptionIdAvailable as u8).write(&mut cursor);
            self.subscription_id_available.write(&mut cursor);
        }
        if !self.shared_subscription_available {
            (PropertyType::SharedSubscriptionAvailable as u8).write(&mut cursor);
            self.shared_subscription_available.write(&mut cursor);
        }

        Ok(cursor.index - index)
    }
}

impl ConnAckPacket {
    pub fn read(
        py: Python,
        cursor: &mut Cursor,
        flags: u8,
        _remaining_length: VariableByteInteger,
    ) -> PyResult<Py<Self>> {
        if flags != 0x00 {
            return Err(PyValueError::new_err("Malformed bytes"));
        }

        // [3.2.2] Variable header
        let packet_flags = u8::read(cursor)?;
        if (packet_flags & 0xfe) != 0 {
            return Err(PyValueError::new_err("Malformed bytes"));
        }
        let session_present = (packet_flags & 0x01) != 0;
        let reason_code = ConnAckReasonCode::read(cursor)?;

        // [3.2.2.3] Properties
        let mut session_expiry_interval = None;
        let mut assigned_client_id = None;
        let mut server_keep_alive = None;
        let mut authentication_method = None;
        let mut authentication_data = None;
        let mut response_information = None;
        let mut server_reference = None;
        let mut reason_string = None;
        let mut receive_maximum = 65535;
        let mut topic_alias_maximum = 0;
        let mut maximum_qos = QoS::ExactlyOnce;
        let mut retain_available = true;
        let mut maximum_packet_size = None;
        let mut wildcard_subscription_available = true;
        let mut subscription_id_available = true;
        let mut shared_subscription_available = true;
        let properties_remaining_length = VariableByteInteger::read(cursor)?.value() as usize;
        let properties_start_index = cursor.index;
        while cursor.index - properties_start_index < properties_remaining_length {
            match PropertyType::new(u8::read(cursor)?)? {
                PropertyType::SessionExpiryInterval => {
                    session_expiry_interval = Some(u32::read(cursor)?);
                }
                PropertyType::AssignedClientId => {
                    assigned_client_id = Some(Py::<PyString>::read(cursor)?);
                }
                PropertyType::ServerKeepAlive => {
                    server_keep_alive = Some(u16::read(cursor)?);
                }
                PropertyType::AuthenticationMethod => {
                    authentication_method = Some(Py::<PyString>::read(cursor)?);
                }
                PropertyType::AuthenticationData => {
                    authentication_data = Some(Py::<PyBytes>::read(cursor)?);
                }
                PropertyType::ResponseInformation => {
                    response_information = Some(Py::<PyString>::read(cursor)?);
                }
                PropertyType::ServerReference => {
                    server_reference = Some(Py::<PyString>::read(cursor)?);
                }
                PropertyType::ReasonString => {
                    reason_string = Some(Py::<PyString>::read(cursor)?);
                }
                PropertyType::ReceiveMaximum => {
                    receive_maximum = u16::read(cursor)?;
                }
                PropertyType::TopicAliasMaximum => {
                    topic_alias_maximum = u16::read(cursor)?;
                }
                PropertyType::MaximumQoS => {
                    maximum_qos = QoS::new(u8::read(cursor)?)?;
                }
                PropertyType::RetainAvailable => {
                    retain_available = bool::read(cursor)?;
                }
                PropertyType::MaximumPacketSize => {
                    maximum_packet_size = Some(u32::read(cursor)?);
                }
                PropertyType::WildcardSubscriptionAvailable => {
                    wildcard_subscription_available = bool::read(cursor)?;
                }
                PropertyType::SubscriptionIdAvailable => {
                    subscription_id_available = bool::read(cursor)?;
                }
                PropertyType::SharedSubscriptionAvailable => {
                    shared_subscription_available = bool::read(cursor)?;
                }
                other => {
                    return Err(PyValueError::new_err(format!(
                        "Invalid property type for ConnAckPacket: {:?}",
                        other
                    )));
                }
            }
        }

        // Return the Python object
        let packet = Self {
            session_present,
            reason_code,
            session_expiry_interval,
            assigned_client_id,
            server_keep_alive,
            authentication_method,
            authentication_data,
            response_information,
            server_reference,
            reason_string,
            receive_maximum,
            topic_alias_maximum,
            maximum_qos,
            retain_available,
            maximum_packet_size,
            wildcard_subscription_available,
            subscription_id_available,
            shared_subscription_available,
        };
        Py::new(py, packet)
    }
}

impl PartialEq for ConnAckPacket {
    fn eq(&self, other: &Self) -> bool {
        self.session_present == other.session_present
            && self.reason_code == other.reason_code
            && self.session_expiry_interval == other.session_expiry_interval
            && self.assigned_client_id.py_eq(&other.assigned_client_id)
            && self.server_keep_alive == other.server_keep_alive
            && self
                .authentication_method
                .py_eq(&other.authentication_method)
            && self.response_information.py_eq(&other.response_information)
            && self.server_reference.py_eq(&other.server_reference)
            && self.reason_string.py_eq(&other.reason_string)
            && self.topic_alias_maximum == other.topic_alias_maximum
            && self.maximum_qos == other.maximum_qos
            && self.retain_available == other.retain_available
            && self.maximum_packet_size == other.maximum_packet_size
            && self.wildcard_subscription_available == other.wildcard_subscription_available
            && self.subscription_id_available == other.subscription_id_available
            && self.shared_subscription_available == other.shared_subscription_available
    }
}

/*
# Commit: d0966a5
mqtt5: Read Connect: Mean +- std dev: 101 ns +- 2 ns
mqtt5: Write Connect: Mean +- std dev: 126 ns +- 1 ns
mqtt5: Read Connect(will): Mean +- std dev: 217 ns +- 13 ns
mqtt5: Write Connect(will): Mean +- std dev: 639 ns +- 6 ns
mqtt5: Read Connect(full): Mean +- std dev: 312 ns +- 17 ns
mqtt5: Write Connect(full): Mean +- std dev: 1.07 us +- 0.01 us
mqtt5: Read ConnAck: Mean +- std dev: 65.8 ns +- 3.5 ns
mqtt5: Write ConnAck: Mean +- std dev: 75.4 ns +- 1.3 ns
mqtt5: Read ConnAck(full): Mean +- std dev: 233 ns +- 14 ns
mqtt5: Write ConnAck(full): Mean +- std dev: 726 ns +- 7 ns
mqtt5: Read Publish(qos0): Mean +- std dev: 101 ns +- 6 ns
mqtt5: Write Publish(qos0): Mean +- std dev: 155 ns +- 1 ns
mqtt5: Read Publish(qos1): Mean +- std dev: 101 ns +- 2 ns
mqtt5: Write Publish(qos1): Mean +- std dev: 203 ns +- 2 ns
mqtt5: Read PubAck: Mean +- std dev: 57.5 ns +- 0.4 ns
mqtt5: Write PubAck: Mean +- std dev: 97.7 ns +- 0.7 ns
mqtt5: Read PubAck(full): Mean +- std dev: 81.1 ns +- 0.8 ns
mqtt5: Write PubAck(full): Mean +- std dev: 250 ns +- 14 ns
mqtt5: Read Subscribe: Mean +- std dev: 115 ns +- 2 ns
mqtt5: Write Subscribe: Mean +- std dev: 288 ns +- 18 ns
mqtt5: Read SubAck: Mean +- std dev: 99.8 ns +- 2.8 ns
mqtt5: Write SubAck: Mean +- std dev: 226 ns +- 93 ns
mqtt5: Read PingReq: Mean +- std dev: 54.1 ns +- 1.1 ns
mqtt5: Write PingReq: Mean +- std dev: 59.9 ns +- 2.9 ns
mqtt5: Read PingResp: Mean +- std dev: 54.5 ns +- 1.3 ns
mqtt5: Write PingResp: Mean +- std dev: 58.9 ns +- 1.0 ns
mqtt5: Read Disconnect: Mean +- std dev: 59.8 ns +- 0.9 ns
mqtt5: Write Disconnect: Mean +- std dev: 66.4 ns +- 0.5 ns
mqtt5: Read Disconnect(full): Mean +- std dev: 106 ns +- 6 ns
mqtt5: Write Disconnect(full): Mean +- std dev: 289 ns +- 3 ns

# Commit: e43085b
mqtt5: Read Connect: Mean +- std dev: 111 ns +- 3 ns
mqtt5: Write Connect: Mean +- std dev: 135 ns +- 2 ns
mqtt5: Read Connect(will): Mean +- std dev: 242 ns +- 3 ns
mqtt5: Write Connect(will): Mean +- std dev: 538 ns +- 6 ns
mqtt5: Read Connect(full): Mean +- std dev: 362 ns +- 6 ns
mqtt5: Write Connect(full): Mean +- std dev: 873 ns +- 10 ns
mqtt5: Read ConnAck: Mean +- std dev: 66.5 ns +- 1.6 ns
mqtt5: Write ConnAck: Mean +- std dev: 81.9 ns +- 0.3 ns
mqtt5: Read ConnAck(full): Mean +- std dev: 269 ns +- 4 ns
mqtt5: Write ConnAck(full): Mean +- std dev: 606 ns +- 8 ns
mqtt5: Read Publish(qos0): Mean +- std dev: 112 ns +- 3 ns
mqtt5: Write Publish(qos0): Mean +- std dev: 209 ns +- 4 ns
mqtt5: Read Publish(qos1): Mean +- std dev: 116 ns +- 7 ns
mqtt5: Write Publish(qos1): Mean +- std dev: 258 ns +- 4 ns
mqtt5: Read PubAck: Mean +- std dev: 63.0 ns +- 3.4 ns
mqtt5: Write PubAck: Mean +- std dev: 97.9 ns +- 1.6 ns
mqtt5: Read PubAck(full): Mean +- std dev: 88.7 ns +- 2.5 ns
mqtt5: Write PubAck(full): Mean +- std dev: 159 ns +- 2 ns
mqtt5: Read Subscribe: Mean +- std dev: 118 ns +- 2 ns
mqtt5: Write Subscribe: Mean +- std dev: 282 ns +- 4 ns
mqtt5: Read SubAck: Mean +- std dev: 100 ns +- 1 ns
mqtt5: Write SubAck: Mean +- std dev: 197 ns +- 2 ns
mqtt5: Read PingReq: Mean +- std dev: 55.9 ns +- 1.3 ns
mqtt5: Write PingReq: Mean +- std dev: 60.0 ns +- 3.6 ns
mqtt5: Read PingResp: Mean +- std dev: 55.7 ns +- 0.4 ns
mqtt5: Write PingResp: Mean +- std dev: 60.5 ns +- 3.6 ns
mqtt5: Read Disconnect: Mean +- std dev: 63.9 ns +- 2.8 ns
mqtt5: Write Disconnect: Mean +- std dev: 67.7 ns +- 0.6 ns
mqtt5: Read Disconnect(full): Mean +- std dev: 117 ns +- 2 ns
mqtt5: Write Disconnect(full): Mean +- std dev: 196 ns +- 3 ns
*/

#[pyclass(frozen, eq, get_all, module = "mqtt5")]
pub struct PublishPacket {
    pub topic: Py<PyString>,
    pub payload: Option<Py<PyBytes>>,
    pub qos: QoS,
    pub retain: bool,
    pub packet_id: Option<u16>,
    pub duplicate: bool,
    pub payload_format_indicator: u8,
    pub message_expiry_interval: Option<u32>,
    pub content_type: Option<Py<PyString>>,
    pub response_topic: Option<Py<PyString>>,
    pub correlation_data: Option<Py<PyBytes>>,
    pub subscription_ids: Py<PyList>,
    pub topic_alias: Option<u16>,
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
        payload_format_indicator=0,
        message_expiry_interval=None,
        content_type=None,
        response_topic=None,
        correlation_data=None,
        subscription_ids=None,
        topic_alias=None,
    ))]
    pub fn new(
        topic: Py<PyString>,
        payload: Option<&Bound<'_, PyBytes>>,
        qos: QoS,
        retain: bool,
        packet_id: Option<u16>,
        duplicate: bool,
        payload_format_indicator: u8,
        message_expiry_interval: Option<u32>,
        content_type: Option<Py<PyString>>,
        response_topic: Option<Py<PyString>>,
        correlation_data: Option<Py<PyBytes>>,
        subscription_ids: Option<&Bound<'_, PyList>>,
        topic_alias: Option<u16>,
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
            topic,
            qos,
            duplicate,
            retain,
            packet_id,
            payload_format_indicator,
            message_expiry_interval,
            content_type,
            response_topic,
            correlation_data,
            subscription_ids: subscription_ids
                .map(|x| x.clone().unbind())
                .unwrap_or_else(|| Python::with_gil(|py| PyList::empty(py).unbind())),
            topic_alias,
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
        let mut properties_nbytes = 0;
        if self.payload_format_indicator != 0 {
            properties_nbytes += 0u8.nbytes() + self.payload_format_indicator.nbytes();
        }
        if let Some(message_expiry_interval) = self.message_expiry_interval {
            properties_nbytes += 0u8.nbytes() + message_expiry_interval.nbytes();
        }
        if let Some(ref content_type) = self.content_type {
            properties_nbytes += 0u8.nbytes() + content_type.nbytes();
        }
        if let Some(ref response_topic) = self.response_topic {
            properties_nbytes += 0u8.nbytes() + response_topic.nbytes();
        }
        if let Some(ref correlation_data) = self.correlation_data {
            properties_nbytes += 0u8.nbytes() + correlation_data.nbytes();
        }
        let subscription_ids = self.subscription_ids.bind(py);
        for element in subscription_ids.try_iter()? {
            let subscription_id: u32 = element?.extract()?;
            properties_nbytes += 0u8.nbytes() + VariableByteInteger::new(subscription_id).nbytes();
        }
        if let Some(topic_alias) = self.topic_alias {
            properties_nbytes += 0u8.nbytes() + topic_alias.nbytes();
        }
        let properties_remaining_length = VariableByteInteger::new(properties_nbytes as u32);
        let nbytes = self.topic.nbytes()
            + self.packet_id.nbytes()
            + properties_remaining_length.nbytes()
            + properties_nbytes
            + payload.map_or(0, |x| x.len());
        let remaining_length = VariableByteInteger::new(nbytes as u32);
        let mut cursor = Cursor::new(buffer, index);
        cursor.require(1 + remaining_length.nbytes() + nbytes)?;

        // [3.3.1] Fixed header
        let first_byte = (PacketType::Publish as u8) << 4
            | (self.duplicate as u8) << 3
            | (self.qos as u8) << 1
            | self.retain as u8;
        first_byte.write(&mut cursor);
        remaining_length.write(&mut cursor);

        // [3.3.2] Variable header
        self.topic.write(&mut cursor);
        self.packet_id.write(&mut cursor);

        // [3.3.2.3] Properties
        properties_remaining_length.write(&mut cursor);
        if self.payload_format_indicator != 0 {
            (PropertyType::PayloadFormatIndicator as u8).write(&mut cursor);
            self.payload_format_indicator.write(&mut cursor);
        }
        if let Some(message_expiry_interval) = self.message_expiry_interval {
            (PropertyType::MessageExpiryInterval as u8).write(&mut cursor);
            message_expiry_interval.write(&mut cursor);
        }
        if let Some(ref content_type) = self.content_type {
            (PropertyType::ContentType as u8).write(&mut cursor);
            content_type.write(&mut cursor);
        }
        if let Some(ref response_topic) = self.response_topic {
            (PropertyType::ResponseTopic as u8).write(&mut cursor);
            response_topic.write(&mut cursor);
        }
        if let Some(ref correlation_data) = self.correlation_data {
            (PropertyType::CorrelationData as u8).write(&mut cursor);
            correlation_data.write(&mut cursor);
        }
        for element in subscription_ids.try_iter()? {
            let id: u32 = element?.extract()?;
            (PropertyType::SubscriptionId as u8).write(&mut cursor);
            VariableByteInteger::new(id).write(&mut cursor);
        }
        if let Some(topic_alias) = self.topic_alias {
            (PropertyType::TopicAlias as u8).write(&mut cursor);
            topic_alias.write(&mut cursor);
        }

        // [3.3.3] Payload
        if let Some(payload) = payload {
            let payload_remaining_length = payload.len();
            cursor.buffer[cursor.index..cursor.index + payload_remaining_length]
                .copy_from_slice(payload);
            cursor.index += payload_remaining_length;
        }

        Ok(cursor.index - index)
    }
}

impl PublishPacket {
    pub fn read(
        py: Python,
        cursor: &mut Cursor,
        flags: u8,
        remaining_length: VariableByteInteger,
    ) -> PyResult<Py<Self>> {
        let start_index = cursor.index;
        let retain = (flags & 0x01) != 0;
        let qos = QoS::new((flags >> 1) & 0x03)?;
        let duplicate = (flags & 0x08) != 0;

        // [3.3.2] Variable header
        let topic = Py::<PyString>::read(cursor)?;
        let packet_id = if qos == QoS::AtLeastOnce || qos == QoS::ExactlyOnce {
            Some(u16::read(cursor)?)
        } else {
            None
        };

        // [3.3.2.3] Properties
        let mut payload_format_indicator = 0;
        let mut message_expiry_interval = None;
        let mut content_type = None;
        let mut response_topic = None;
        let mut correlation_data = None;
        let subscription_ids = pyo3::types::PyList::empty(py);
        let mut topic_alias = None;
        let properties_remaining_length = VariableByteInteger::read(cursor)?.value() as usize;
        let properties_start_index = cursor.index;
        while cursor.index - properties_start_index < properties_remaining_length {
            match crate::types::PropertyType::new(u8::read(cursor)?)? {
                crate::types::PropertyType::PayloadFormatIndicator => {
                    payload_format_indicator = u8::read(cursor)?;
                }
                crate::types::PropertyType::MessageExpiryInterval => {
                    message_expiry_interval = Some(u32::read(cursor)?);
                }
                crate::types::PropertyType::ContentType => {
                    content_type = Some(Py::<PyString>::read(cursor)?);
                }
                crate::types::PropertyType::ResponseTopic => {
                    response_topic = Some(Py::<PyString>::read(cursor)?);
                }
                crate::types::PropertyType::CorrelationData => {
                    correlation_data = Some(Py::<PyBytes>::read(cursor)?);
                }
                crate::types::PropertyType::SubscriptionId => {
                    let sub_id = VariableByteInteger::read(cursor)?.value();
                    subscription_ids.append(sub_id)?;
                }
                crate::types::PropertyType::TopicAlias => {
                    topic_alias = Some(u16::read(cursor)?);
                }
                other => {
                    return Err(PyValueError::new_err(format!(
                        "Invalid property type for PublishPacket: {:?}",
                        other
                    )));
                }
            }
        }

        // [3.3.3] Payload
        let payload_remaining_length =
            start_index + remaining_length.value() as usize - cursor.index;
        let payload = PyBytes::new(
            py,
            &cursor.buffer[cursor.index..cursor.index + payload_remaining_length],
        );
        cursor.index += payload_remaining_length;

        // Return the Python object
        let packet = Self {
            topic,
            payload: Some(payload.unbind()),
            qos,
            retain,
            packet_id,
            duplicate,
            payload_format_indicator,
            message_expiry_interval,
            content_type,
            response_topic,
            correlation_data,
            subscription_ids: subscription_ids.unbind(),
            topic_alias,
        };
        Py::new(py, packet)
    }
}

impl PartialEq for PublishPacket {
    fn eq(&self, other: &Self) -> bool {
        self.topic.py_eq(&other.topic)
            && self.payload.py_eq(&other.payload)
            && self.qos == other.qos
            && self.retain == other.retain
            && self.packet_id == other.packet_id
            && self.duplicate == other.duplicate
            && self.payload_format_indicator == other.payload_format_indicator
            && self.message_expiry_interval == other.message_expiry_interval
            && self.content_type.py_eq(&other.content_type)
            && self.response_topic.py_eq(&other.response_topic)
            && self.correlation_data.py_eq(&other.correlation_data)
            && Python::with_gil(|py| -> PyResult<bool> {
                let list1 = self.subscription_ids.bind(py);
                let list2 = other.subscription_ids.bind(py);
                Ok(list1.len() == list2.len()
                    && list1.try_iter()?.zip(list2.try_iter()?).try_fold(
                        true,
                        |acc, (a, b)| -> PyResult<bool> {
                            let val1: u32 = a?.extract()?;
                            let val2: u32 = b?.extract()?;
                            Ok(acc && val1 == val2)
                        },
                    )?)
            })
            .unwrap_or(false)
            && self.topic_alias == other.topic_alias
    }
}

#[pyclass(frozen, eq, get_all, module = "mqtt5")]
pub struct PubAckPacket {
    pub packet_id: u16,
    pub reason_code: PubAckReasonCode,
    pub reason_string: Option<Py<PyString>>,
}

#[pymethods]
impl PubAckPacket {
    #[new]
    #[pyo3(signature = (
        packet_id,
        *,
        reason_code=PubAckReasonCode::Success,
        reason_string=None,
    ))]
    pub fn new(
        packet_id: u16,
        reason_code: PubAckReasonCode,
        reason_string: Option<Py<PyString>>,
    ) -> PyResult<Self> {
        Ok(Self {
            packet_id,
            reason_code,
            reason_string,
        })
    }

    #[pyo3(signature = (buffer, /, *, index=0))]
    pub fn write(&self, buffer: &Bound<'_, PyByteArray>, index: usize) -> PyResult<usize> {
        let mut properties_nbytes = 0;
        if let Some(ref reason_string) = self.reason_string {
            properties_nbytes += 0u8.nbytes() + reason_string.nbytes();
        }
        let properties_remaining_length = VariableByteInteger::new(properties_nbytes as u32);
        let nbytes = self.packet_id.nbytes()
            + self.reason_code.nbytes()
            + properties_remaining_length.nbytes()
            + properties_nbytes;
        let remaining_length = VariableByteInteger::new(nbytes as u32);
        let mut cursor = Cursor::new(buffer, index);
        cursor.require(1 + remaining_length.nbytes() + nbytes)?;

        // [3.4.1] Fixed header
        let first_byte = (PacketType::PubAck as u8) << 4;
        first_byte.write(&mut cursor);
        remaining_length.write(&mut cursor);

        // [3.4.2] Variable header
        self.packet_id.write(&mut cursor);
        self.reason_code.write(&mut cursor);

        // [3.4.2.2] Properties
        properties_remaining_length.write(&mut cursor);
        if let Some(ref reason_string) = self.reason_string {
            (PropertyType::ReasonString as u8).write(&mut cursor);
            reason_string.write(&mut cursor);
        }

        Ok(cursor.index - index)
    }
}

impl PubAckPacket {
    pub fn read(
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
        let reason_code = if remaining_length.value() > 2 {
            PubAckReasonCode::read(cursor)?
        } else {
            PubAckReasonCode::Success
        };

        // [3.4.2.2] Properties
        let mut reason_string = None;
        if remaining_length.value() > 3 {
            let properties_remaining_length = VariableByteInteger::read(cursor)?.value() as usize;
            let properties_start_index = cursor.index;
            while cursor.index - properties_start_index < properties_remaining_length {
                match crate::types::PropertyType::new(u8::read(cursor)?)? {
                    crate::types::PropertyType::ReasonString => {
                        reason_string = Some(Py::<PyString>::read(cursor)?);
                    }
                    other => {
                        return Err(PyValueError::new_err(format!(
                            "Invalid property type for PubAckPacket: {:?}",
                            other
                        )));
                    }
                }
            }
        }

        // Return the Python object
        let packet = Self {
            packet_id,
            reason_code,
            reason_string,
        };
        Py::new(py, packet)
    }
}

impl PartialEq for PubAckPacket {
    fn eq(&self, other: &Self) -> bool {
        self.packet_id == other.packet_id
            && self.reason_code == other.reason_code
            && self.reason_string.py_eq(&other.reason_string)
    }
}

#[pyclass(frozen, eq, get_all, module = "mqtt5")]
pub struct SubscribePacket {
    pub packet_id: u16,
    pub subscriptions: Py<PyList>,
    pub subscription_id: Option<VariableByteInteger>,
}

#[pymethods]
impl SubscribePacket {
    #[new]
    #[pyo3(signature = (
        packet_id,
        subscriptions,
        *,
        subscription_id=None,
    ))]
    pub fn new(
        packet_id: u16,
        subscriptions: &Bound<'_, PyList>,
        subscription_id: Option<VariableByteInteger>,
    ) -> PyResult<Self> {
        Ok(Self {
            packet_id,
            subscriptions: subscriptions.clone().unbind(),
            subscription_id,
        })
    }

    #[pyo3(signature = (buffer, /, *, index=0))]
    pub fn write(
        &self,
        py: Python,
        buffer: &Bound<'_, PyByteArray>,
        index: usize,
    ) -> PyResult<usize> {
        let mut properties_nbytes = 0;
        if let Some(ref subscription_id) = self.subscription_id {
            properties_nbytes += 0u8.nbytes() + subscription_id.nbytes();
        }
        let properties_remaining_length = VariableByteInteger::new(properties_nbytes as u32);
        let subscriptions = self.subscriptions.bind(py);
        let nbytes = self.packet_id.nbytes()
            + properties_remaining_length.nbytes()
            + properties_nbytes
            + subscriptions
                .try_iter()?
                .try_fold(0, |acc, item| -> PyResult<usize> {
                    Ok(acc + item?.extract::<PyRef<Subscription>>()?.pattern.nbytes() + 1)
                })?;
        let remaining_length = VariableByteInteger::new(nbytes as u32);
        let mut cursor = Cursor::new(buffer, index);
        cursor.require(1 + remaining_length.nbytes() + nbytes)?;

        // [3.8.1] Fixed header
        let first_byte = (PacketType::Subscribe as u8) << 4 | 0x02;
        first_byte.write(&mut cursor);
        remaining_length.write(&mut cursor);

        // [3.8.2] Variable header
        self.packet_id.write(&mut cursor);

        // [3.8.2.1] Properties
        properties_remaining_length.write(&mut cursor);
        if let Some(ref subscription_id) = self.subscription_id {
            (PropertyType::SubscriptionId as u8).write(&mut cursor);
            subscription_id.write(&mut cursor);
        }

        // [3.8.3] Payload
        for item in subscriptions.try_iter()? {
            let subscription: PyRef<Subscription> = item?.extract()?;
            subscription.pattern.write(&mut cursor);
            let options = subscription.maximum_qos as u8
                | (subscription.no_local as u8) << 2
                | (subscription.retain_as_published as u8) << 3
                | (subscription.retain_handling as u8) << 4;
            options.write(&mut cursor);
        }

        Ok(cursor.index - index)
    }
}

impl SubscribePacket {
    pub fn read(
        py: Python,
        cursor: &mut Cursor,
        flags: u8,
        remaining_length: VariableByteInteger,
    ) -> PyResult<Py<Self>> {
        if flags != 0x02 {
            return Err(PyValueError::new_err("Malformed bytes"));
        }
        let start_index = cursor.index;

        // [3.8.2] Variable header
        let packet_id = u16::read(cursor)?;

        // [3.8.2.1] Properties
        let mut subscription_id = None;
        let properties_remaining_length = VariableByteInteger::read(cursor)?.value() as usize;
        let properties_start_index = cursor.index;
        while cursor.index - properties_start_index < properties_remaining_length {
            match crate::types::PropertyType::new(u8::read(cursor)?)? {
                crate::types::PropertyType::SubscriptionId => {
                    subscription_id = Some(VariableByteInteger::read(cursor)?);
                }
                other => {
                    return Err(PyValueError::new_err(format!(
                        "Invalid property type for SubscribePacket: {:?}",
                        other
                    )));
                }
            }
        }

        // [3.8.3] Payload
        let subscriptions = PyList::empty(py);
        while cursor.index - start_index < remaining_length.value() as usize {
            let pattern = Py::<PyString>::read(cursor)?;
            let options = u8::read(cursor)?;
            let subscription = Subscription {
                pattern,
                maximum_qos: QoS::new(options & 0x03)?,
                no_local: (options >> 2) & 0x01 != 0,
                retain_as_published: (options >> 3) & 0x01 != 0,
                retain_handling: RetainHandling::new((options >> 4) & 0x03)?,
            };
            subscriptions.append(subscription)?;
        }

        // Return the Python object
        let packet = Self {
            packet_id,
            subscriptions: subscriptions.unbind(),
            subscription_id,
        };
        Py::new(py, packet)
    }
}

impl PartialEq for SubscribePacket {
    fn eq(&self, other: &Self) -> bool {
        self.packet_id == other.packet_id
            && self.subscription_id == other.subscription_id
            && Python::with_gil(|py| -> PyResult<bool> {
                let seq1 = self.subscriptions.bind(py);
                let seq2 = other.subscriptions.bind(py);
                Ok(seq1.len() == seq2.len()
                    && seq1.try_iter()?.zip(seq2.try_iter()?).try_fold(
                        true,
                        |acc, (a, b)| -> PyResult<bool> {
                            let sub1: PyRef<Subscription> = a?.extract()?;
                            let sub2: PyRef<Subscription> = b?.extract()?;
                            Ok(acc && *sub1 == *sub2)
                        },
                    )?)
            })
            .unwrap_or(false)
    }
}

#[pyclass(frozen, eq, get_all, module = "mqtt5")]
pub struct SubAckPacket {
    pub packet_id: u16,
    pub reason_codes: Py<PyList>,
    pub reason_string: Option<Py<PyString>>,
}

#[pymethods]
impl SubAckPacket {
    #[new]
    #[pyo3(signature = (
        packet_id,
        reason_codes,
        *,
        reason_string=None,
    ))]
    pub fn new(
        packet_id: u16,
        reason_codes: &Bound<'_, PyList>,
        reason_string: Option<Py<PyString>>,
    ) -> PyResult<Self> {
        Ok(Self {
            packet_id,
            reason_codes: reason_codes.clone().unbind(),
            reason_string,
        })
    }

    #[pyo3(signature = (buffer, /, *, index=0))]
    pub fn write(
        &self,
        py: Python,
        buffer: &Bound<'_, PyByteArray>,
        index: usize,
    ) -> PyResult<usize> {
        let mut properties_nbytes = 0;
        if let Some(ref reason_string) = self.reason_string {
            properties_nbytes += 0u8.nbytes() + reason_string.nbytes();
        }
        let properties_remaining_length = VariableByteInteger::new(properties_nbytes as u32);
        let reason_codes = self.reason_codes.bind(py);
        let nbytes = self.packet_id.nbytes()
            + properties_remaining_length.nbytes()
            + properties_nbytes
            + reason_codes
                .try_iter()?
                .try_fold(0, |acc, item| -> PyResult<usize> {
                    Ok(acc + item?.extract::<PyRef<SubAckReasonCode>>()?.nbytes())
                })?;
        let remaining_length = VariableByteInteger::new(nbytes as u32);
        let mut cursor = Cursor::new(buffer, index);
        cursor.require(1 + remaining_length.nbytes() + nbytes)?;

        // [3.9.1] Fixed header
        let first_byte = (PacketType::SubAck as u8) << 4;
        first_byte.write(&mut cursor);
        remaining_length.write(&mut cursor);

        // [3.9.2] Variable header
        self.packet_id.write(&mut cursor);

        // [3.9.2.1] Properties
        properties_remaining_length.write(&mut cursor);
        if let Some(ref reason_string) = self.reason_string {
            (PropertyType::ReasonString as u8).write(&mut cursor);
            reason_string.write(&mut cursor);
        }

        // [3.9.3] Payload
        for item in reason_codes.try_iter()? {
            let reason_code: PyRef<SubAckReasonCode> = item?.extract()?;
            reason_code.write(&mut cursor);
        }

        Ok(cursor.index - index)
    }
}

impl SubAckPacket {
    pub fn read(
        py: Python,
        cursor: &mut Cursor,
        flags: u8,
        remaining_length: VariableByteInteger,
    ) -> PyResult<Py<Self>> {
        if flags != 0x00 {
            return Err(PyValueError::new_err("Malformed bytes"));
        }
        let start_index = cursor.index;

        // [3.9.2] Variable header
        let packet_id = u16::read(cursor)?;

        // [3.9.2.1] Properties
        let mut reason_string = None;
        let properties_remaining_length = VariableByteInteger::read(cursor)?.value() as usize;
        let properties_start_index = cursor.index;
        while cursor.index - properties_start_index < properties_remaining_length {
            match crate::types::PropertyType::new(u8::read(cursor)?)? {
                crate::types::PropertyType::ReasonString => {
                    reason_string = Some(Py::<PyString>::read(cursor)?);
                }
                other => {
                    return Err(PyValueError::new_err(format!(
                        "Invalid property type for SubAckPacket: {:?}",
                        other
                    )));
                }
            }
        }

        // [3.9.3] Payload
        let reason_codes = PyList::empty(py);
        while cursor.index - start_index < remaining_length.value() as usize {
            let reason_code = SubAckReasonCode::read(cursor)?;
            reason_codes.append(reason_code)?;
        }

        // Return the Python object
        let packet = Self {
            packet_id,
            reason_codes: reason_codes.unbind(),
            reason_string,
        };
        Py::new(py, packet)
    }
}

impl PartialEq for SubAckPacket {
    fn eq(&self, other: &Self) -> bool {
        self.packet_id == other.packet_id
            && self.reason_string.py_eq(&other.reason_string)
            && Python::with_gil(|py| -> PyResult<bool> {
                let seq1 = self.reason_codes.bind(py);
                let seq2 = other.reason_codes.bind(py);
                Ok(seq1.len() == seq2.len()
                    && seq1.try_iter()?.zip(seq2.try_iter()?).try_fold(
                        true,
                        |acc, (a, b)| -> PyResult<bool> {
                            let sub1: PyRef<SubAckReasonCode> = a?.extract()?;
                            let sub2: PyRef<SubAckReasonCode> = b?.extract()?;
                            Ok(acc && *sub1 == *sub2)
                        },
                    )?)
            })
            .unwrap_or(false)
    }
}

#[pyclass(frozen, eq, get_all, module = "mqtt5")]
pub struct PingReqPacket {}

#[pymethods]
impl PingReqPacket {
    #[new]
    #[pyo3(signature = ())]
    pub fn new() -> PyResult<Self> {
        Ok(Self {})
    }

    #[pyo3(signature = (buffer, /, *, index=0))]
    pub fn write(&self, buffer: &Bound<'_, PyByteArray>, index: usize) -> PyResult<usize> {
        let remaining_length = VariableByteInteger::new(0);
        let mut cursor = Cursor::new(buffer, index);
        cursor.require(2)?;

        // [3.12.1] Fixed header
        let first_byte = (PacketType::PingReq as u8) << 4;
        first_byte.write(&mut cursor);
        remaining_length.write(&mut cursor);

        Ok(cursor.index - index)
    }
}

impl PingReqPacket {
    pub fn read(
        py: Python,
        _cursor: &mut Cursor,
        flags: u8,
        _remaining_length: VariableByteInteger,
    ) -> PyResult<Py<Self>> {
        if flags != 0x00 {
            return Err(PyValueError::new_err("Malformed bytes"));
        }

        // Return the Python object
        let packet = Self {};
        Py::new(py, packet)
    }
}

impl PartialEq for PingReqPacket {
    fn eq(&self, _other: &Self) -> bool {
        true
    }
}

#[pyclass(frozen, eq, get_all, module = "mqtt5")]
pub struct PingRespPacket {}

#[pymethods]
impl PingRespPacket {
    #[new]
    #[pyo3(signature = ())]
    pub fn new() -> PyResult<Self> {
        Ok(Self {})
    }

    #[pyo3(signature = (buffer, /, *, index=0))]
    pub fn write(&self, buffer: &Bound<'_, PyByteArray>, index: usize) -> PyResult<usize> {
        let remaining_length = VariableByteInteger::new(0);
        let mut cursor = Cursor::new(buffer, index);
        cursor.require(2)?;

        // [3.13.1] Fixed header
        let first_byte = (PacketType::PingResp as u8) << 4;
        first_byte.write(&mut cursor);
        remaining_length.write(&mut cursor);

        Ok(cursor.index - index)
    }
}

impl PingRespPacket {
    pub fn read(
        py: Python,
        _cursor: &mut Cursor,
        flags: u8,
        _remaining_length: VariableByteInteger,
    ) -> PyResult<Py<Self>> {
        if flags != 0x00 {
            return Err(PyValueError::new_err("Malformed bytes"));
        }

        // Return the Python object
        let packet = Self {};
        Py::new(py, packet)
    }
}

impl PartialEq for PingRespPacket {
    fn eq(&self, _other: &Self) -> bool {
        true
    }
}

#[pyclass(frozen, eq, get_all, module = "mqtt5")]
pub struct DisconnectPacket {
    pub reason_code: DisconnectReasonCode,
    pub session_expiry_interval: Option<u32>,
    pub server_reference: Option<Py<PyString>>,
    pub reason_string: Option<Py<PyString>>,
}

#[pymethods]
impl DisconnectPacket {
    #[new]
    #[pyo3(signature = (
        *,
        reason_code=DisconnectReasonCode::NormalDisconnection,
        session_expiry_interval=None,
        server_reference=None,
        reason_string=None,
    ))]
    pub fn new(
        reason_code: DisconnectReasonCode,
        session_expiry_interval: Option<u32>,
        server_reference: Option<Py<PyString>>,
        reason_string: Option<Py<PyString>>,
    ) -> PyResult<Self> {
        Ok(Self {
            reason_code,
            session_expiry_interval,
            server_reference,
            reason_string,
        })
    }

    #[pyo3(signature = (buffer, /, *, index=0))]
    pub fn write(&self, buffer: &Bound<'_, PyByteArray>, index: usize) -> PyResult<usize> {
        let mut properties_nbytes = 0;
        if let Some(session_expiry_interval) = self.session_expiry_interval {
            properties_nbytes += 0u8.nbytes() + session_expiry_interval.nbytes();
        }
        if let Some(ref server_reference) = self.server_reference {
            properties_nbytes += 0u8.nbytes() + server_reference.nbytes();
        }
        if let Some(ref reason_string) = self.reason_string {
            properties_nbytes += 0u8.nbytes() + reason_string.nbytes();
        }
        let properties_remaining_length = VariableByteInteger::new(properties_nbytes as u32);
        let nbytes =
            self.reason_code.nbytes() + properties_remaining_length.nbytes() + properties_nbytes;
        let remaining_length = VariableByteInteger::new(nbytes as u32);
        let mut cursor = Cursor::new(buffer, index);
        cursor.require(1 + remaining_length.nbytes() + nbytes)?;

        // [3.14.1] Fixed header
        let first_byte = (PacketType::Disconnect as u8) << 4;
        first_byte.write(&mut cursor);
        remaining_length.write(&mut cursor);

        // [3.14.2] Variable header
        self.reason_code.write(&mut cursor);

        // [3.14.2.2] Properties
        properties_remaining_length.write(&mut cursor);
        if let Some(session_expiry_interval) = self.session_expiry_interval {
            (PropertyType::SessionExpiryInterval as u8).write(&mut cursor);
            session_expiry_interval.write(&mut cursor);
        }
        if let Some(ref server_reference) = self.server_reference {
            (PropertyType::ServerReference as u8).write(&mut cursor);
            server_reference.write(&mut cursor);
        }
        if let Some(ref reason_string) = self.reason_string {
            (PropertyType::ReasonString as u8).write(&mut cursor);
            reason_string.write(&mut cursor);
        }

        Ok(cursor.index - index)
    }
}

impl DisconnectPacket {
    pub fn read(
        py: Python,
        cursor: &mut Cursor,
        flags: u8,
        remaining_length: VariableByteInteger,
    ) -> PyResult<Py<Self>> {
        if flags != 0x00 {
            return Err(PyValueError::new_err("Malformed bytes"));
        }

        // [3.14.2] Variable header
        let reason_code = if remaining_length.value() > 0 {
            DisconnectReasonCode::read(cursor)?
        } else {
            DisconnectReasonCode::NormalDisconnection
        };

        // [3.14.2.2] Properties
        let mut session_expiry_interval = None;
        let mut server_reference = None;
        let mut reason_string = None;
        if remaining_length.value() > 1 {
            let properties_remaining_length = VariableByteInteger::read(cursor)?.value() as usize;
            let properties_start_index = cursor.index;
            while cursor.index - properties_start_index < properties_remaining_length {
                match crate::types::PropertyType::new(u8::read(cursor)?)? {
                    crate::types::PropertyType::SessionExpiryInterval => {
                        session_expiry_interval = Some(u32::read(cursor)?);
                    }
                    crate::types::PropertyType::ServerReference => {
                        server_reference = Some(Py::<PyString>::read(cursor)?);
                    }
                    crate::types::PropertyType::ReasonString => {
                        reason_string = Some(Py::<PyString>::read(cursor)?);
                    }
                    other => {
                        return Err(PyValueError::new_err(format!(
                            "Invalid property type for DisconnectPacket: {:?}",
                            other
                        )));
                    }
                }
            }
        }

        // Return the Python object
        let packet = Self {
            reason_code,
            session_expiry_interval,
            server_reference,
            reason_string,
        };
        Py::new(py, packet)
    }
}

impl PartialEq for DisconnectPacket {
    fn eq(&self, other: &Self) -> bool {
        self.reason_code == other.reason_code
            && self.session_expiry_interval == other.session_expiry_interval
            && self.server_reference.py_eq(&other.server_reference)
            && self.reason_string.py_eq(&other.reason_string)
    }
}
