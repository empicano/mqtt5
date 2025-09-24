"""The MQTTv5 protocol for Python written in Rust."""

import typing

from .mqtt5 import *

if not typing.TYPE_CHECKING:
    Packet: typing.TypeAlias = (
        ConnectPacket
        | ConnAckPacket
        | PublishPacket
        | PubAckPacket
        | PubRecPacket
        | PubRelPacket
        | PubCompPacket
        | SubscribePacket
        | SubAckPacket
        | UnsubscribePacket
        | UnsubAckPacket
        | PingReqPacket
        | PingRespPacket
        | DisconnectPacket
        | AuthPacket
    )
