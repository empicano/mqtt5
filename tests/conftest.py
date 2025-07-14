import pytest
import mqtt5
import mqttproto


def connect_packet():
    return mqtt5.ConnectPacket(client_id="Bulbasaur")


def connect_packet_mqttproto():
    return mqttproto.MQTTConnectPacket(client_id="Bulbasaur")


def connect_packet_will():
    return mqtt5.ConnectPacket(
        client_id="Bulbasaur",
        will=mqtt5.Will(
            topic="foo/bar/+",
            payload=b"\x12" * 2**8,
            qos=mqtt5.QoS.EXACTLY_ONCE,
            retain=True,
            properties=mqtt5.WillProperties(
                payload_format_indicator=1,
                message_expiry_interval=2**24,
                content_type="text/html",
                response_topic="HELLO/4444/#",
                correlation_data=b"\x12" * 2**8,
                will_delay_interval=12,
            ),
        ),
    )


def connect_packet_will_mqttproto():
    return mqttproto.MQTTConnectPacket(
        client_id="Bulbasaur",
        will=mqttproto.Will(
            topic="foo/bar/+",
            payload=b"\x12" * 2**8,
            qos=mqttproto.QoS.EXACTLY_ONCE,
            retain=True,
            properties={
                mqttproto.PropertyType.PAYLOAD_FORMAT_INDICATOR: 1,
                mqttproto.PropertyType.MESSAGE_EXPIRY_INTERVAL: 2**24,
                mqttproto.PropertyType.CONTENT_TYPE: "text/html",
                mqttproto.PropertyType.RESPONSE_TOPIC: "HELLO/4444/#",
                mqttproto.PropertyType.CORRELATION_DATA: b"\x12" * 2**8,
                mqttproto.PropertyType.WILL_DELAY_INTERVAL: 12,
            },
        ),
    )


def connect_packet_full():
    return mqtt5.ConnectPacket(
        client_id="Bulbasaur",
        username="ProfOak",
        password="RazorLeaf?456",
        clean_start=True,
        will=mqtt5.Will(
            topic="foo/bar/+",
            payload=b"\x12" * 2**8,
            qos=mqtt5.QoS.EXACTLY_ONCE,
            retain=True,
            properties=mqtt5.WillProperties(
                payload_format_indicator=1,
                message_expiry_interval=2**24,
                content_type="text/html",
                response_topic="HELLO/4444/#",
                correlation_data=b"\x12" * 2**8,
                will_delay_interval=12,
            ),
        ),
        keep_alive=6789,
        properties=mqtt5.ConnectProperties(
            session_expiry_interval=9999,
            authentication_method="GS2-KRB5",
            authentication_data=b"\x12" * 2**8,
            request_problem_information=0,
            request_response_information=1,
            receive_maximum=55555,
            topic_alias_maximum=3,
            maximum_packet_size=5000,
        ),
    )


def connect_packet_full_mqttproto():
    return mqttproto.MQTTConnectPacket(
        client_id="Bulbasaur",
        username="ProfOak",
        password="RazorLeaf?456",
        clean_start=True,
        will=mqttproto.Will(
            topic="foo/bar/+",
            payload=b"\x12" * 2**8,
            qos=mqttproto.QoS.EXACTLY_ONCE,
            retain=True,
            properties={
                mqttproto.PropertyType.PAYLOAD_FORMAT_INDICATOR: 1,
                mqttproto.PropertyType.MESSAGE_EXPIRY_INTERVAL: 2**24,
                mqttproto.PropertyType.CONTENT_TYPE: "text/html",
                mqttproto.PropertyType.RESPONSE_TOPIC: "HELLO/4444/#",
                mqttproto.PropertyType.CORRELATION_DATA: b"\x12" * 2**8,
                mqttproto.PropertyType.WILL_DELAY_INTERVAL: 12,
            },
        ),
        keep_alive=6789,
        properties={
            mqttproto.PropertyType.SESSION_EXPIRY_INTERVAL: 9999,
            mqttproto.PropertyType.AUTHENTICATION_METHOD: "GS2-KRB5",
            mqttproto.PropertyType.AUTHENTICATION_DATA: b"\x12" * 2**8,
            mqttproto.PropertyType.REQUEST_PROBLEM_INFORMATION: 0,
            mqttproto.PropertyType.REQUEST_RESPONSE_INFORMATION: 1,
            mqttproto.PropertyType.RECEIVE_MAXIMUM: 55555,
            mqttproto.PropertyType.TOPIC_ALIAS_MAXIMUM: 3,
            mqttproto.PropertyType.MAXIMUM_PACKET_SIZE: 5000,
        },
    )


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
            session_expiry_interval=9999,
            assigned_client_id="Bulbasaur",
            server_keep_alive=6789,
            authentication_method="GS2-KRB5",
            authentication_data=b"\x12" * 2**8,
            response_information="response/information",
            server_reference="example.com:1883",
            reason_string="The reason string is a human readable string designed for diagnostics.",
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
            mqttproto.PropertyType.SESSION_EXPIRY_INTERVAL: 9999,
            mqttproto.PropertyType.ASSIGNED_CLIENT_IDENTIFIER: "Bulbasaur",
            mqttproto.PropertyType.SERVER_KEEP_ALIVE: 6789,
            mqttproto.PropertyType.AUTHENTICATION_METHOD: "GS2-KRB5",
            mqttproto.PropertyType.AUTHENTICATION_DATA: b"\x12" * 2**8,
            mqttproto.PropertyType.RESPONSE_INFORMATION: "response/information",
            mqttproto.PropertyType.SERVER_REFERENCE: "example.com:1883",
            mqttproto.PropertyType.REASON_STRING: "The reason string is a human readable string designed for diagnostics.",
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


def publish_packet_qos0():
    return mqtt5.PublishPacket(topic="foo/bar/+", payload=b"\x12" * 2**8)


def publish_packet_qos0_mqttproto():
    return mqttproto.MQTTPublishPacket(topic="foo/bar/+", payload=b"\x12" * 2**8)


def publish_packet_qos1():
    return mqtt5.PublishPacket(
        topic="foo/bar/+",
        payload=b"\x12" * 2**8,
        qos=mqtt5.QoS.AT_LEAST_ONCE,
        packet_id=1234,
    )


def publish_packet_qos1_mqttproto():
    return mqttproto.MQTTPublishPacket(
        topic="foo/bar/+",
        payload=b"\x12" * 2**8,
        qos=mqttproto.QoS.AT_LEAST_ONCE,
        packet_id=1234,
    )


def puback_packet():
    return mqtt5.PubAckPacket(packet_id=1234)


def puback_packet_mqttproto():
    return mqttproto.MQTTPublishAckPacket(
        packet_id=1234, reason_code=mqttproto.ReasonCode.SUCCESS
    )


def puback_packet_full():
    return mqtt5.PubAckPacket(
        packet_id=1234,
        reason_code=mqtt5.PubAckReasonCode.NO_MATCHING_SUBSCRIBERS,
        properties=mqtt5.PubAckProperties(
            reason_string="The reason string is a human readable string designed for diagnostics.",
        ),
    )


def puback_packet_full_mqttproto():
    return mqttproto.MQTTPublishAckPacket(
        packet_id=1234,
        reason_code=mqttproto.ReasonCode.NO_MATCHING_SUBSCRIBERS,
        properties={
            mqttproto.PropertyType.REASON_STRING: "The reason string is a human readable string designed for diagnostics.",
        },
    )


def subscribe_packet():
    return mqtt5.SubscribePacket(
        packet_id=1234, subscriptions=[mqtt5.Subscription(pattern="foo/bar/+")]
    )


def subscribe_packet_mqttproto():
    return mqttproto.MQTTSubscribePacket(
        packet_id=1234, subscriptions=[mqttproto.Subscription(pattern="foo/bar/+")]
    )


def suback_packet():
    return mqtt5.SubAckPacket(
        packet_id=1234, reason_codes=[mqtt5.SubAckReasonCode.TOPIC_FILTER_INVALID]
    )


def suback_packet_mqttproto():
    return mqttproto.MQTTSubscribeAckPacket(
        packet_id=1234, reason_codes=[mqttproto.ReasonCode.TOPIC_FILTER_INVALID]
    )


def disconnect_packet():
    return mqtt5.DisconnectPacket()


def disconnect_packet_mqttproto():
    return mqttproto.MQTTDisconnectPacket(
        reason_code=mqttproto.ReasonCode.NORMAL_DISCONNECTION
    )


def disconnect_packet_full():
    return mqtt5.DisconnectPacket(
        reason_code=mqtt5.DisconnectReasonCode.SERVER_SHUTTING_DOWN,
        properties=mqtt5.DisconnectProperties(
            session_expiry_interval=9999,
            server_reference="example.com:1883",
            reason_string="The reason string is a human readable string designed for diagnostics.",
        ),
    )


def disconnect_packet_full_mqttproto():
    return mqttproto.MQTTDisconnectPacket(
        reason_code=mqttproto.ReasonCode.SERVER_SHUTTING_DOWN,
        properties={
            mqttproto.PropertyType.SESSION_EXPIRY_INTERVAL: 9999,
            mqttproto.PropertyType.SERVER_REFERENCE: "example.com:1883",
            mqttproto.PropertyType.REASON_STRING: "The reason string is a human readable string designed for diagnostics.",
        },
    )


PACKET_NAMES, PACKET_INITS, PACKET_INITS_MQTTPROTO = [], [], []

for key, value in dict(locals()).items():
    tags = key.split("_")
    if len(tags) > 1 and tags[1] == "packet":
        if tags[-1] == "mqttproto":
            PACKET_INITS_MQTTPROTO.append(value)
            continue
        name = type(value()).__name__[:-6]
        if len(tags) > 2:
            name += f"({'_'.join(tags[2:])})"
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
