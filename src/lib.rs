mod io;
mod packets;
mod reason_codes;
mod types;

use io::{Cursor, Readable, VariableByteInteger};
use packets::*;
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
    #[rustfmt::skip]
    let packet = match PacketType::new(first_byte >> 4)? {
        PacketType::Connect => ConnectPacket::read(py, &mut cursor, flags, remaining_length)?.into(),
        PacketType::ConnAck => ConnAckPacket::read(py, &mut cursor, flags, remaining_length)?.into(),
        PacketType::Publish => PublishPacket::read(py, &mut cursor, flags, remaining_length)?.into(),
        PacketType::PubAck => PubAckPacket::read(py, &mut cursor, flags, remaining_length)?.into(),
        PacketType::PubRec => PubRecPacket::read(py, &mut cursor, flags, remaining_length)?.into(),
        PacketType::PubRel => PubRelPacket::read(py, &mut cursor, flags, remaining_length)?.into(),
        PacketType::PubComp => PubCompPacket::read(py, &mut cursor, flags, remaining_length)?.into(),
        PacketType::Subscribe => SubscribePacket::read(py, &mut cursor, flags, remaining_length)?.into(),
        PacketType::SubAck => SubAckPacket::read(py, &mut cursor, flags, remaining_length)?.into(),
        PacketType::Unsubscribe => UnsubscribePacket::read(py, &mut cursor, flags, remaining_length)?.into(),
        PacketType::UnsubAck => UnsubAckPacket::read(py, &mut cursor, flags, remaining_length)?.into(),
        PacketType::PingReq => PingReqPacket::read(py, &mut cursor, flags, remaining_length)?.into(),
        PacketType::PingResp => PingRespPacket::read(py, &mut cursor, flags, remaining_length)?.into(),
        PacketType::Disconnect => DisconnectPacket::read(py, &mut cursor, flags, remaining_length)?.into(),
        PacketType::Auth => AuthPacket::read(py, &mut cursor, flags, remaining_length)?.into(),
    };
    Ok((packet, cursor.index - index))
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
    m.add_class::<AuthPacket>()?;
    // Reason codes
    m.add_class::<ConnAckReasonCode>()?;
    m.add_class::<PubAckReasonCode>()?;
    m.add_class::<PubRecReasonCode>()?;
    m.add_class::<PubRelReasonCode>()?;
    m.add_class::<PubCompReasonCode>()?;
    m.add_class::<SubAckReasonCode>()?;
    m.add_class::<UnsubAckReasonCode>()?;
    m.add_class::<DisconnectReasonCode>()?;
    m.add_class::<AuthReasonCode>()?;
    // Misc
    m.add_class::<QoS>()?;
    m.add_class::<RetainHandling>()?;
    m.add_class::<Will>()?;
    m.add_class::<Subscription>()?;
    // Functions
    m.add_function(wrap_pyfunction!(read, m)?)?;
    Ok(())
}
