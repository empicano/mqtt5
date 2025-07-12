import pytest
import mqtt5
import mqttproto


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


def connack_packet_full():
    return mqtt5.ConnAckPacket(
        session_present=True,
        reason_code=mqtt5.ConnAckReasonCode.UNSPECIFIED_ERROR,
        properties=mqtt5.ConnAckProperties(
            session_expiry_interval=2**8,
            assigned_client_id="Bulbasaur",
            server_keep_alive=2**12,
            authentication_method="GS2-KRB5",
            authentication_data=b"\x12" * 2**8,
            response_information="response/information",
            server_reference="example.com:1883",
            reason_string="The reason string is a human readable string designed for diagnostics",
            receive_maximum=2**10,
            topic_alias_maximum=2**8,
            maximum_qos=0,
            retain_available=1,
            maximum_packet_size=2**12,
            wildcard_subscription_available=0,
            subscription_id_available=1,
            shared_subscription_available=0,
        ),
    )


def connack_packet_full_mqttproto():
    return mqttproto.MQTTConnAckPacket(
        session_present=True,
        reason_code=mqttproto.ReasonCode.UNSPECIFIED_ERROR,
        properties={
            mqttproto.PropertyType.SESSION_EXPIRY_INTERVAL: 2**8,
            mqttproto.PropertyType.ASSIGNED_CLIENT_IDENTIFIER: "Bulbasaur",
            mqttproto.PropertyType.SERVER_KEEP_ALIVE: 2**12,
            mqttproto.PropertyType.AUTHENTICATION_METHOD: "GS2-KRB5",
            mqttproto.PropertyType.AUTHENTICATION_DATA: b"\x12" * 2**8,
            mqttproto.PropertyType.RESPONSE_INFORMATION: "response/information",
            mqttproto.PropertyType.SERVER_REFERENCE: "example.com:1883",
            mqttproto.PropertyType.REASON_STRING: "The reason string is a human readable string designed for diagnostics",
            mqttproto.PropertyType.RECEIVE_MAXIMUM: 2**10,
            mqttproto.PropertyType.TOPIC_ALIAS_MAXIMUM: 2**8,
            mqttproto.PropertyType.MAXIMUM_QOS: 0,
            mqttproto.PropertyType.RETAIN_AVAILABLE: 1,
            mqttproto.PropertyType.MAXIMUM_PACKET_SIZE: 2**12,
            mqttproto.PropertyType.WILDCARD_SUBSCRIPTION_AVAILABLE: 0,
            mqttproto.PropertyType.SUBSCRIPTION_IDENTIFIER_AVAILABLE: 1,
            mqttproto.PropertyType.SHARED_SUBSCRIPTION_AVAILABLE: 0,
        },
    )


# TODO: Make payload empty
def publish_packet():
    return mqtt5.PublishPacket(topic="foo/bar/+", payload=b"\x12" * 2**8)


# TODO: Make payload empty
def publish_packet_mqttproto():
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


PACKET_NAMES, PACKET_INITS, PACKET_INITS_MQTTPROTO = [], [], []

for key, value in dict(locals()).items():
    tags = key.split("_")
    if len(tags) > 1 and tags[1] == "packet":
        if tags[-1] == "mqttproto":
            PACKET_INITS_MQTTPROTO.append(value)
            continue
        name = type(value()).__name__
        for t in tags[2:]:
            name += f",{t}"
        PACKET_NAMES.append(name)
        PACKET_INITS.append(value)

# Validate that we have both mqtt5 and mqttproto implementations for all test packets
assert len(PACKET_INITS) == len(PACKET_INITS_MQTTPROTO)
# Collect the initialized packets
PACKETS = [f() for f in PACKET_INITS]
PACKETS_MQTTPROTO = [f() for f in PACKET_INITS_MQTTPROTO]


@pytest.fixture(scope="session")
def buffer():
    """Pre-allocated buffer for packet de/serialization."""
    return bytearray(1024)
