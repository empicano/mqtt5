"""Tests error handling in edge cases."""

import re

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
    ("buffer", "message"),
    [
        pytest.param(
            b"\x00\x02\x00\x00",
            "Invalid PacketType value: 0",
            id="Invalid packet type",
        ),
        pytest.param(
            b"\x10\x0c\x00\x03\x53\x53\x48\x05\x00\x00\x00\x00\x00\x00",
            "Invalid protocol name",
            id="Connect: Invalid protocol name",
        ),
        pytest.param(
            b"\x10\x0d\x00\x04\x4d\x51\x54\x54\x04\x00\x00\x00\x00\x00\x00",
            "Invalid protocol version",
            id="Connect: Invalid protocol version",
        ),
        pytest.param(
            b"\x10\x10\x00\x04\x4d\x51\x54\x54\x05\x00\x00\x00\x03\x21\x00\x00\x00\x00",
            "Receive maximum must be != 0",
            id="Connect: Receive maximum == 0",
        ),
        pytest.param(
            b"\x10\x12\x00\x04\x4d\x51\x54\x54\x05\x00\x00\x00\x05\x27\x00\x00\x00\x00\x00\x00",
            "Maximum packet size must be != 0",
            id="Connect: Maximum packet size == 0",
        ),
        pytest.param(
            b"\x20\x83\x80\x80\x80\x00\x00\x00",
            "Invalid variable byte integer",
            id="ConnAck: VariableByteInteger 4th continuation bit",
        ),
        pytest.param(
            b"\x20\x83\x00\x00\x00\x00",
            "Invalid variable byte integer",
            id="ConnAck: VariableByteInteger unnecessary zero byte",
        ),
        pytest.param(
            b"\x20\x03\x00\x8b\x00",
            "Invalid ConnAckReasonCode value: 139",
            id="ConnAck: Invalid reason code",
        ),
        pytest.param(
            b"\x20\x06\x00\x00\x03\x23\xff\xff",
            "Invalid property type: TopicAlias",
            id="ConnAck: Invalid property",
        ),
        pytest.param(
            b"\x20\x02\x00\x00",
            "Missing property length",
            id="ConnAck: Missing property length",
        ),
        pytest.param(
            b"\x20\x06\x00\x00\x03\x21\x00\x00",
            "Receive maximum must be != 0",
            id="ConnAck: Receive maximum == 0",
        ),
        pytest.param(
            b"\x20\x08\x00\x00\x05\x27\x00\x00\x00\x00",
            "Maximum packet size must be != 0",
            id="ConnAck: Maximum packet size == 0",
        ),
        pytest.param(
            b"\x30\x02\x00\x00",
            "Missing property length",
            id="Publish: Missing property length",
        ),
        pytest.param(
            b"\x32\x03\x00\x00\x00",
            "Invalid remaining length",
            id="Publish: QoS=1 without packet id",
        ),
        pytest.param(
            b"\x34\x03\x00\x00\x00",
            "Invalid remaining length",
            id="Publish: QoS=2 without packet id",
        ),
        pytest.param(
            b"\x30\x07\x00\x01\x61\x03\x23\x00\x00",
            "Topic alias must be != 0",
            id="Publish: Topic alias == 0",
        ),
        pytest.param(
            b"\x30\x06\x00\x01\x61\x02\x0b\x00",
            "Subscription ID must be != 0",
            id="Publish: Subscription ID entry == 0",
        ),
        pytest.param(
            b"\x60\x04\xff\xff\x00\x00",
            "Invalid fixed header flags",
            id="PubRel: Invalid flags",
        ),
        pytest.param(
            b"\x72\x04\xff\xff\x00\x00",
            "Invalid fixed header flags",
            id="PubComp: Invalid flags",
        ),
        pytest.param(
            b"\x80\x06\xff\xff\x00\x00\x00\x00",
            "Invalid fixed header flags",
            id="Subscribe: Invalid flags",
        ),
        pytest.param(
            b"\x82\x09\x00\x01\x02\x0b\x00\x00\x01\x61\x00",
            "Subscription ID must be != 0",
            id="Subscribe: Subscription ID == 0",
        ),
        pytest.param(
            b"\xa0\x05\xff\xff\x00\x00\x00",
            "Invalid fixed header flags",
            id="Unsubscribe: Invalid flags",
        ),
        pytest.param(
            b"\x20\x04\x00\x00\x00\x00",
            "Invalid remaining length",
            id="ConnAck: Unconsumed remaining bytes",
        ),
        pytest.param(
            b"\x40\x01\xff\xff",
            "Invalid remaining length",
            id="PubAck: Remaining length value too small",
        ),
        pytest.param(
            b"\x30\x03\x00\x00\x00",
            "Topic alias must be set if topic is empty",
            id="Publish: Empty topic without topic alias",
        ),
    ],
)
def test_read_malformed_bytes(buffer: bytearray, message: str) -> None:
    """Test error from reading malformed bytes."""
    with pytest.raises(ValueError, match=f"^{re.escape(message)}$"):
        mqtt5.read(memoryview(buffer))


@pytest.mark.parametrize(
    ("cls", "args", "message"),
    [
        pytest.param(
            mqtt5.Will,
            {"topic": "foo/+/bar"},
            "Invalid topic",
            id="Will: Topic with single-level wildcard",
        ),
        pytest.param(
            mqtt5.Will,
            {"topic": "foo/#"},
            "Invalid topic",
            id="Will: Topic with multi-level wildcard",
        ),
        pytest.param(
            mqtt5.Will,
            {"topic": "foo/bar", "response_topic": "foo/+/bar"},
            "Invalid topic",
            id="Will: Response topic with single-level wildcard",
        ),
        pytest.param(
            mqtt5.Will,
            {"topic": "foo/bar", "response_topic": "foo/#"},
            "Invalid topic",
            id="Will: Response topic with multi-level wildcard",
        ),
        pytest.param(
            mqtt5.TopicFilter,
            {"pattern": "foo+/bar"},
            "Invalid topic filter",
            id="TopicFilter: Single-level wildcard not alone in level",
        ),
        pytest.param(
            mqtt5.TopicFilter,
            {"pattern": "foo/#/bar"},
            "Invalid topic filter",
            id="TopicFilter: Multi-level wildcard in the middle",
        ),
        pytest.param(
            mqtt5.ConnectPacket,
            {"client_id": "a" * 65536},
            "String must be < 65535 bytes",
            id="Connect: Client ID > 65535 bytes",
        ),
        pytest.param(
            mqtt5.ConnectPacket,
            {"client_id": "Bulbasaur", "password": b"a" * 65536},
            "Binary data must be < 65535 bytes",
            id="Connect: Password > 65535 bytes",
        ),
        pytest.param(
            mqtt5.ConnectPacket,
            {"client_id": "Bulbasaur", "receive_max": 0},
            "Receive maximum must be != 0",
            id="Connect: Receive maximum == 0",
        ),
        pytest.param(
            mqtt5.ConnectPacket,
            {"client_id": "Bulbasaur", "max_packet_size": 0},
            "Maximum packet size must be != 0",
            id="Connect: Maximum packet size == 0",
        ),
        pytest.param(
            mqtt5.ConnAckPacket,
            {"receive_max": 0},
            "Receive maximum must be != 0",
            id="ConnAck: Receive maximum == 0",
        ),
        pytest.param(
            mqtt5.ConnAckPacket,
            {"max_packet_size": 0},
            "Maximum packet size must be != 0",
            id="ConnAck: Maximum packet size == 0",
        ),
        pytest.param(
            mqtt5.PublishPacket,
            {"topic": "foo/bar/1234", "payload": b"", "packet_id": 1},
            "Packet ID must not be set for QoS=0",
            id="Publish: QoS=0 with packet id",
        ),
        pytest.param(
            mqtt5.PublishPacket,
            {"topic": "foo/bar/1234", "payload": b"", "qos": mqtt5.QoS.AT_LEAST_ONCE},
            "Packet ID must be set for QoS=1 and QoS=2",
            id="Publish: QoS=1 without packet id",
        ),
        pytest.param(
            mqtt5.PublishPacket,
            {"topic": "foo/bar/1234", "payload": b"", "qos": mqtt5.QoS.EXACTLY_ONCE},
            "Packet ID must be set for QoS=1 and QoS=2",
            id="Publish: QoS=2 without packet id",
        ),
        pytest.param(
            mqtt5.PublishPacket,
            {"topic": "", "payload": b""},
            "Topic alias must be set if topic is empty",
            id="Publish: Empty topic without topic alias",
        ),
        pytest.param(
            mqtt5.PublishPacket,
            {"topic": "a" * 65536, "payload": b""},
            "String must be < 65535 bytes",
            id="Publish: Topic > 65535 bytes",
        ),
        pytest.param(
            mqtt5.PublishPacket,
            {"topic": "foo/+/bar", "payload": b""},
            "Invalid topic",
            id="Publish: Topic with single-level wildcard",
        ),
        pytest.param(
            mqtt5.PublishPacket,
            {"topic": "foo/#", "payload": b""},
            "Invalid topic",
            id="Publish: Topic with multi-level wildcard",
        ),
        pytest.param(
            mqtt5.PublishPacket,
            {"topic": "foo/bar", "payload": b"", "response_topic": "foo/+/bar"},
            "Invalid topic",
            id="Publish: Response topic with single-level wildcard",
        ),
        pytest.param(
            mqtt5.PublishPacket,
            {"topic": "foo/bar", "payload": b"", "response_topic": "foo/#"},
            "Invalid topic",
            id="Publish: Response topic with multi-level wildcard",
        ),
        pytest.param(
            mqtt5.PublishPacket,
            {"topic": "foo", "payload": b"", "subscription_ids": [2**28]},
            "Variable byte integer must be < 2**28",
            id="Publish: Subscription ID entry >= 2**28",
        ),
        pytest.param(
            mqtt5.PublishPacket,
            {"topic": "foo", "payload": b"", "subscription_ids": [0]},
            "Subscription ID must be != 0",
            id="Publish: Subscription ID entry == 0",
        ),
        pytest.param(
            mqtt5.PublishPacket,
            {"topic": "foo", "payload": b"", "topic_alias": 0},
            "Topic alias must be != 0",
            id="Publish: Topic alias == 0",
        ),
        pytest.param(
            mqtt5.PubAckPacket,
            {"packet_id": 1, "user_properties": [("a" * 65536, "value")]},
            "String must be < 65535 bytes",
            id="PubAck: User property key > 65535 bytes",
        ),
        pytest.param(
            mqtt5.PubAckPacket,
            {"packet_id": 1, "user_properties": [("key", "a" * 65536)]},
            "String must be < 65535 bytes",
            id="PubAck: User property value > 65535 bytes",
        ),
        pytest.param(
            mqtt5.SubscribePacket,
            {"packet_id": 1, "topic_filters": []},
            "Topic filter list must contain at least one entry",
            id="Subscribe: Empty topic filter list",
        ),
        pytest.param(
            mqtt5.SubscribePacket,
            {
                "packet_id": 1,
                "topic_filters": [mqtt5.TopicFilter(pattern="+/bar/#")],
                "subscription_id": 0,
            },
            "Subscription ID must be != 0",
            id="Subscribe: Subscription ID == 0",
        ),
        pytest.param(
            mqtt5.SubscribePacket,
            {
                "packet_id": 1,
                "topic_filters": [mqtt5.TopicFilter(pattern="+/bar/#")],
                "subscription_id": 2**28,
            },
            "Variable byte integer must be < 2**28",
            id="Subscribe: Subscription ID >= 2**28",
        ),
        pytest.param(
            mqtt5.UnsubscribePacket,
            {"packet_id": 1, "patterns": []},
            "Pattern list must contain at least one entry",
            id="Unsubscribe: Empty pattern list",
        ),
        pytest.param(
            mqtt5.UnsubscribePacket,
            {"packet_id": 1, "patterns": ["a" * 65536]},
            "String must be < 65535 bytes",
            id="Unsubscribe: Pattern > 65535 bytes",
        ),
        pytest.param(
            mqtt5.UnsubscribePacket,
            {"packet_id": 1, "patterns": ["foo+/bar"]},
            "Invalid topic filter",
            id="Unsubscribe: Single-level wildcard not alone in level",
        ),
        pytest.param(
            mqtt5.UnsubscribePacket,
            {"packet_id": 1, "patterns": ["foo/#/bar"]},
            "Invalid topic filter",
            id="Unsubscribe: Multi-level wildcard in the middle",
        ),
    ],
)
def test_invalid_arguments(cls: type, args: dict, message: str) -> None:
    """Test error from initializing with invalid arguments."""
    with pytest.raises(ValueError, match=f"^{re.escape(message)}$"):
        cls(**args)
