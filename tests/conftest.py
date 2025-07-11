import pytest
import mqtt5
import mqttproto
import inspect


_PACKETS = [
    name
    for name, _ in inspect.getmembers(mqtt5, inspect.isclass)
    if name.endswith("Packet")
]


@pytest.fixture(
    scope="session",
    params=[name.lower()[:-6] + "_packet" for name in _PACKETS],
    ids=_PACKETS,
)
def packet_fixture(request):
    return request.param


@pytest.fixture(scope="session")
def buffer():
    """Pre-allocated buffer for packet de/serialization."""
    return bytearray(1024)


@pytest.fixture(scope="session")
def connect_packet():
    return mqtt5.ConnectPacket(client_id="Bulbasaur")


@pytest.fixture(scope="session")
def connect_packet_mqttproto():
    return mqttproto.MQTTConnectPacket(client_id="Bulbasaur")


@pytest.fixture(scope="session")
def connack_packet():
    return mqtt5.ConnAckPacket()


@pytest.fixture(scope="session")
def connack_packet_mqttproto():
    return mqttproto.MQTTConnAckPacket(
        session_present=False, reason_code=mqttproto.ReasonCode.SUCCESS
    )


@pytest.fixture(scope="session")
def publish_packet():
    # TODO: Make payload empty
    return mqtt5.PublishPacket(topic="foo/bar/+", payload=b"\x12" * 2**8)


@pytest.fixture(scope="session")
def publish_packet_mqttproto():
    # TODO: Make payload empty
    return mqttproto.MQTTPublishPacket(topic="foo/bar/+", payload=b"\x12" * 2**8)


@pytest.fixture(scope="session")
def puback_packet():
    return mqtt5.PubAckPacket(packet_id=1234)


@pytest.fixture(scope="session")
def puback_packet_mqttproto():
    return mqttproto.MQTTPublishAckPacket(
        packet_id=1234, reason_code=mqttproto.ReasonCode.SUCCESS
    )


@pytest.fixture(scope="session")
def subscribe_packet():
    return mqtt5.SubscribePacket(
        packet_id=1234, subscriptions=[mqtt5.Subscription(pattern="foo/bar/+")]
    )


@pytest.fixture(scope="session")
def subscribe_packet_mqttproto():
    return mqttproto.MQTTSubscribePacket(
        packet_id=1234, subscriptions=[mqttproto.Subscription(pattern="foo/bar/+")]
    )


@pytest.fixture(scope="session")
def disconnect_packet():
    return mqtt5.DisconnectPacket()


@pytest.fixture(scope="session")
def disconnect_packet_mqttproto():
    return mqttproto.MQTTDisconnectPacket(
        reason_code=mqttproto.ReasonCode.NORMAL_DISCONNECTION
    )
