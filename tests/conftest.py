import pytest
import mqtt5


@pytest.fixture(scope="session")
def buffer():
    """Pre-allocated buffer for packet de/serialization."""
    return bytearray(1024)


@pytest.fixture(scope="session")
def connect_packet():
    return mqtt5.ConnectPacket(client_id="Bulbasaur")


@pytest.fixture(scope="session")
def connack_packet():
    return mqtt5.ConnAckPacket()


@pytest.fixture(scope="session")
def publish_packet():
    # TODO: Make payload empty
    return mqtt5.PublishPacket(topic="foo/bar/+", payload=b"\x12" * 2**8)


@pytest.fixture(scope="session")
def puback_packet():
    return mqtt5.PubAckPacket(packet_id=1234)


@pytest.fixture(scope="session")
def disconnect_packet():
    return mqtt5.DisconnectPacket()
