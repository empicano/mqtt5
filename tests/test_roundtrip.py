"""Tests write/read (roundtrip) consistency."""

import conftest
import pytest

import mqtt5


@pytest.mark.parametrize("packet", conftest.PACKETS, ids=conftest.PACKET_NAMES)
def test_roundtrip(packet: mqtt5.Packet) -> None:
    """Test write/read (roundtrip) consistency."""
    data = packet.write()
    packet2, nbytes = mqtt5.read(bytearray(data))
    assert nbytes == len(data)
    assert isinstance(packet2, type(packet))
    assert packet == packet2
