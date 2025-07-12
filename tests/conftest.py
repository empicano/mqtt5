import pytest
import mqtt5
import mqttproto
import inspect


def connect_packet():
    return mqtt5.ConnectPacket(client_id="Bulbasaur")


def connect_packet_mqttproto():
    return mqttproto.MQTTConnectPacket(client_id="Bulbasaur")


def connack_packet():
    return mqtt5.ConnAckPacket()


def connack_packet_mqttproto():
    return mqttproto.MQTTConnAckPacket(
        session_present=False, reason_code=mqttproto.ReasonCode.SUCCESS
    )


def publish_packet():
    # TODO: Make payload empty
    return mqtt5.PublishPacket(topic="foo/bar/+", payload=b"\x12" * 2**8)


def publish_packet_mqttproto():
    # TODO: Make payload empty
    return mqttproto.MQTTPublishPacket(topic="foo/bar/+", payload=b"\x12" * 2**8)


def puback_packet():
    return mqtt5.PubAckPacket(packet_id=1234)


def puback_packet_mqttproto():
    return mqttproto.MQTTPublishAckPacket(
        packet_id=1234, reason_code=mqttproto.ReasonCode.SUCCESS
    )


def subscribe_packet():
    return mqtt5.SubscribePacket(
        packet_id=1234, subscriptions=[mqtt5.Subscription(pattern="foo/bar/+")]
    )


def subscribe_packet_mqttproto():
    return mqttproto.MQTTSubscribePacket(
        packet_id=1234, subscriptions=[mqttproto.Subscription(pattern="foo/bar/+")]
    )


def disconnect_packet():
    return mqtt5.DisconnectPacket()


def disconnect_packet_mqttproto():
    return mqttproto.MQTTDisconnectPacket(
        reason_code=mqttproto.ReasonCode.NORMAL_DISCONNECTION
    )


PACKET_NAMES = [
    name
    for name, _ in inspect.getmembers(mqtt5, inspect.isclass)
    if name.endswith("Packet")
]
PACKETS, PACKETS_MQTTPROTO = [], []
for name in PACKET_NAMES:
    PACKETS.append(locals()[name.lower()[:-6] + "_packet"]())
    PACKETS_MQTTPROTO.append(locals()[name.lower()[:-6] + "_packet_mqttproto"]())


@pytest.fixture(scope="session")
def buffer():
    """Pre-allocated buffer for packet de/serialization."""
    return bytearray(1024)
