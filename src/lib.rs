mod io;
mod packets;
mod reason_codes;
mod types;

use io::{Cursor, Readable, VariableByteInteger};
use packets::*;
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
        PacketType::PubRec => {
            let packet = PubRecPacket::read(py, &mut cursor, flags, remaining_length)?;
            Ok((packet.into(), cursor.index))
        }
        PacketType::PubRel => {
            let packet = PubRelPacket::read(py, &mut cursor, flags, remaining_length)?;
            Ok((packet.into(), cursor.index))
        }
        PacketType::PubComp => {
            let packet = PubCompPacket::read(py, &mut cursor, flags, remaining_length)?;
            Ok((packet.into(), cursor.index))
        }
        PacketType::Subscribe => {
            let packet = SubscribePacket::read(py, &mut cursor, flags, remaining_length)?;
            Ok((packet.into(), cursor.index))
        }
        PacketType::SubAck => {
            let packet = SubAckPacket::read(py, &mut cursor, flags, remaining_length)?;
            Ok((packet.into(), cursor.index))
        }
        PacketType::Unsubscribe => {
            let packet = UnsubscribePacket::read(py, &mut cursor, flags, remaining_length)?;
            Ok((packet.into(), cursor.index))
        }
        PacketType::UnsubAck => {
            let packet = UnsubAckPacket::read(py, &mut cursor, flags, remaining_length)?;
            Ok((packet.into(), cursor.index))
        }
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
    m.add_class::<PubRecPacket>()?;
    m.add_class::<PubRelPacket>()?;
    m.add_class::<PubCompPacket>()?;
    m.add_class::<SubscribePacket>()?;
    m.add_class::<SubAckPacket>()?;
    m.add_class::<UnsubscribePacket>()?;
    m.add_class::<UnsubAckPacket>()?;
    m.add_class::<PingReqPacket>()?;
    m.add_class::<PingRespPacket>()?;
    m.add_class::<DisconnectPacket>()?;
    // Reason codes
    m.add_class::<ConnAckReasonCode>()?;
    m.add_class::<PubAckReasonCode>()?;
    m.add_class::<PubRecReasonCode>()?;
    m.add_class::<PubRelReasonCode>()?;
    m.add_class::<PubCompReasonCode>()?;
    m.add_class::<SubAckReasonCode>()?;
    m.add_class::<UnsubAckReasonCode>()?;
    m.add_class::<DisconnectReasonCode>()?;
    // Misc
    m.add_class::<QoS>()?;
    m.add_class::<RetainHandling>()?;
    m.add_class::<Will>()?;
    m.add_class::<Subscription>()?;
    // Functions
    m.add_function(wrap_pyfunction!(read, m)?)?;
    Ok(())
}
