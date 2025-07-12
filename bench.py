import pyperf
import inspect

import tests.conftest


def source(func):
    """Extract the body of a packet initialization function as str."""
    s = inspect.getsource(func).split("\n")
    del s[0], s[-1]
    s = "\n".join([line[4:] for line in s])
    if s.startswith("return"):
        s = s[7:]
    return s


runner = pyperf.Runner()
for packet_name, packet_init, packet_init_mqttproto in zip(
    tests.conftest.PACKET_NAMES,
    tests.conftest.PACKET_INITS,
    tests.conftest.PACKET_INITS_MQTTPROTO,
):
    buffer = bytearray()
    packet_init_mqttproto().encode(buffer)
    runner.timeit(
        name=f"mqtt5: Read {packet_name}",
        setup=f"import mqtt5; buffer = bytearray({bytes(buffer)!r})",
        stmt="mqtt5.read(buffer)",
    )
    runner.timeit(
        name=f"mprot: Read {packet_name}",
        setup=f"import mqttproto; buffer = memoryview(bytearray({bytes(buffer)!r}))",
        stmt="mqttproto._types.decode_packet(buffer)",
    )
    runner.timeit(
        name=f"mqtt5: Write {packet_name}",
        setup="import mqtt5",
        stmt=source(packet_init) + ".write(buffer)",
        # Packets are written to a pre-allocated buffer. Re-use a global variable
        # instead of pre-allocating the buffer again and again between runs.
        globals={"buffer": buffer},
    )
    runner.timeit(
        name=f"mprot: Write {packet_name}",
        setup="import mqttproto; buffer = bytearray()",
        stmt=source(packet_init_mqttproto) + ".encode(buffer)",
    )
