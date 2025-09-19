"""Tests specification compliance by comparing mqtt5 and mqttproto outputs."""

import conftest
import mqttproto
import pytest

import mqtt5


@pytest.mark.parametrize(
    ("packet", "packet_mqttproto"),
    zip(conftest.PACKETS, conftest.PACKETS_MQTTPROTO, strict=True),
    ids=conftest.PACKET_NAMES,
)
def test_compliance(
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
