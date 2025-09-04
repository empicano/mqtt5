"""The MQTTv5 protocol for Python written in Rust."""

from .mqtt5 import *

Packet = (
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
