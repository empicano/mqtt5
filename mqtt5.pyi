import typing
import dataclasses
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
    BAD_AUTHENTICATION_METHOD = 140
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

class PubCompReasonCode(enum.IntEnum):
    SUCCESS = 0
    PACKET_ID_NOT_FOUND = 146

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
    RECEIVE_MAXIMUM_EXCEEDED = 147
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
    MAXIMUM_CONNECT_TIME = 160
    SUBSCRIPTION_IDS_NOT_SUPPORTED = 161
    WILDCARD_SUBSCRIPTIONS_NOT_SUPPORTED = 162

@dataclasses.dataclass
class WillProperties:
    payload_format_indicator: int | None = None
    message_expiry_interval: int | None = None
    content_type: str | None = None
    response_topic: str | None = None
    correlation_data: bytes | None = None
    will_delay_interval: int | None = None

@dataclasses.dataclass
class ConnectProperties:
    session_expiry_interval: int | None = None
    authentication_method: str | None = None
    authentication_data: bytes | None = None
    request_problem_information: int | None = None
    request_response_information: int | None = None
    receive_maximum: int | None = None
    topic_alias_maximum: int | None = None
    maximum_packet_size: int | None = None

@dataclasses.dataclass
class ConnAckProperties:
    session_expiry_interval: int | None = None
    assigned_client_id: str | None = None
    server_keep_alive: int | None = None
    authentication_method: str | None = None
    authentication_data: bytes | None = None
    response_information: str | None = None
    server_reference: str | None = None
    reason_string: str | None = None
    receive_maximum: int | None = None
    topic_alias_maximum: int | None = None
    maximum_qos: int | None = None
    retain_available: int | None = None
    maximum_packet_size: int | None = None
    wildcard_subscription_available: int | None = None
    subscription_id_available: int | None = None
    shared_subscription_available: int | None = None

@dataclasses.dataclass
class PublishProperties:
    payload_format_indicator: int | None = None
    message_expiry_interval: int | None = None
    content_type: str | None = None
    response_topic: str | None = None
    correlation_data: bytes | None = None
    subscription_id: int | None = None
    topic_alias: int | None = None

@dataclasses.dataclass
class PubAckProperties:
    reason_string: str | None = None

@dataclasses.dataclass
class PubRecProperties:
    reason_string: str | None = None

@dataclasses.dataclass
class PubCompProperties:
    reason_string: str | None = None

@dataclasses.dataclass
class SubscribeProperties:
    subscription_id: int | None = None

@dataclasses.dataclass
class SubAckProperties:
    reason_string: str | None = None

@dataclasses.dataclass
class DisconnectProperties:
    session_expiry_interval: int | None = None
    server_reference: str | None = None
    reason_string: str | None = None

@dataclasses.dataclass
class Will:
    topic: str
    payload: bytes | None = None
    qos: QoS = QoS.AT_MOST_ONCE
    retain: bool = False
    properties: WillProperties | None = None

@dataclasses.dataclass
class Subscription:
    pattern: str
    maximum_qos: QoS = QoS.AT_MOST_ONCE
    no_local: bool = False
    retain_as_published: bool = True
    retain_handling: RetainHandling = RetainHandling.SEND_ALWAYS

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
    properties: ConnectProperties

    def __init__(
        self,
        client_id: str,
        *,
        username: str | None = None,
        password: str | None = None,
        clean_start: bool = False,
        will: Will | None = None,
        keep_alive: int = 0,
        properties: ConnectProperties | None = None,
    ) -> None: ...
    def write(self, buffer: bytearray, /, *, index: int = 0) -> int:
        """
        Writes the packet to the buffer.

        :return: The number of bytes written
        """

class ConnAckPacket:
    session_present: bool
    reason_code: ConnAckReasonCode
    properties: ConnAckProperties

    def __init__(
        self,
        *,
        session_present: bool = False,
        reason_code: ConnAckReasonCode = ConnAckReasonCode.SUCCESS,
        properties: ConnAckProperties | None = None,
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
    properties: PublishProperties

    def __init__(
        self,
        topic: str,
        *,
        payload: bytes | None = None,
        qos: QoS = QoS.AT_MOST_ONCE,
        retain: bool = False,
        packet_id: int | None = None,
        duplicate: bool = False,
        properties: PublishProperties | None = None,
    ) -> None: ...
    def write(self, buffer: bytearray, /, *, index: int = 0) -> int:
        """
        Writes the packet to the buffer.

        :return: The number of bytes written
        """

class PubAckPacket:
    packet_id: int
    reason_code: PubAckReasonCode
    properties: PubAckProperties

    def __init__(
        self,
        packet_id: int,
        *,
        reason_code: PubAckReasonCode = PubAckReasonCode.SUCCESS,
        properties: PubAckProperties | None = None,
    ) -> None: ...
    def write(self, buffer: bytearray, /, *, index: int = 0) -> int:
        """
        Writes the packet to the buffer.

        :return: The number of bytes written
        """

class SubscribePacket:
    packet_id: int
    subscriptions: list[Subscription]
    properties: SubscribeProperties

    def __init__(
        self,
        packet_id: int,
        subscriptions: list[Subscription],
        *,
        properties: SubscribeProperties | None = None,
    ) -> None: ...
    def write(self, buffer: bytearray, /, *, index: int = 0) -> int:
        """
        Writes the packet to the buffer.

        :return: The number of bytes written
        """

class DisconnectPacket:
    reason_code: DisconnectReasonCode
    properties: DisconnectProperties

    def __init__(
        self,
        *,
        reason_code: DisconnectReasonCode = DisconnectReasonCode.NORMAL_DISCONNECTION,
        properties: DisconnectProperties | None = None,
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
