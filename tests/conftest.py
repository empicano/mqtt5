"""Contains test configurations and packet definitions."""

import mqttproto

import mqtt5


def connect_packet() -> mqtt5.ConnectPacket:  # noqa: D103
    return mqtt5.ConnectPacket(client_id="Bulbasaur")


def connect_packet_mqttproto() -> mqttproto.MQTTConnectPacket:  # noqa: D103
    return mqttproto.MQTTConnectPacket(client_id="Bulbasaur")


def connect_packet_will() -> mqtt5.ConnectPacket:  # noqa: D103
    return mqtt5.ConnectPacket(
        client_id="Bulbasaur",
        will=mqtt5.Will(
            topic="foo/bar/1234",
            payload=b"\x12" * 2**8,
            qos=mqtt5.QoS.EXACTLY_ONCE,
            retain=True,
            payload_format_indicator=1,
            message_expiry_interval=2**24,
            content_type="text/html",
            response_topic="foo/bar/1234",
            correlation_data=b"\x12" * 2**8,
            will_delay_interval=20,
        ),
    )


def connect_packet_will_mqttproto() -> mqttproto.MQTTConnectPacket:  # noqa: D103
    return mqttproto.MQTTConnectPacket(
        client_id="Bulbasaur",
        will=mqttproto.Will(
            topic="foo/bar/1234",
            payload=b"\x12" * 2**8,
            qos=mqttproto.QoS.EXACTLY_ONCE,
            retain=True,
            properties={
                mqttproto.PropertyType.PAYLOAD_FORMAT_INDICATOR: 1,
                mqttproto.PropertyType.MESSAGE_EXPIRY_INTERVAL: 2**24,
                mqttproto.PropertyType.CONTENT_TYPE: "text/html",
                mqttproto.PropertyType.RESPONSE_TOPIC: "foo/bar/1234",
                mqttproto.PropertyType.CORRELATION_DATA: b"\x12" * 2**8,
                mqttproto.PropertyType.WILL_DELAY_INTERVAL: 20,
            },
        ),
    )


def connect_packet_full() -> mqtt5.ConnectPacket:  # noqa: D103
    return mqtt5.ConnectPacket(
        client_id="Bulbasaur",
        username="ProfOak",
        password="RazorLeaf?456",
        clean_start=True,
        will=mqtt5.Will(
            topic="foo/bar/1234",
            payload=b"\x12" * 2**8,
            qos=mqtt5.QoS.EXACTLY_ONCE,
            retain=True,
            payload_format_indicator=1,
            message_expiry_interval=2**24,
            content_type="text/html",
            response_topic="foo/bar/1234",
            correlation_data=b"\x12" * 2**8,
            will_delay_interval=20,
            user_properties=[
                ("location", "Pallet Town"),
                ("type", "Grass"),
            ],
        ),
        keep_alive=6789,
        session_expiry_interval=600,
        authentication_method="GS2-KRB5",
        authentication_data=b"\x12" * 2**8,
        request_problem_info=False,
        request_response_info=True,
        receive_max=55555,
        topic_alias_max=3,
        max_packet_size=2**12,
        user_properties=[
            ("location", "Pallet Town"),
            ("type", "Grass"),
        ],
    )


def connect_packet_full_mqttproto() -> mqttproto.MQTTConnectPacket:  # noqa: D103
    return mqttproto.MQTTConnectPacket(
        client_id="Bulbasaur",
        username="ProfOak",
        password="RazorLeaf?456",
        clean_start=True,
        will=mqttproto.Will(
            topic="foo/bar/1234",
            payload=b"\x12" * 2**8,
            qos=mqttproto.QoS.EXACTLY_ONCE,
            retain=True,
            properties={
                mqttproto.PropertyType.PAYLOAD_FORMAT_INDICATOR: 1,
                mqttproto.PropertyType.MESSAGE_EXPIRY_INTERVAL: 2**24,
                mqttproto.PropertyType.CONTENT_TYPE: "text/html",
                mqttproto.PropertyType.RESPONSE_TOPIC: "foo/bar/1234",
                mqttproto.PropertyType.CORRELATION_DATA: b"\x12" * 2**8,
                mqttproto.PropertyType.WILL_DELAY_INTERVAL: 20,
            },
            user_properties={"location": "Pallet Town", "type": "Grass"},
        ),
        keep_alive=6789,
        properties={
            mqttproto.PropertyType.SESSION_EXPIRY_INTERVAL: 600,
            mqttproto.PropertyType.AUTHENTICATION_METHOD: "GS2-KRB5",
            mqttproto.PropertyType.AUTHENTICATION_DATA: b"\x12" * 2**8,
            mqttproto.PropertyType.REQUEST_PROBLEM_INFORMATION: 0,
            mqttproto.PropertyType.REQUEST_RESPONSE_INFORMATION: 1,
            mqttproto.PropertyType.RECEIVE_MAXIMUM: 55555,
            mqttproto.PropertyType.TOPIC_ALIAS_MAXIMUM: 3,
            mqttproto.PropertyType.MAXIMUM_PACKET_SIZE: 2**12,
        },
        user_properties={"location": "Pallet Town", "type": "Grass"},
    )


def connack_packet() -> mqtt5.ConnAckPacket:  # noqa: D103
    return mqtt5.ConnAckPacket()


def connack_packet_mqttproto() -> mqttproto.MQTTConnAckPacket:  # noqa: D103
    return mqttproto.MQTTConnAckPacket(
        session_present=False, reason_code=mqttproto.ReasonCode.SUCCESS
    )


def connack_packet_full() -> mqtt5.ConnAckPacket:  # noqa: D103
    return mqtt5.ConnAckPacket(
        session_present=True,
        reason_code=mqtt5.ConnAckReasonCode.UNSPECIFIED_ERROR,
        session_expiry_interval=600,
        assigned_client_id="Bulbasaur",
        server_keep_alive=6789,
        authentication_method="GS2-KRB5",
        authentication_data=b"\x12" * 2**8,
        response_info="response/information",
        server_reference="example.com:1883",
        reason_str="The reason string is a human readable string designed for diagnostics.",  # noqa: E501
        receive_max=2**10,
        topic_alias_max=2**8,
        max_qos=mqtt5.QoS.AT_MOST_ONCE,
        retain_available=False,
        max_packet_size=2**12,
        wildcard_subscription_available=False,
        subscription_id_available=False,
        shared_subscription_available=False,
        user_properties=[
            ("location", "Pallet Town"),
            ("type", "Grass"),
        ],
    )


def connack_packet_full_mqttproto() -> mqttproto.MQTTConnAckPacket:  # noqa: D103
    return mqttproto.MQTTConnAckPacket(
        session_present=True,
        reason_code=mqttproto.ReasonCode.UNSPECIFIED_ERROR,
        properties={
            mqttproto.PropertyType.SESSION_EXPIRY_INTERVAL: 600,
            mqttproto.PropertyType.ASSIGNED_CLIENT_IDENTIFIER: "Bulbasaur",
            mqttproto.PropertyType.SERVER_KEEP_ALIVE: 6789,
            mqttproto.PropertyType.AUTHENTICATION_METHOD: "GS2-KRB5",
            mqttproto.PropertyType.AUTHENTICATION_DATA: b"\x12" * 2**8,
            mqttproto.PropertyType.RESPONSE_INFORMATION: "response/information",
            mqttproto.PropertyType.SERVER_REFERENCE: "example.com:1883",
            mqttproto.PropertyType.REASON_STRING: "The reason string is a human readable string designed for diagnostics.",  # noqa: E501
            mqttproto.PropertyType.RECEIVE_MAXIMUM: 2**10,
            mqttproto.PropertyType.TOPIC_ALIAS_MAXIMUM: 2**8,
            mqttproto.PropertyType.MAXIMUM_QOS: 0,
            mqttproto.PropertyType.RETAIN_AVAILABLE: 0,
            mqttproto.PropertyType.MAXIMUM_PACKET_SIZE: 2**12,
            mqttproto.PropertyType.WILDCARD_SUBSCRIPTION_AVAILABLE: 0,
            mqttproto.PropertyType.SUBSCRIPTION_IDENTIFIER_AVAILABLE: 0,
            mqttproto.PropertyType.SHARED_SUBSCRIPTION_AVAILABLE: 0,
        },
        user_properties={"location": "Pallet Town", "type": "Grass"},
    )


def publish_packet_qos0() -> mqtt5.PublishPacket:  # noqa: D103
    return mqtt5.PublishPacket(topic="foo/bar/1234", payload=b"\x12" * 2**8)


def publish_packet_qos0_mqttproto() -> mqttproto.MQTTPublishPacket:  # noqa: D103
    return mqttproto.MQTTPublishPacket(topic="foo/bar/1234", payload=b"\x12" * 2**8)


def publish_packet_qos1() -> mqtt5.PublishPacket:  # noqa: D103
    return mqtt5.PublishPacket(
        topic="foo/bar/1234",
        payload=b"\x12" * 2**8,
        qos=mqtt5.QoS.AT_LEAST_ONCE,
        packet_id=999,
    )


def publish_packet_qos1_mqttproto() -> mqttproto.MQTTPublishPacket:  # noqa: D103
    return mqttproto.MQTTPublishPacket(
        topic="foo/bar/1234",
        payload=b"\x12" * 2**8,
        qos=mqttproto.QoS.AT_LEAST_ONCE,
        packet_id=999,
    )


def puback_packet() -> mqtt5.PubAckPacket:  # noqa: D103
    return mqtt5.PubAckPacket(packet_id=999)


def puback_packet_mqttproto() -> mqttproto.MQTTPublishAckPacket:  # noqa: D103
    return mqttproto.MQTTPublishAckPacket(
        packet_id=999, reason_code=mqttproto.ReasonCode.SUCCESS
    )


def puback_packet_full() -> mqtt5.PubAckPacket:  # noqa: D103
    return mqtt5.PubAckPacket(
        packet_id=999,
        reason_code=mqtt5.PubAckReasonCode.NO_MATCHING_SUBSCRIBERS,
        reason_str="The reason string is a human readable string designed for diagnostics.",  # noqa: E501
    )


def puback_packet_full_mqttproto() -> mqttproto.MQTTPublishAckPacket:  # noqa: D103
    return mqttproto.MQTTPublishAckPacket(
        packet_id=999,
        reason_code=mqttproto.ReasonCode.NO_MATCHING_SUBSCRIBERS,
        properties={
            mqttproto.PropertyType.REASON_STRING: "The reason string is a human readable string designed for diagnostics.",  # noqa: E501
        },
    )


def pubrec_packet() -> mqtt5.PubRecPacket:  # noqa: D103
    return mqtt5.PubRecPacket(packet_id=999)


def pubrec_packet_mqttproto() -> mqttproto.MQTTPublishReceivePacket:  # noqa: D103
    return mqttproto.MQTTPublishReceivePacket(
        packet_id=999, reason_code=mqttproto.ReasonCode.SUCCESS
    )


def pubrec_packet_full() -> mqtt5.PubRecPacket:  # noqa: D103
    return mqtt5.PubRecPacket(
        packet_id=999,
        reason_code=mqtt5.PubRecReasonCode.NO_MATCHING_SUBSCRIBERS,
        reason_str="The reason string is a human readable string designed for diagnostics.",  # noqa: E501
        user_properties=[
            ("location", "Pallet Town"),
            ("type", "Grass"),
        ],
    )


def pubrec_packet_full_mqttproto() -> mqttproto.MQTTPublishReceivePacket:  # noqa: D103
    return mqttproto.MQTTPublishReceivePacket(
        packet_id=999,
        reason_code=mqttproto.ReasonCode.NO_MATCHING_SUBSCRIBERS,
        properties={
            mqttproto.PropertyType.REASON_STRING: "The reason string is a human readable string designed for diagnostics.",  # noqa: E501
        },
        user_properties={"location": "Pallet Town", "type": "Grass"},
    )


def pubrel_packet() -> mqtt5.PubRelPacket:  # noqa: D103
    return mqtt5.PubRelPacket(packet_id=999)


def pubrel_packet_mqttproto() -> mqttproto.MQTTPublishReleasePacket:  # noqa: D103
    return mqttproto.MQTTPublishReleasePacket(
        packet_id=999, reason_code=mqttproto.ReasonCode.SUCCESS
    )


def pubrel_packet_full() -> mqtt5.PubRelPacket:  # noqa: D103
    return mqtt5.PubRelPacket(
        packet_id=999,
        reason_code=mqtt5.PubRelReasonCode.PACKET_ID_NOT_FOUND,
        reason_str="The reason string is a human readable string designed for diagnostics.",  # noqa: E501
        user_properties=[
            ("location", "Pallet Town"),
            ("type", "Grass"),
        ],
    )


def pubrel_packet_full_mqttproto() -> mqttproto.MQTTPublishReleasePacket:  # noqa: D103
    return mqttproto.MQTTPublishReleasePacket(
        packet_id=999,
        reason_code=mqttproto.ReasonCode.PACKET_IDENTIFIER_NOT_FOUND,
        properties={
            mqttproto.PropertyType.REASON_STRING: "The reason string is a human readable string designed for diagnostics.",  # noqa: E501
        },
        user_properties={"location": "Pallet Town", "type": "Grass"},
    )


def pubcomp_packet() -> mqtt5.PubCompPacket:  # noqa: D103
    return mqtt5.PubCompPacket(packet_id=999)


def pubcomp_packet_mqttproto() -> mqttproto.MQTTPublishCompletePacket:  # noqa: D103
    return mqttproto.MQTTPublishCompletePacket(
        packet_id=999, reason_code=mqttproto.ReasonCode.SUCCESS
    )


def pubcomp_packet_full() -> mqtt5.PubCompPacket:  # noqa: D103
    return mqtt5.PubCompPacket(
        packet_id=999,
        reason_code=mqtt5.PubCompReasonCode.PACKET_ID_NOT_FOUND,
        reason_str="The reason string is a human readable string designed for diagnostics.",  # noqa: E501
        user_properties=[
            ("location", "Pallet Town"),
            ("type", "Grass"),
        ],
    )


def pubcomp_packet_full_mqttproto() -> mqttproto.MQTTPublishCompletePacket:  # noqa: D103
    return mqttproto.MQTTPublishCompletePacket(
        packet_id=999,
        reason_code=mqttproto.ReasonCode.PACKET_IDENTIFIER_NOT_FOUND,
        properties={
            mqttproto.PropertyType.REASON_STRING: "The reason string is a human readable string designed for diagnostics.",  # noqa: E501
        },
        user_properties={"location": "Pallet Town", "type": "Grass"},
    )


def subscribe_packet() -> mqtt5.SubscribePacket:  # noqa: D103
    return mqtt5.SubscribePacket(
        packet_id=999, subscriptions=[mqtt5.Subscription(pattern="+/bar/#")]
    )


def subscribe_packet_mqttproto() -> mqttproto.MQTTSubscribePacket:  # noqa: D103
    return mqttproto.MQTTSubscribePacket(
        packet_id=999, subscriptions=[mqttproto.Subscription(pattern="+/bar/#")]
    )


def suback_packet() -> mqtt5.SubAckPacket:  # noqa: D103
    return mqtt5.SubAckPacket(
        packet_id=999, reason_codes=[mqtt5.SubAckReasonCode.TOPIC_FILTER_INVALID]
    )


def suback_packet_mqttproto() -> mqttproto.MQTTSubscribeAckPacket:  # noqa: D103
    return mqttproto.MQTTSubscribeAckPacket(
        packet_id=999, reason_codes=[mqttproto.ReasonCode.TOPIC_FILTER_INVALID]
    )


def unsubscribe_packet() -> mqtt5.UnsubscribePacket:  # noqa: D103
    return mqtt5.UnsubscribePacket(packet_id=999, patterns=["+/bar/#", "foo/#"])


def unsubscribe_packet_mqttproto() -> mqttproto.MQTTUnsubscribePacket:  # noqa: D103
    return mqttproto.MQTTUnsubscribePacket(packet_id=999, patterns=["+/bar/#", "foo/#"])


def unsuback_packet() -> mqtt5.UnsubAckPacket:  # noqa: D103
    return mqtt5.UnsubAckPacket(
        packet_id=999,
        reason_codes=[mqtt5.UnsubAckReasonCode.TOPIC_FILTER_INVALID],
        reason_str="The reason string is a human readable string designed for diagnostics.",  # noqa: E501
        user_properties=[
            ("location", "Pallet Town"),
            ("type", "Grass"),
        ],
    )


def unsuback_packet_mqttproto() -> mqttproto.MQTTUnsubscribeAckPacket:  # noqa: D103
    return mqttproto.MQTTUnsubscribeAckPacket(
        packet_id=999,
        reason_codes=[mqttproto.ReasonCode.TOPIC_FILTER_INVALID],
        properties={
            mqttproto.PropertyType.REASON_STRING: "The reason string is a human readable string designed for diagnostics.",  # noqa: E501
        },
        user_properties={"location": "Pallet Town", "type": "Grass"},
    )


def pingreq_packet() -> mqtt5.PingReqPacket:  # noqa: D103
    return mqtt5.PingReqPacket()


def pingreq_packet_mqttproto() -> mqttproto.MQTTPingRequestPacket:  # noqa: D103
    return mqttproto.MQTTPingRequestPacket()


def pingresp_packet() -> mqtt5.PingRespPacket:  # noqa: D103
    return mqtt5.PingRespPacket()


def pingresp_packet_mqttproto() -> mqttproto.MQTTPingResponsePacket:  # noqa: D103
    return mqttproto.MQTTPingResponsePacket()


def disconnect_packet() -> mqtt5.DisconnectPacket:  # noqa: D103
    return mqtt5.DisconnectPacket()


def disconnect_packet_mqttproto() -> mqttproto.MQTTDisconnectPacket:  # noqa: D103
    return mqttproto.MQTTDisconnectPacket(
        reason_code=mqttproto.ReasonCode.NORMAL_DISCONNECTION
    )


def disconnect_packet_full() -> mqtt5.DisconnectPacket:  # noqa: D103
    return mqtt5.DisconnectPacket(
        reason_code=mqtt5.DisconnectReasonCode.SERVER_SHUTTING_DOWN,
        session_expiry_interval=600,
        server_reference="example.com:1883",
        reason_str="The reason string is a human readable string designed for diagnostics.",  # noqa: E501
        user_properties=[
            ("location", "Pallet Town"),
            ("type", "Grass"),
        ],
    )


def disconnect_packet_full_mqttproto() -> mqttproto.MQTTDisconnectPacket:  # noqa: D103
    return mqttproto.MQTTDisconnectPacket(
        reason_code=mqttproto.ReasonCode.SERVER_SHUTTING_DOWN,
        properties={
            mqttproto.PropertyType.SESSION_EXPIRY_INTERVAL: 600,
            mqttproto.PropertyType.SERVER_REFERENCE: "example.com:1883",
            mqttproto.PropertyType.REASON_STRING: "The reason string is a human readable string designed for diagnostics.",  # noqa: E501
        },
        user_properties={"location": "Pallet Town", "type": "Grass"},
    )


def auth_packet() -> mqtt5.AuthPacket:  # noqa: D103
    return mqtt5.AuthPacket()


def auth_packet_mqttproto() -> mqttproto.MQTTAuthPacket:  # noqa: D103
    return mqttproto.MQTTAuthPacket(reason_code=mqttproto.ReasonCode.SUCCESS)


def auth_packet_full() -> mqtt5.AuthPacket:  # noqa: D103
    return mqtt5.AuthPacket(
        reason_code=mqtt5.AuthReasonCode.CONTINUE_AUTHENTICATION,
        authentication_method="GS2-KRB5",
        authentication_data=b"\x12" * 2**8,
        reason_str="The reason string is a human readable string designed for diagnostics.",  # noqa: E501
        user_properties=[
            ("location", "Pallet Town"),
            ("type", "Grass"),
        ],
    )


def auth_packet_full_mqttproto() -> mqttproto.MQTTAuthPacket:  # noqa: D103
    return mqttproto.MQTTAuthPacket(
        reason_code=mqttproto.ReasonCode.CONTINUE_AUTHENTICATION,
        properties={
            mqttproto.PropertyType.AUTHENTICATION_METHOD: "GS2-KRB5",
            mqttproto.PropertyType.AUTHENTICATION_DATA: b"\x12" * 2**8,
            mqttproto.PropertyType.REASON_STRING: "The reason string is a human readable string designed for diagnostics.",  # noqa: E501
        },
        user_properties={"location": "Pallet Town", "type": "Grass"},
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

# Collect the initialized packets
PACKETS = [f() for f in PACKET_INITS]
PACKETS_MQTTPROTO = [f() for f in PACKET_INITS_MQTTPROTO]
