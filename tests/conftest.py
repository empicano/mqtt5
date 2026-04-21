"""Contains test configurations and packet definitions."""

import mqttproto
import zmqtt
import zmqtt._internal.packets

import mqtt5


def _connect_packet() -> mqtt5.ConnectPacket:
    return mqtt5.ConnectPacket(client_id="Bulbasaur")


def _connect_packet_mqttproto() -> mqttproto.MQTTConnectPacket:
    return mqttproto.MQTTConnectPacket(client_id="Bulbasaur")


def _connect_packet_zmqtt() -> zmqtt._internal.packets.Connect:
    return zmqtt._internal.packets.Connect(
        client_id="Bulbasaur", clean_session=False, keepalive=0
    )


def _connect_packet_will() -> mqtt5.ConnectPacket:
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


def _connect_packet_will_mqttproto() -> mqttproto.MQTTConnectPacket:
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


def _connect_packet_will_zmqtt() -> zmqtt._internal.packets.Connect:
    return zmqtt._internal.packets.Connect(
        client_id="Bulbasaur",
        clean_session=False,
        keepalive=0,
        will=zmqtt._internal.packets.Will(
            topic="foo/bar/1234",
            payload=b"\x12" * 2**8,
            qos=zmqtt.QoS.EXACTLY_ONCE,
            retain=True,
            properties=zmqtt._internal.packets.WillProperties(
                payload_format_indicator=1,
                message_expiry_interval=2**24,
                content_type="text/html",
                response_topic="foo/bar/1234",
                correlation_data=b"\x12" * 2**8,
                will_delay_interval=20,
            ),
        ),
    )


def _connect_packet_full() -> mqtt5.ConnectPacket:
    return mqtt5.ConnectPacket(
        client_id="Bulbasaur",
        username="ProfOak",
        password=b"RazorLeaf?456",
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


def _connect_packet_full_mqttproto() -> mqttproto.MQTTConnectPacket:
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


def _connect_packet_full_zmqtt() -> zmqtt._internal.packets.Connect:
    return zmqtt._internal.packets.Connect(
        client_id="Bulbasaur",
        username="ProfOak",
        password=b"RazorLeaf?456",
        clean_session=True,
        keepalive=6789,
        will=zmqtt._internal.packets.Will(
            topic="foo/bar/1234",
            payload=b"\x12" * 2**8,
            qos=zmqtt.QoS.EXACTLY_ONCE,
            retain=True,
            properties=zmqtt._internal.packets.WillProperties(
                payload_format_indicator=1,
                message_expiry_interval=2**24,
                content_type="text/html",
                response_topic="foo/bar/1234",
                correlation_data=b"\x12" * 2**8,
                will_delay_interval=20,
                user_properties=(("location", "Pallet Town"), ("type", "Grass")),
            ),
        ),
        properties=zmqtt._internal.packets.ConnectProperties(
            session_expiry_interval=600,
            authentication_method="GS2-KRB5",
            authentication_data=b"\x12" * 2**8,
            request_problem_information=False,
            request_response_information=True,
            receive_maximum=55555,
            topic_alias_maximum=3,
            maximum_packet_size=2**12,
            user_properties=(("location", "Pallet Town"), ("type", "Grass")),
        ),
    )


def _connack_packet() -> mqtt5.ConnAckPacket:
    return mqtt5.ConnAckPacket()


def _connack_packet_mqttproto() -> mqttproto.MQTTConnAckPacket:
    return mqttproto.MQTTConnAckPacket(
        session_present=False, reason_code=mqttproto.ReasonCode.SUCCESS
    )


def _connack_packet_zmqtt() -> zmqtt._internal.packets.ConnAck:
    return zmqtt._internal.packets.ConnAck(session_present=False, return_code=0)


def _connack_packet_full() -> mqtt5.ConnAckPacket:
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


def _connack_packet_full_mqttproto() -> mqttproto.MQTTConnAckPacket:
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


def _connack_packet_full_zmqtt() -> zmqtt._internal.packets.ConnAck:
    return zmqtt._internal.packets.ConnAck(
        session_present=True,
        return_code=0x80,
        properties=zmqtt._internal.packets.ConnAckProperties(
            session_expiry_interval=600,
            assigned_client_identifier="Bulbasaur",
            server_keep_alive=6789,
            authentication_method="GS2-KRB5",
            authentication_data=b"\x12" * 2**8,
            response_information="response/information",
            server_reference="example.com:1883",
            reason_string="The reason string is a human readable string designed for diagnostics.",  # noqa: E501
            receive_maximum=2**10,
            topic_alias_maximum=2**8,
            maximum_qos=0,
            retain_available=False,
            maximum_packet_size=2**12,
            wildcard_subscription_available=False,
            subscription_identifier_available=False,
            shared_subscription_available=False,
            user_properties=(("location", "Pallet Town"), ("type", "Grass")),
        ),
    )


def _publish_packet_qos0() -> mqtt5.PublishPacket:
    return mqtt5.PublishPacket(topic="foo/bar/1234", payload=b"\x12" * 2**8)


def _publish_packet_qos0_mqttproto() -> mqttproto.MQTTPublishPacket:
    return mqttproto.MQTTPublishPacket(topic="foo/bar/1234", payload=b"\x12" * 2**8)


def _publish_packet_qos0_zmqtt() -> zmqtt._internal.packets.Publish:
    return zmqtt._internal.packets.Publish(
        topic="foo/bar/1234",
        payload=b"\x12" * 2**8,
        qos=zmqtt.QoS.AT_MOST_ONCE,
        retain=False,
        dup=False,
    )


def _publish_packet_qos1() -> mqtt5.PublishPacket:
    return mqtt5.PublishPacket(
        topic="foo/bar/1234",
        payload=b"\x12" * 2**8,
        qos=mqtt5.QoS.AT_LEAST_ONCE,
        packet_id=999,
    )


def _publish_packet_qos1_mqttproto() -> mqttproto.MQTTPublishPacket:
    return mqttproto.MQTTPublishPacket(
        topic="foo/bar/1234",
        payload=b"\x12" * 2**8,
        qos=mqttproto.QoS.AT_LEAST_ONCE,
        packet_id=999,
    )


def _publish_packet_qos1_zmqtt() -> zmqtt._internal.packets.Publish:
    return zmqtt._internal.packets.Publish(
        topic="foo/bar/1234",
        payload=b"\x12" * 2**8,
        qos=zmqtt.QoS.AT_LEAST_ONCE,
        retain=False,
        dup=False,
        packet_id=999,
    )


def _puback_packet() -> mqtt5.PubAckPacket:
    return mqtt5.PubAckPacket(packet_id=999)


def _puback_packet_mqttproto() -> mqttproto.MQTTPublishAckPacket:
    return mqttproto.MQTTPublishAckPacket(
        packet_id=999, reason_code=mqttproto.ReasonCode.SUCCESS
    )


def _puback_packet_zmqtt() -> zmqtt._internal.packets.PubAck:
    return zmqtt._internal.packets.PubAck(packet_id=999)


def _puback_packet_full() -> mqtt5.PubAckPacket:
    return mqtt5.PubAckPacket(
        packet_id=999,
        reason_code=mqtt5.PubAckReasonCode.NO_MATCHING_SUBSCRIBERS,
        reason_str="The reason string is a human readable string designed for diagnostics.",  # noqa: E501
    )


def _puback_packet_full_mqttproto() -> mqttproto.MQTTPublishAckPacket:
    return mqttproto.MQTTPublishAckPacket(
        packet_id=999,
        reason_code=mqttproto.ReasonCode.NO_MATCHING_SUBSCRIBERS,
        properties={
            mqttproto.PropertyType.REASON_STRING: "The reason string is a human readable string designed for diagnostics.",  # noqa: E501
        },
    )


def _puback_packet_full_zmqtt() -> zmqtt._internal.packets.PubAck:
    return zmqtt._internal.packets.PubAck(
        packet_id=999,
        reason_code=0x10,
        properties=zmqtt._internal.packets.PubAckProperties(
            reason_string="The reason string is a human readable string designed for diagnostics.",  # noqa: E501
        ),
    )


def _pubrec_packet() -> mqtt5.PubRecPacket:
    return mqtt5.PubRecPacket(packet_id=999)


def _pubrec_packet_mqttproto() -> mqttproto.MQTTPublishReceivePacket:
    return mqttproto.MQTTPublishReceivePacket(
        packet_id=999, reason_code=mqttproto.ReasonCode.SUCCESS
    )


def _pubrec_packet_zmqtt() -> zmqtt._internal.packets.PubRec:
    return zmqtt._internal.packets.PubRec(packet_id=999)


def _pubrec_packet_full() -> mqtt5.PubRecPacket:
    return mqtt5.PubRecPacket(
        packet_id=999,
        reason_code=mqtt5.PubRecReasonCode.NO_MATCHING_SUBSCRIBERS,
        reason_str="The reason string is a human readable string designed for diagnostics.",  # noqa: E501
        user_properties=[
            ("location", "Pallet Town"),
            ("type", "Grass"),
        ],
    )


def _pubrec_packet_full_mqttproto() -> mqttproto.MQTTPublishReceivePacket:
    return mqttproto.MQTTPublishReceivePacket(
        packet_id=999,
        reason_code=mqttproto.ReasonCode.NO_MATCHING_SUBSCRIBERS,
        properties={
            mqttproto.PropertyType.REASON_STRING: "The reason string is a human readable string designed for diagnostics.",  # noqa: E501
        },
        user_properties={"location": "Pallet Town", "type": "Grass"},
    )


def _pubrec_packet_full_zmqtt() -> zmqtt._internal.packets.PubRec:
    return zmqtt._internal.packets.PubRec(
        packet_id=999,
        reason_code=0x10,
        properties=zmqtt._internal.packets.PubAckProperties(
            reason_string="The reason string is a human readable string designed for diagnostics.",  # noqa: E501
            user_properties=(("location", "Pallet Town"), ("type", "Grass")),
        ),
    )


def _pubrel_packet() -> mqtt5.PubRelPacket:
    return mqtt5.PubRelPacket(packet_id=999)


def _pubrel_packet_mqttproto() -> mqttproto.MQTTPublishReleasePacket:
    return mqttproto.MQTTPublishReleasePacket(
        packet_id=999, reason_code=mqttproto.ReasonCode.SUCCESS
    )


def _pubrel_packet_zmqtt() -> zmqtt._internal.packets.PubRel:
    return zmqtt._internal.packets.PubRel(packet_id=999)


def _pubrel_packet_full() -> mqtt5.PubRelPacket:
    return mqtt5.PubRelPacket(
        packet_id=999,
        reason_code=mqtt5.PubRelReasonCode.PACKET_ID_NOT_FOUND,
        reason_str="The reason string is a human readable string designed for diagnostics.",  # noqa: E501
        user_properties=[
            ("location", "Pallet Town"),
            ("type", "Grass"),
        ],
    )


def _pubrel_packet_full_mqttproto() -> mqttproto.MQTTPublishReleasePacket:
    return mqttproto.MQTTPublishReleasePacket(
        packet_id=999,
        reason_code=mqttproto.ReasonCode.PACKET_IDENTIFIER_NOT_FOUND,
        properties={
            mqttproto.PropertyType.REASON_STRING: "The reason string is a human readable string designed for diagnostics.",  # noqa: E501
        },
        user_properties={"location": "Pallet Town", "type": "Grass"},
    )


def _pubrel_packet_full_zmqtt() -> zmqtt._internal.packets.PubRel:
    return zmqtt._internal.packets.PubRel(
        packet_id=999,
        reason_code=0x92,
        properties=zmqtt._internal.packets.PubAckProperties(
            reason_string="The reason string is a human readable string designed for diagnostics.",  # noqa: E501
            user_properties=(("location", "Pallet Town"), ("type", "Grass")),
        ),
    )


def _pubcomp_packet() -> mqtt5.PubCompPacket:
    return mqtt5.PubCompPacket(packet_id=999)


def _pubcomp_packet_mqttproto() -> mqttproto.MQTTPublishCompletePacket:
    return mqttproto.MQTTPublishCompletePacket(
        packet_id=999, reason_code=mqttproto.ReasonCode.SUCCESS
    )


def _pubcomp_packet_zmqtt() -> zmqtt._internal.packets.PubComp:
    return zmqtt._internal.packets.PubComp(packet_id=999)


def _pubcomp_packet_full() -> mqtt5.PubCompPacket:
    return mqtt5.PubCompPacket(
        packet_id=999,
        reason_code=mqtt5.PubCompReasonCode.PACKET_ID_NOT_FOUND,
        reason_str="The reason string is a human readable string designed for diagnostics.",  # noqa: E501
        user_properties=[
            ("location", "Pallet Town"),
            ("type", "Grass"),
        ],
    )


def _pubcomp_packet_full_mqttproto() -> mqttproto.MQTTPublishCompletePacket:
    return mqttproto.MQTTPublishCompletePacket(
        packet_id=999,
        reason_code=mqttproto.ReasonCode.PACKET_IDENTIFIER_NOT_FOUND,
        properties={
            mqttproto.PropertyType.REASON_STRING: "The reason string is a human readable string designed for diagnostics.",  # noqa: E501
        },
        user_properties={"location": "Pallet Town", "type": "Grass"},
    )


def _pubcomp_packet_full_zmqtt() -> zmqtt._internal.packets.PubComp:
    return zmqtt._internal.packets.PubComp(
        packet_id=999,
        reason_code=0x92,
        properties=zmqtt._internal.packets.PubAckProperties(
            reason_string="The reason string is a human readable string designed for diagnostics.",  # noqa: E501
            user_properties=(("location", "Pallet Town"), ("type", "Grass")),
        ),
    )


def _subscribe_packet() -> mqtt5.SubscribePacket:
    return mqtt5.SubscribePacket(
        packet_id=999, topic_filters=[mqtt5.TopicFilter(pattern="+/bar/#")]
    )


def _subscribe_packet_mqttproto() -> mqttproto.MQTTSubscribePacket:
    return mqttproto.MQTTSubscribePacket(
        packet_id=999, subscriptions=[mqttproto.Subscription(pattern="+/bar/#")]
    )


def _subscribe_packet_zmqtt() -> zmqtt._internal.packets.Subscribe:
    return zmqtt._internal.packets.Subscribe(
        packet_id=999,
        subscriptions=(
            zmqtt._internal.packets.SubscriptionRequest(
                topic_filter="+/bar/#",
                qos=zmqtt.QoS.EXACTLY_ONCE,
                retain_as_published=True,
            ),
        ),
    )


def _suback_packet() -> mqtt5.SubAckPacket:
    return mqtt5.SubAckPacket(
        packet_id=999, reason_codes=[mqtt5.SubAckReasonCode.TOPIC_FILTER_INVALID]
    )


def _suback_packet_mqttproto() -> mqttproto.MQTTSubscribeAckPacket:
    return mqttproto.MQTTSubscribeAckPacket(
        packet_id=999, reason_codes=[mqttproto.ReasonCode.TOPIC_FILTER_INVALID]
    )


def _suback_packet_zmqtt() -> zmqtt._internal.packets.SubAck:
    return zmqtt._internal.packets.SubAck(packet_id=999, return_codes=(0x8F,))


def _unsubscribe_packet() -> mqtt5.UnsubscribePacket:
    return mqtt5.UnsubscribePacket(packet_id=999, patterns=["+/bar/#", "foo/#"])


def _unsubscribe_packet_mqttproto() -> mqttproto.MQTTUnsubscribePacket:
    return mqttproto.MQTTUnsubscribePacket(packet_id=999, patterns=["+/bar/#", "foo/#"])


def _unsubscribe_packet_zmqtt() -> zmqtt._internal.packets.Unsubscribe:
    return zmqtt._internal.packets.Unsubscribe(
        packet_id=999, topic_filters=("+/bar/#", "foo/#")
    )


def _unsuback_packet() -> mqtt5.UnsubAckPacket:
    return mqtt5.UnsubAckPacket(
        packet_id=999,
        reason_codes=[mqtt5.UnsubAckReasonCode.TOPIC_FILTER_INVALID],
        reason_str="The reason string is a human readable string designed for diagnostics.",  # noqa: E501
        user_properties=[
            ("location", "Pallet Town"),
            ("type", "Grass"),
        ],
    )


def _unsuback_packet_mqttproto() -> mqttproto.MQTTUnsubscribeAckPacket:
    return mqttproto.MQTTUnsubscribeAckPacket(
        packet_id=999,
        reason_codes=[mqttproto.ReasonCode.TOPIC_FILTER_INVALID],
        properties={
            mqttproto.PropertyType.REASON_STRING: "The reason string is a human readable string designed for diagnostics.",  # noqa: E501
        },
        user_properties={"location": "Pallet Town", "type": "Grass"},
    )


def _unsuback_packet_zmqtt() -> zmqtt._internal.packets.UnsubAck:
    return zmqtt._internal.packets.UnsubAck(
        packet_id=999,
        reason_codes=(0x8F,),
        properties=zmqtt._internal.packets.UnsubAckProperties(
            reason_string="The reason string is a human readable string designed for diagnostics.",  # noqa: E501
            user_properties=(("location", "Pallet Town"), ("type", "Grass")),
        ),
    )


def _pingreq_packet() -> mqtt5.PingReqPacket:
    return mqtt5.PingReqPacket()


def _pingreq_packet_mqttproto() -> mqttproto.MQTTPingRequestPacket:
    return mqttproto.MQTTPingRequestPacket()


def _pingreq_packet_zmqtt() -> zmqtt._internal.packets.PingReq:
    return zmqtt._internal.packets.PingReq()


def _pingresp_packet() -> mqtt5.PingRespPacket:
    return mqtt5.PingRespPacket()


def _pingresp_packet_mqttproto() -> mqttproto.MQTTPingResponsePacket:
    return mqttproto.MQTTPingResponsePacket()


def _pingresp_packet_zmqtt() -> zmqtt._internal.packets.PingResp:
    return zmqtt._internal.packets.PingResp()


def _disconnect_packet() -> mqtt5.DisconnectPacket:
    return mqtt5.DisconnectPacket()


def _disconnect_packet_mqttproto() -> mqttproto.MQTTDisconnectPacket:
    return mqttproto.MQTTDisconnectPacket(
        reason_code=mqttproto.ReasonCode.NORMAL_DISCONNECTION
    )


def _disconnect_packet_zmqtt() -> zmqtt._internal.packets.Disconnect:
    return zmqtt._internal.packets.Disconnect(reason_code=0)


def _disconnect_packet_full() -> mqtt5.DisconnectPacket:
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


def _disconnect_packet_full_mqttproto() -> mqttproto.MQTTDisconnectPacket:
    return mqttproto.MQTTDisconnectPacket(
        reason_code=mqttproto.ReasonCode.SERVER_SHUTTING_DOWN,
        properties={
            mqttproto.PropertyType.SESSION_EXPIRY_INTERVAL: 600,
            mqttproto.PropertyType.SERVER_REFERENCE: "example.com:1883",
            mqttproto.PropertyType.REASON_STRING: "The reason string is a human readable string designed for diagnostics.",  # noqa: E501
        },
        user_properties={"location": "Pallet Town", "type": "Grass"},
    )


def _disconnect_packet_full_zmqtt() -> zmqtt._internal.packets.Disconnect:
    return zmqtt._internal.packets.Disconnect(
        reason_code=0x8B,
        properties=zmqtt._internal.packets.DisconnectProperties(
            session_expiry_interval=600,
            server_reference="example.com:1883",
            reason_string="The reason string is a human readable string designed for diagnostics.",  # noqa: E501
            user_properties=(("location", "Pallet Town"), ("type", "Grass")),
        ),
    )


def _auth_packet() -> mqtt5.AuthPacket:
    return mqtt5.AuthPacket()


def _auth_packet_mqttproto() -> mqttproto.MQTTAuthPacket:
    return mqttproto.MQTTAuthPacket(reason_code=mqttproto.ReasonCode.SUCCESS)


def _auth_packet_zmqtt() -> zmqtt._internal.packets.Auth:
    return zmqtt._internal.packets.Auth(reason_code=0)


def _auth_packet_full() -> mqtt5.AuthPacket:
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


def _auth_packet_full_mqttproto() -> mqttproto.MQTTAuthPacket:
    return mqttproto.MQTTAuthPacket(
        reason_code=mqttproto.ReasonCode.CONTINUE_AUTHENTICATION,
        properties={
            mqttproto.PropertyType.AUTHENTICATION_METHOD: "GS2-KRB5",
            mqttproto.PropertyType.AUTHENTICATION_DATA: b"\x12" * 2**8,
            mqttproto.PropertyType.REASON_STRING: "The reason string is a human readable string designed for diagnostics.",  # noqa: E501
        },
        user_properties={"location": "Pallet Town", "type": "Grass"},
    )


def _auth_packet_full_zmqtt() -> zmqtt._internal.packets.Auth:
    return zmqtt._internal.packets.Auth(
        reason_code=0x18,
        properties=zmqtt._internal.packets.AuthProperties(
            authentication_method="GS2-KRB5",
            authentication_data=b"\x12" * 2**8,
            reason_string="The reason string is a human readable string designed for diagnostics.",  # noqa: E501
            user_properties=(("location", "Pallet Town"), ("type", "Grass")),
        ),
    )


PACKET_NAMES, PACKET_INITS, PACKET_INITS_MQTTPROTO, PACKET_INITS_ZMQTT = [], [], [], []

for key, value in dict(locals()).items():
    tags = key.lstrip("_").split("_")
    if len(tags) > 1 and tags[1] == "packet":
        if tags[-1] == "mqttproto":
            PACKET_INITS_MQTTPROTO.append(value)
            continue
        if tags[-1] == "zmqtt":
            PACKET_INITS_ZMQTT.append(value)
            continue
        name = type(value()).__name__[:-6]
        if len(tags) > 2:
            name += f"({'_'.join(tags[2:])})"
        PACKET_NAMES.append(name)
        PACKET_INITS.append(value)

# Collect the initialized packets
PACKETS = [f() for f in PACKET_INITS]
PACKETS_MQTTPROTO = [f() for f in PACKET_INITS_MQTTPROTO]
PACKETS_ZMQTT = [f() for f in PACKET_INITS_ZMQTT]
