"""Tests for the write/read (roundtrip) consistency of the implementation."""

import mqtt5
import pytest
import conftest


@pytest.mark.parametrize("packet", conftest.PACKETS, ids=conftest.PACKET_NAMES)
def test_roundtrip(packet, buffer):
    """Test write/read (roundtrip) consistency for all packet types."""
    data = packet.write()
    packet2, nbytes = mqtt5.read(bytearray(data))
    assert nbytes == len(data)
    assert isinstance(packet2, type(packet))
    assert packet == packet2
