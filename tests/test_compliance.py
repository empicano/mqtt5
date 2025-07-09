"""Tests specification compliance by comparing mqtt5 and mqttproto outputs."""


def test_compliance(request, packet_fixture, buffer):
    """Test that mqtt5 writes the same bytes as mqttproto for all packet types."""
    n = request.getfixturevalue(packet_fixture).write(buffer)
    buffer_mqttproto = bytearray()
    request.getfixturevalue(packet_fixture + "_mqttproto").encode(buffer_mqttproto)
    assert n == len(buffer_mqttproto)
    assert buffer[:n] == buffer_mqttproto
