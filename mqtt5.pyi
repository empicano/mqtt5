import typing
import enum

class QoS(enum.IntEnum):
    AT_MOST_ONCE = 0
    AT_LEAST_ONCE = 1
    EXACTLY_ONCE = 2

class RetainHandling(enum.IntEnum):
    SEND_ALWAYS = 0
    SEND_IF_SUBSCRIPTION_NOT_EXISTS = 1
    SEND_NEVER = 2

class ConnAckReasonCode(enum.IntEnum):
    SUCCESS = 0
    UNSPECIFIED_ERROR = 128
    MALFORMED_PACKET = 129
    PROTOCOL_ERROR = 130
    IMPLEMENTATION_SPECIFIC_ERROR = 131
    UNSUPPORTED_PROTOCOL_VERSION = 132
    CLIENT_ID_NOT_VALID = 133
    BAD_USER_NAME_OR_PASSWORD = 134
    NOT_AUTHORIZED = 135
    SERVER_UNAVAILABLE = 136
    SERVER_BUSY = 137
    BANNED = 138
    BAD_AUTH_METHOD = 140
    TOPIC_NAME_INVALID = 144
    PACKET_TOO_LARGE = 149
    QUOTA_EXCEEDED = 151
    PAYLOAD_FORMAT_INVALID = 153
    RETAIN_NOT_SUPPORTED = 154
    QUALITY_NOT_SUPPORTED = 155
    USE_ANOTHER_SERVER = 156
    SERVER_MOVED = 157
    CONNECTION_RATE_EXCEEDED = 159

class PubAckReasonCode(enum.IntEnum):
    SUCCESS = 0
    NO_MATCHING_SUBSCRIBERS = 16
    UNSPECIFIED_ERROR = 128
    IMPLEMENTATION_SPECIFIC_ERROR = 131
    NOT_AUTHORIZED = 135
    TOPIC_NAME_INVALID = 144
    PACKET_ID_IN_USE = 145
    QUOTA_EXCEEDED = 151
    PAYLOAD_FORMAT_INVALID = 153

class PubRecReasonCode(enum.IntEnum):
    SUCCESS = 0
    NO_MATCHING_SUBSCRIBERS = 16
    UNSPECIFIED_ERROR = 128
    IMPLEMENTATION_SPECIFIC_ERROR = 131
    NOT_AUTHORIZED = 135
    TOPIC_NAME_INVALID = 144
    PACKET_ID_IN_USE = 145
    QUOTA_EXCEEDED = 151
    PAYLOAD_FORMAT_INVALID = 153

class PubRelReasonCode(enum.IntEnum):
    SUCCESS = 0
    PACKET_ID_NOT_FOUND = 146

class PubCompReasonCode(enum.IntEnum):
    SUCCESS = 0
    PACKET_ID_NOT_FOUND = 146

class SubAckReasonCode(enum.IntEnum):
    GRANTED_QOS_AT_MOST_ONCE = 0
    GRANTED_QOS_AT_LEAST_ONCE = 1
    GRANTED_QOS_EXACTLY_ONCE = 2
    UNSPECIFIED_ERROR = 128
    IMPLEMENTATION_SPECIFIC_ERROR = 131
    NOT_AUTHORIZED = 135
    TOPIC_FILTER_INVALID = 143
    PACKET_ID_IN_USE = 145
    QUOTA_EXCEEDED = 151
    SHARED_SUBSCRIPTIONS_NOT_SUPPORTED = 158
    SUBSCRIPTION_IDS_NOT_SUPPORTED = 161
    WILDCARD_SUBSCRIPTIONS_NOT_SUPPORTED = 162

class UnsubAckReasonCode(enum.IntEnum):
    SUCCESS = 0
    NO_SUBSCRIPTION_EXISTED = 17
    UNSPECIFIED_ERROR = 128
    IMPLEMENTATION_SPECIFIC_ERROR = 131
    NOT_AUTHORIZED = 135
    TOPIC_FILTER_INVALID = 143
    PACKET_ID_IN_USE = 145

class DisconnectReasonCode(enum.IntEnum):
    NORMAL_DISCONNECTION = 0
    DISCONNECT_WITH_WILL_MESSAGE = 4
    UNSPECIFIED_ERROR = 128
    MALFORMED_PACKET = 129
    PROTOCOL_ERROR = 130
    IMPLEMENTATION_SPECIFIC_ERROR = 131
    NOT_AUTHORIZED = 135
    SERVER_BUSY = 137
    SERVER_SHUTTING_DOWN = 139
    KEEP_ALIVE_TIMEOUT = 141
    SESSION_TAKEN_OVER = 142
    TOPIC_FILTER_INVALID = 143
    TOPIC_NAME_INVALID = 144
    RECEIVE_MAX_EXCEEDED = 147
    TOPIC_ALIAS_INVALID = 148
    PACKET_TOO_LARGE = 149
    MESSAGE_RATE_TOO_HIGH = 150
    QUOTA_EXCEEDED = 151
    ADMINISTRATIVE_ACTION = 152
    PAYLOAD_FORMAT_INVALID = 153
    RETAIN_NOT_SUPPORTED = 154
    QOS_NOT_SUPPORTED = 155
    USE_ANOTHER_SERVER = 156
    SERVER_MOVED = 157
    SHARED_SUBSCRIPTIONS_NOT_SUPPORTED = 158
    CONNECTION_RATE_EXCEEDED = 159
    MAX_CONNECT_TIME = 160
    SUBSCRIPTION_IDS_NOT_SUPPORTED = 161
    WILDCARD_SUBSCRIPTIONS_NOT_SUPPORTED = 162

class Will:
    topic: str
    payload: bytes | None
    qos: QoS
    retain: bool
    payload_format_indicator: int
    message_expiry_interval: int | None
    content_type: str | None
    response_topic: str | None
    correlation_data: bytes | None
    will_delay_interval: int
    user_properties: list[tuple[str, str]]

    def __init__(
        self,
        topic: str,
        *,
        payload: bytes | None = None,
        qos: QoS = QoS.AT_MOST_ONCE,
        retain: bool = False,
        payload_format_indicator: int = 0,
        message_expiry_interval: int | None = None,
        content_type: str | None = None,
        response_topic: str | None = None,
        correlation_data: bytes | None = None,
        will_delay_interval: int = 0,
        user_properties: list[tuple[str, str]] | None = None
    ) -> None: ...

class Subscription:
    pattern: str
    max_qos: QoS
    no_local: bool
    retain_as_published: bool
    retain_handling: RetainHandling

    def __init__(
        self,
        pattern: str,
        *,
        max_qos: QoS = QoS.AT_MOST_ONCE,
        no_local: bool = False,
        retain_as_published: bool = True,
        retain_handling: RetainHandling = RetainHandling.SEND_ALWAYS,
    ) -> None: ...

class Packet(typing.Protocol):
    def write(self, buffer: bytearray, /, *, index: int = 0) -> int:
        """
        Writes the packet to the buffer.

        :return: The number of bytes written
        """

class ConnectPacket:
    client_id: str
    username: str | None
    password: str | None
    clean_start: bool
    will: Will
    keep_alive: int
    session_expiry_interval: int
    auth_method: str | None
    auth_data: bytes | None
    request_problem_info: bool
    request_response_info: bool
    receive_max: int
    topic_alias_max: int
    max_packet_size: int | None
    user_properties: list[tuple[str, str]]

    def __init__(
        self,
        client_id: str,
        *,
        username: str | None = None,
        password: str | None = None,
        clean_start: bool = False,
        will: Will | None = None,
        keep_alive: int = 0,
        session_expiry_interval: int = 0,
        auth_method: str | None = None,
        auth_data: bytes | None = None,
        request_problem_info: bool = True,
        request_response_info: bool = False,
        receive_max: int = 65535,
        topic_alias_max: int = 0,
        max_packet_size: int | None = None,
        user_properties: list[tuple[str, str]] | None = None,
    ) -> None: ...
    def write(self, buffer: bytearray, /, *, index: int = 0) -> int:
        """
        Writes the packet to the buffer.

        :return: The number of bytes written
        """

class ConnAckPacket:
    session_present: bool
    reason_code: ConnAckReasonCode
    session_expiry_interval: int | None
    assigned_client_id: str | None
    server_keep_alive: int | None
    auth_method: str | None
    auth_data: bytes | None
    response_info: str | None
    server_reference: str | None
    reason_str: str | None
    receive_max: int
    topic_alias_max: int
    max_qos: QoS
    retain_available: bool
    max_packet_size: int | None
    wildcard_subscription_available: bool
    subscription_id_available: bool
    shared_subscription_available: bool
    user_properties: list[tuple[str, str]]

    def __init__(
        self,
        *,
        session_present: bool = False,
        reason_code: ConnAckReasonCode = ConnAckReasonCode.SUCCESS,
        session_expiry_interval: int | None = None,
        assigned_client_id: str | None = None,
        server_keep_alive: int | None = None,
        auth_method: str | None = None,
        auth_data: bytes | None = None,
        response_info: str | None = None,
        server_reference: str | None = None,
        reason_str: str | None = None,
        receive_max: int = 65535,
        topic_alias_max: int = 0,
        max_qos: QoS = QoS.EXACTLY_ONCE,
        retain_available: bool = True,
        max_packet_size: int | None = None,
        wildcard_subscription_available: bool = True,
        subscription_id_available: bool = True,
        shared_subscription_available: bool = True,
        user_properties: list[tuple[str, str]] | None = None,
    ): ...
    def write(self, buffer: bytearray, /, *, index: int = 0) -> int:
        """
        Writes the packet to the buffer.

        :return: The number of bytes written
        """

class PublishPacket:
    topic: str
    payload: bytes
    qos: QoS
    retain: bool
    packet_id: int | None
    duplicate: bool
    payload_format_indicator: int
    message_expiry_interval: int | None
    content_type: str | None
    response_topic: str | None
    correlation_data: bytes | None
    subscription_ids: list[int]
    topic_alias: int | None
    user_properties: list[tuple[str, str]]

    def __init__(
        self,
        topic: str,
        *,
        payload: bytes | None = None,
        qos: QoS = QoS.AT_MOST_ONCE,
        retain: bool = False,
        packet_id: int | None = None,
        duplicate: bool = False,
        payload_format_indicator: int = 0,
        message_expiry_interval: int | None = None,
        content_type: str | None = None,
        response_topic: str | None = None,
        correlation_data: bytes | None = None,
        subscription_ids: list[int] | None = None,
        topic_alias: int | None = None,
        user_properties: list[tuple[str, str]] | None = None,
    ) -> None: ...
    def write(self, buffer: bytearray, /, *, index: int = 0) -> int:
        """
        Writes the packet to the buffer.

        :return: The number of bytes written
        """

class PubAckPacket:
    packet_id: int
    reason_code: PubAckReasonCode
    reason_str: str | None
    user_properties: list[tuple[str, str]]

    def __init__(
        self,
        packet_id: int,
        *,
        reason_code: PubAckReasonCode = PubAckReasonCode.SUCCESS,
        reason_str: str | None = None,
        user_properties: list[tuple[str, str]] | None = None,
    ) -> None: ...
    def write(self, buffer: bytearray, /, *, index: int = 0) -> int:
        """
        Writes the packet to the buffer.

        :return: The number of bytes written
        """

class PubRecPacket:
    packet_id: int
    reason_code: PubRecReasonCode
    reason_str: str | None
    user_properties: list[tuple[str, str]]

    def __init__(
        self,
        packet_id: int,
        *,
        reason_code: PubRecReasonCode = PubRecReasonCode.SUCCESS,
        reason_str: str | None = None,
        user_properties: list[tuple[str, str]] | None = None,
    ) -> None: ...
    def write(self, buffer: bytearray, /, *, index: int = 0) -> int:
        """
        Writes the packet to the buffer.

        :return: The number of bytes written
        """

class PubRelPacket:
    packet_id: int
    reason_code: PubRelReasonCode
    reason_str: str | None
    user_properties: list[tuple[str, str]]

    def __init__(
        self,
        packet_id: int,
        *,
        reason_code: PubRelReasonCode = PubRelReasonCode.SUCCESS,
        reason_str: str | None = None,
        user_properties: list[tuple[str, str]] | None = None,
    ) -> None: ...
    def write(self, buffer: bytearray, /, *, index: int = 0) -> int:
        """
        Writes the packet to the buffer.

        :return: The number of bytes written
        """

class PubCompPacket:
    packet_id: int
    reason_code: PubCompReasonCode
    reason_str: str | None
    user_properties: list[tuple[str, str]]

    def __init__(
        self,
        packet_id: int,
        *,
        reason_code: PubCompReasonCode = PubCompReasonCode.SUCCESS,
        reason_str: str | None = None,
        user_properties: list[tuple[str, str]] | None = None,
    ) -> None: ...
    def write(self, buffer: bytearray, /, *, index: int = 0) -> int:
        """
        Writes the packet to the buffer.

        :return: The number of bytes written
        """

class SubscribePacket:
    packet_id: int
    subscriptions: list[Subscription]
    subscription_id: int | None
    user_properties: list[tuple[str, str]]

    def __init__(
        self,
        packet_id: int,
        subscriptions: list[Subscription],
        *,
        subscription_id: int | None = None,
        user_properties: list[tuple[str, str]] | None = None,
    ) -> None: ...
    def write(self, buffer: bytearray, /, *, index: int = 0) -> int:
        """
        Writes the packet to the buffer.

        :return: The number of bytes written
        """

class SubAckPacket:
    packet_id: int
    reason_codes: list[SubAckReasonCode]
    reason_str: str | None
    user_properties: list[tuple[str, str]]

    def __init__(
        self,
        packet_id: int,
        reason_codes: list[SubAckReasonCode],
        *,
        reason_str: str | None = None,
        user_properties: list[tuple[str, str]] | None = None,
    ) -> None: ...
    def write(self, buffer: bytearray, /, *, index: int = 0) -> int:
        """
        Writes the packet to the buffer.

        :return: The number of bytes written
        """

class UnsubscribePacket:
    packet_id: int
    patterns: list[str]
    user_properties: list[tuple[str, str]]

    def __init__(
        self,
        packet_id: int,
        patterns: list[str],
        *,
        user_properties: list[tuple[str, str]] | None = None,
    ) -> None: ...
    def write(self, buffer: bytearray, /, *, index: int = 0) -> int:
        """
        Writes the packet to the buffer.

        :return: The number of bytes written
        """

class UnsubAckPacket:
    packet_id: int
    reason_codes: list[UnsubAckReasonCode]
    reason_str: str | None
    user_properties: list[tuple[str, str]]

    def __init__(
        self,
        packet_id: int,
        reason_codes: list[UnsubAckReasonCode],
        *,
        reason_str: str | None = None,
        user_properties: list[tuple[str, str]] | None = None,
    ) -> None: ...
    def write(self, buffer: bytearray, /, *, index: int = 0) -> int:
        """
        Writes the packet to the buffer.

        :return: The number of bytes written
        """

class PingReqPacket:
    def __init__(self) -> None: ...
    def write(self, buffer: bytearray, /, *, index: int = 0) -> int:
        """
        Writes the packet to the buffer.

        :return: The number of bytes written
        """

class PingRespPacket:
    def __init__(self) -> None: ...
    def write(self, buffer: bytearray, /, *, index: int = 0) -> int:
        """
        Writes the packet to the buffer.

        :return: The number of bytes written
        """

class DisconnectPacket:
    reason_code: DisconnectReasonCode
    session_expiry_interval: int | None
    server_reference: str | None
    reason_str: str | None
    user_properties: list[tuple[str, str]]

    def __init__(
        self,
        *,
        reason_code: DisconnectReasonCode = DisconnectReasonCode.NORMAL_DISCONNECTION,
        session_expiry_interval: int | None = None,
        server_reference: str | None = None,
        reason_str: str | None = None,
        user_properties: list[tuple[str, str]] | None = None,
    ) -> None: ...
    def write(self, buffer: bytearray, /, *, index: int = 0) -> int:
        """
        Writes the packet to the buffer.

        :return: The number of bytes written
        """

def read(buffer: bytearray, /, *, index: int = 0) -> tuple[Packet, int]:
    """
    Reads the next packet from the buffer.

    :return: The packet object and the number of bytes read
    """
