"""Tests for the write/read (roundtrip) consistency of the implementation."""

import pytest
import mqtt5


@pytest.mark.parametrize("packet_fixture", [
    "connect_packet",
    "connack_packet",
    "publish_packet",
    "puback_packet",
    "disconnect_packet",
])
def test_roundtrip(request, packet_fixture, buffer):
    """Test write/read (roundtrip) consistency for all packet types."""
    packet = request.getfixturevalue(packet_fixture)
    n = packet.write(buffer)
    packet2, n2 = mqtt5.read(buffer)
    assert n == n2
    assert isinstance(packet2, type(packet))
    assert packet == packet2
