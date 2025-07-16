mod io;
mod packets;
mod properties;
mod reason_codes;
mod types;

use io::{Cursor, Readable, VariableByteInteger};
use packets::*;
use properties::*;
use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;
use pyo3::types::PyByteArray;
use pyo3::PyResult;
use reason_codes::*;
use types::*;

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
        PacketType::Subscribe => {
            let packet = SubscribePacket::read(py, &mut cursor, flags, remaining_length)?;
            Ok((packet.into(), cursor.index))
        }
        PacketType::SubAck => {
            let packet = SubAckPacket::read(py, &mut cursor, flags, remaining_length)?;
            Ok((packet.into(), cursor.index))
        }
        PacketType::Unsubscribe => Err(PyValueError::new_err("Not implemented")),
        PacketType::UnsubAck => Err(PyValueError::new_err("Not implemented")),
        PacketType::PingReq => {
            let packet = PingReqPacket::read(py, &mut cursor, flags, remaining_length)?;
            Ok((packet.into(), cursor.index))
        }
        PacketType::PingResp => {
            let packet = PingRespPacket::read(py, &mut cursor, flags, remaining_length)?;
            Ok((packet.into(), cursor.index))
        }
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
    m.add_class::<SubscribePacket>()?;
    m.add_class::<SubAckPacket>()?;
    m.add_class::<PingReqPacket>()?;
    m.add_class::<PingRespPacket>()?;
    m.add_class::<DisconnectPacket>()?;
    // Properties
    m.add_class::<ConnectProperties>()?;
    m.add_class::<ConnAckProperties>()?;
    m.add_class::<PublishProperties>()?;
    m.add_class::<PubAckProperties>()?;
    m.add_class::<SubscribeProperties>()?;
    m.add_class::<SubAckProperties>()?;
    m.add_class::<DisconnectProperties>()?;
    // Reason codes
    m.add_class::<ConnAckReasonCode>()?;
    m.add_class::<PubAckReasonCode>()?;
    m.add_class::<SubAckReasonCode>()?;
    m.add_class::<DisconnectReasonCode>()?;
    // Misc
    m.add_class::<QoS>()?;
    m.add_class::<RetainHandling>()?;
    m.add_class::<WillProperties>()?;
    m.add_class::<Will>()?;
    m.add_class::<Subscription>()?;
    // Functions
    m.add_function(wrap_pyfunction!(read, m)?)?;
    Ok(())
}
