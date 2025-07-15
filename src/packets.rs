use crate::io::{Cursor, Readable, VariableByteInteger, Writable};
use crate::properties::{PyEq, *};
use crate::reason_codes::*;
use crate::types::{PacketType, QoS, RetainHandling};
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
    pub properties: WillProperties,
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
        self.pattern.equals(&other.pattern)
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
    pub properties: ConnectProperties,
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
        let size = PROTOCOL_NAME.size()
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
        let remaining_length = VariableByteInteger(size as u32);
        let mut cursor = Cursor::new(buffer, index);
        cursor.assert(1 + remaining_length.size() + size)?;

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
    pub fn read(
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
        let packet_flags = u8::read(cursor)?;
        let clean_start = (packet_flags & 0x02) != 0;
        let keep_alive = u16::read(cursor)?;
        let properties = ConnectProperties::read(cursor)?;

        // [3.1.3] Payload
        let client_id = Py::<PyString>::read(cursor)?;
        let will = if (packet_flags & 0x04) != 0 {
            let properties = WillProperties::read(cursor)?;
            let topic = Py::<PyString>::read(cursor)?;
            let payload = Py::<PyBytes>::read(cursor)?;
            Some(Will {
                topic,
                payload: Some(payload),
                qos: QoS::new((packet_flags >> 3) & 0x03)?,
                retain: (packet_flags & 0x20) != 0,
                properties,
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

#[pyclass(eq, get_all, module = "mqtt5")]
pub struct ConnAckPacket {
    pub session_present: bool,
    pub reason_code: ConnAckReasonCode,
    pub properties: ConnAckProperties,
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
        let size = 0u8.size() + self.reason_code.size() + self.properties.size();
        let remaining_length = VariableByteInteger(size as u32);
        let mut cursor = Cursor::new(buffer, index);
        cursor.assert(1 + remaining_length.size() + size)?;

        // [3.2.1] Fixed header
        let first_byte = (PacketType::ConnAck as u8) << 4;
        first_byte.write(&mut cursor);
        remaining_length.write(&mut cursor);

        // [3.2.2] Variable header
        let packet_flags = self.session_present as u8;
        packet_flags.write(&mut cursor);
        self.reason_code.write(&mut cursor);
        self.properties.write(&mut cursor);

        Ok(cursor.index - index)
    }
}

impl ConnAckPacket {
    pub fn read(
        py: Python,
        cursor: &mut Cursor,
        flags: u8,
        remaining_length: VariableByteInteger,
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

#[pyclass(frozen, eq, get_all, module = "mqtt5")]
pub struct PublishPacket {
    pub topic: Py<PyString>,
    pub payload: Option<Py<PyBytes>>,
    pub qos: QoS,
    pub retain: bool,
    pub packet_id: Option<u16>,
    pub duplicate: bool,
    pub properties: PublishProperties,
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
        let size = self.topic.size()
            + self.packet_id.size()
            + self.properties.size()
            + payload.map_or(0, |x| x.len());
        let remaining_length = VariableByteInteger(size as u32);
        let mut cursor = Cursor::new(buffer, index);
        cursor.assert(1 + remaining_length.size() + size)?;

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
    pub fn read(
        py: Python,
        cursor: &mut Cursor,
        flags: u8,
        remaining_length: VariableByteInteger,
    ) -> PyResult<Py<Self>> {
        let i0 = cursor.index;
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

#[pyclass(frozen, eq, get_all, module = "mqtt5")]
pub struct PubAckPacket {
    pub packet_id: u16,
    pub reason_code: PubAckReasonCode,
    pub properties: PubAckProperties,
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
        let size = self.packet_id.size() + self.reason_code.size() + self.properties.size();
        let remaining_length = VariableByteInteger(size as u32);
        let mut cursor = Cursor::new(buffer, index);
        cursor.assert(1 + remaining_length.size() + size)?;

        // [3.4.1] Fixed header
        let first_byte = (PacketType::PubAck as u8) << 4;
        first_byte.write(&mut cursor);
        remaining_length.write(&mut cursor);

        // [3.4.2] Variable header
        self.packet_id.write(&mut cursor);
        self.reason_code.write(&mut cursor);
        self.properties.write(&mut cursor);

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

#[pyclass(frozen, eq, get_all, module = "mqtt5")]
pub struct SubscribePacket {
    pub packet_id: u16,
    pub subscriptions: Py<PyList>,
    pub properties: SubscribeProperties,
}

#[pymethods]
impl SubscribePacket {
    #[new]
    #[pyo3(signature = (
        packet_id,
        subscriptions,
        *,
        properties=None,
    ))]
    pub fn new(
        py: Python,
        packet_id: u16,
        subscriptions: &Bound<'_, PyList>,
        properties: Option<SubscribeProperties>,
    ) -> PyResult<Self> {
        Ok(Self {
            packet_id,
            subscriptions: subscriptions.clone().unbind(),
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
        let subscriptions = self.subscriptions.bind(py);
        let size = self.packet_id.size()
            + self.properties.size()
            + subscriptions
                .try_iter()?
                .try_fold(0, |acc, item| -> PyResult<usize> {
                    Ok(acc + item?.extract::<PyRef<Subscription>>()?.pattern.size() + 1)
                })?;
        let remaining_length = VariableByteInteger(size as u32);
        let mut cursor = Cursor::new(buffer, index);
        cursor.assert(1 + remaining_length.size() + size)?;

        // [3.8.1] Fixed header
        let first_byte = (PacketType::Subscribe as u8) << 4 | 0x02;
        first_byte.write(&mut cursor);
        remaining_length.write(&mut cursor);

        // [3.8.2] Variable header
        self.packet_id.write(&mut cursor);
        self.properties.write(&mut cursor);

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
        let i0 = cursor.index;

        // [3.8.2] Variable header
        let packet_id = u16::read(cursor)?;
        let properties = SubscribeProperties::read(cursor)?;

        // [3.8.3] Payload
        let subscriptions = PyList::empty(py);
        while cursor.index - i0 < remaining_length.get() as usize {
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

        // Return Python object
        let packet = Self {
            packet_id,
            subscriptions: subscriptions.unbind(),
            properties,
        };
        Py::new(py, packet)
    }
}

impl PartialEq for SubscribePacket {
    fn eq(&self, other: &Self) -> bool {
        self.packet_id == other.packet_id
            && self.properties == other.properties
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
    pub properties: SubAckProperties,
}

#[pymethods]
impl SubAckPacket {
    #[new]
    #[pyo3(signature = (
        packet_id,
        reason_codes,
        *,
        properties=None,
    ))]
    pub fn new(
        py: Python,
        packet_id: u16,
        reason_codes: &Bound<'_, PyList>,
        properties: Option<SubAckProperties>,
    ) -> PyResult<Self> {
        Ok(Self {
            packet_id,
            reason_codes: reason_codes.clone().unbind(),
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
        let reason_codes = self.reason_codes.bind(py);
        let size = self.packet_id.size()
            + self.properties.size()
            + reason_codes
                .try_iter()?
                .try_fold(0, |acc, item| -> PyResult<usize> {
                    Ok(acc + item?.extract::<PyRef<SubAckReasonCode>>()?.size())
                })?;

        let remaining_length = VariableByteInteger(size as u32);
        let mut cursor = Cursor::new(buffer, index);
        cursor.assert(1 + remaining_length.size() + size)?;

        // [3.9.1] Fixed header
        let first_byte = (PacketType::SubAck as u8) << 4;
        first_byte.write(&mut cursor);
        remaining_length.write(&mut cursor);

        // [3.9.2] Variable header
        self.packet_id.write(&mut cursor);
        self.properties.write(&mut cursor);

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
        let i0 = cursor.index;

        // [3.9.2] Variable header
        let packet_id = u16::read(cursor)?;
        let properties = SubAckProperties::read(cursor)?;

        // [3.9.3] Payload
        let reason_codes = PyList::empty(py);
        while cursor.index - i0 < remaining_length.get() as usize {
            let reason_code = SubAckReasonCode::read(cursor)?;
            reason_codes.append(reason_code)?;
        }

        // Return Python object
        let packet = Self {
            packet_id,
            reason_codes: reason_codes.unbind(),
            properties,
        };
        Py::new(py, packet)
    }
}

impl PartialEq for SubAckPacket {
    fn eq(&self, other: &Self) -> bool {
        self.packet_id == other.packet_id
            && self.properties == other.properties
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
pub struct DisconnectPacket {
    pub reason_code: DisconnectReasonCode,
    pub properties: DisconnectProperties,
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
        let size = self.reason_code.size() + self.properties.size();
        let remaining_length = VariableByteInteger(size as u32);
        let mut cursor = Cursor::new(buffer, index);
        cursor.assert(1 + remaining_length.size() + size)?;

        // [3.14.1] Fixed header
        let first_byte = (PacketType::Disconnect as u8) << 4;
        first_byte.write(&mut cursor);
        remaining_length.write(&mut cursor);

        // [3.14.2] Variable header
        self.reason_code.write(&mut cursor);
        self.properties.write(&mut cursor);

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
