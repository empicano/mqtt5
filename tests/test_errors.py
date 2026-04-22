"""Tests error handling in edge cases."""

import conftest
import pytest

import mqtt5


@pytest.mark.parametrize("packet", conftest.PACKETS, ids=conftest.PACKET_NAMES)
def test_read_incomplete_buffer(packet: mqtt5.Packet) -> None:
    """Test error from reading an incomplete buffer."""
    buffer = memoryview(packet.write())
    for index in range(len(buffer)):
        with pytest.raises(IndexError):
            mqtt5.read(buffer[:index])


@pytest.mark.parametrize(
    "buffer",
    [
        pytest.param(
            b"\x00\x02\x00\x00",
            id="Invalid packet type",
        ),
        pytest.param(
            b"\x10\x0d\x00\x03\x53\x53\x48\x05\x00\x00\x00\x00\x00\x01\x31",
            id="Connect: Invalid protocol name",
        ),
        pytest.param(
            b"\x10\x0e\x00\x04\x4d\x51\x54\x54\x04\x00\x00\x00\x00\x00\x01\x31",
            id="Connect: Invalid protocol version",
        ),
        pytest.param(
            b"\x20\x83\x80\x80\x80\x00\x00\x00",
            id="ConnAck: VariableByteInteger 4th continuation bit",
        ),
        pytest.param(
            b"\x20\x83\x00\x00\x00\x00",
            id="ConnAck: VariableByteInteger unnecessary zero byte",
        ),
        pytest.param(
            b"\x20\x03\x00\x8b\x00",
            id="ConnAck: Invalid reason code",
        ),
        pytest.param(
            b"\x20\x06\x00\x00\x03\x23\xff\xff",
            id="ConnAck: Invalid property",
        ),
        pytest.param(
            b"\x20\x02\x00\x00",
            id="ConnAck: Missing property length",
        ),
        pytest.param(
            b"\x30\x02\x00\x00",
            id="Publish: Missing property length",
        ),
        pytest.param(
            b"\x32\x03\x00\x00\x00",
            id="Publish: QoS=1 without packet id",
        ),
        pytest.param(
            b"\x34\x03\x00\x00\x00",
            id="Publish: QoS=2 without packet id",
        ),
        pytest.param(
            b"\x60\x04\xff\xff\x00\x00",
            id="PubRel: Invalid flags",
        ),
        pytest.param(
            b"\x72\x04\xff\xff\x00\x00",
            id="PubComp: Invalid flags",
        ),
        pytest.param(
            b"\x80\x06\xff\xff\x00\x00\x00\x00",
            id="Subscribe: Invalid flags",
        ),
        pytest.param(
            b"\xa0\x05\xff\xff\x00\x00\x00",
            id="Unsubscribe: Invalid flags",
        ),
        pytest.param(
            b"\x20\x04\x00\x00\x00\x00",
            id="ConnAck: Unconsumed remaining bytes",
        ),
        pytest.param(
            b"\x40\x01\xff\xff",
            id="PubAck: Remaining length value too small",
        ),
        pytest.param(
            b"\x30\x03\x00\x00\x00",
            id="Publish: Empty topic without topic alias",
        ),
    ],
)
def test_read_malformed_packet(buffer: bytearray) -> None:
    """Test error from reading a malformed packet."""
    with pytest.raises(ValueError):  # noqa: PT011
        mqtt5.read(memoryview(buffer))


@pytest.mark.parametrize(
    ("packet_type", "args"),
    [
        pytest.param(
            mqtt5.ConnectPacket,
            {"client_id": "a" * 65536},
            id="Connect: Client ID > 65535 bytes",
        ),
        pytest.param(
            mqtt5.ConnectPacket,
            {"client_id": "Bulbasaur", "password": b"a" * 65536},
            id="Connect: Password > 65535 bytes",
        ),
        pytest.param(
            mqtt5.PublishPacket,
            {"topic": "foo/bar/1234", "payload": b"", "packet_id": 1},
            id="Publish: QoS=0 with packet id",
        ),
        pytest.param(
            mqtt5.PublishPacket,
            {"topic": "foo/bar/1234", "payload": b"", "qos": mqtt5.QoS.AT_LEAST_ONCE},
            id="Publish: QoS=1 without packet id",
        ),
        pytest.param(
            mqtt5.PublishPacket,
            {"topic": "foo/bar/1234", "payload": b"", "qos": mqtt5.QoS.EXACTLY_ONCE},
            id="Publish: QoS=2 without packet id",
        ),
        pytest.param(
            mqtt5.PublishPacket,
            {"topic": "", "payload": b""},
            id="Publish: Empty topic without topic alias",
        ),
        pytest.param(
            mqtt5.PublishPacket,
            {"topic": "", "payload": b"", "topic_alias": 0},
            id="Publish: Empty topic with topic alias equals zero",
        ),
        pytest.param(
            mqtt5.PublishPacket,
            {"topic": "a" * 65536, "payload": b""},
            id="Publish: Topic > 65535 bytes",
        ),
        pytest.param(
            mqtt5.PublishPacket,
            {"topic": "foo", "payload": b"", "subscription_ids": [2**28]},
            id="Publish: Subscription ID entry >= 2**28",
        ),
        pytest.param(
            mqtt5.PubAckPacket,
            {"packet_id": 1, "user_properties": [("a" * 65536, "value")]},
            id="PubAck: User property key > 65535 bytes",
        ),
        pytest.param(
            mqtt5.PubAckPacket,
            {"packet_id": 1, "user_properties": [("key", "a" * 65536)]},
            id="PubAck: User property value > 65535 bytes",
        ),
        pytest.param(
            mqtt5.SubscribePacket,
            {"packet_id": 1, "topic_filters": []},
            id="Subscribe: Empty topic filter list",
        ),
        pytest.param(
            mqtt5.SubscribePacket,
            {
                "packet_id": 1,
                "topic_filters": [mqtt5.TopicFilter(pattern="+/bar/#")],
                "subscription_id": 2**28,
            },
            id="Subscribe: Subscription ID >= 2**28",
        ),
        pytest.param(
            mqtt5.UnsubscribePacket,
            {"packet_id": 1, "patterns": []},
            id="Unsubscribe: Empty pattern list",
        ),
        pytest.param(
            mqtt5.UnsubscribePacket,
            {"packet_id": 1, "patterns": ["a" * 65536]},
            id="Unsubscribe: Pattern > 65535 bytes",
        ),
    ],
)
def test_write_invalid_arguments(packet_type: type[mqtt5.Packet], args: dict) -> None:
    """Test error from writing with invalid arguments."""
    with pytest.raises(ValueError):  # noqa: PT011
        packet_type(**args)
