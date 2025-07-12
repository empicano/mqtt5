"""Tests for the write/read (roundtrip) consistency of the implementation."""

import mqtt5
import pytest
import conftest


@pytest.mark.parametrize("packet", conftest.PACKETS, ids=conftest.PACKET_NAMES)
def test_roundtrip(packet, buffer):
    """Test write/read (roundtrip) consistency for all packet types."""
    n = packet.write(buffer)
    packet2, n2 = mqtt5.read(buffer)
    assert n == n2
    assert isinstance(packet2, type(packet))
    assert packet == packet2
