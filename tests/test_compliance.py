"""Tests specification compliance by comparing mqtt5 against other libraries."""

import conftest
import mqttproto
import pytest
import zmqtt._internal.packets
import zmqtt._internal.packets.codec

import mqtt5


@pytest.mark.parametrize(
    ("packet", "packet_mqttproto"),
    zip(conftest.PACKETS, conftest.PACKETS_MQTTPROTO, strict=True),
    ids=conftest.PACKET_NAMES,
)
def test_compliance_mqttproto(
    packet: mqtt5.Packet,
    packet_mqttproto: mqttproto.MQTTPacket,
    request: pytest.FixtureRequest,
) -> None:
    """Test that mqtt5 writes the same bytes as mqttproto."""
    identifier = request.node.callspec.id
    if identifier in {"PubAck", "PubRec", "PubRel", "PubComp", "Disconnect"}:
        # Not all packets have to include the property length when they have no
        # properties. We optimize for this case, but mqttproto doesn't.
        pytest.xfail("Mismatch due to encoding optimization")

    data = packet.write()
    buffer_mqttproto = bytearray()
    packet_mqttproto.encode(buffer_mqttproto)
    assert len(data) == len(buffer_mqttproto)
    assert data == buffer_mqttproto


@pytest.mark.parametrize(
    ("packet", "packet_zmqtt"),
    zip(conftest.PACKETS, conftest.PACKETS_ZMQTT, strict=True),
    ids=conftest.PACKET_NAMES,
)
def test_compliance_zmqtt(
    packet: mqtt5.Packet,
    packet_zmqtt: zmqtt._internal.packets.Packet,
    request: pytest.FixtureRequest,
) -> None:
    """Test that mqtt5 writes the same bytes as zmqtt."""
    identifier = request.node.callspec.id
    if identifier in {
        "Connect(will)",
        "Connect(full)",
        "ConnAck(full)",
        "Disconnect(full)",
    }:
        # MQTT 5.0 does not mandate an order for properties within a properties
        # block; mqtt5 and zmqtt pick different orderings.
        pytest.xfail("Mismatch due to property order")

    data = packet.write()
    data_zmqtt = zmqtt._internal.packets.codec.encode(packet_zmqtt, version="5.0")
    assert len(data) == len(data_zmqtt)
    assert data == data_zmqtt
