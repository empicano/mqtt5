"""Tests specification compliance by comparing mqtt5 and mqttproto outputs."""

import pytest
import conftest


@pytest.mark.parametrize(
    "packet,packet_mqttproto",
    zip(conftest.PACKETS, conftest.PACKETS_MQTTPROTO),
    ids=conftest.PACKET_NAMES,
)
def test_compliance(packet, packet_mqttproto, buffer):
    """Test that mqtt5 writes the same bytes as mqttproto for all packet types."""
    n = packet.write(buffer)
    buffer_mqttproto = bytearray()
    packet_mqttproto.encode(buffer_mqttproto)
    assert n == len(buffer_mqttproto)
    assert buffer[:n] == buffer_mqttproto
