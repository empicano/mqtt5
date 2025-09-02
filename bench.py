import pyperf
import inspect
import argparse

import tests.conftest


def source(func):
    """Extract the body of a packet initialization function as str."""
    s = inspect.getsource(func).split("\n")
    del s[0], s[-1]
    s = "\n".join([line[4:] for line in s])
    if s.startswith("return"):
        s = s[7:]
    return s


def _add_cmdline_args(cmd, args):
    """Propagate our custom runner command line arguments to the workers."""
    for item in args.packets:
        cmd.extend(["--packets", item])
    if args.compare:
        cmd.append("--compare")


runner = pyperf.Runner(add_cmdline_args=_add_cmdline_args)
runner.argparser.add_argument("--packets", action="append", type=str, default=[])
runner.argparser.add_argument("--compare", action="store_true")
args = runner.parse_args()

# Parse command line arguments for the workers
parser = argparse.ArgumentParser()
parser.add_argument("--packets", action="append", type=str, default=[])
parser.add_argument("--compare", action="store_true")
args, _ = parser.parse_known_args()

benchmarks = [
    (packet_name, packet_init, packet_init_mqttproto)
    for (packet_name, packet_init, packet_init_mqttproto) in zip(
        tests.conftest.PACKET_NAMES,
        tests.conftest.PACKET_INITS,
        tests.conftest.PACKET_INITS_MQTTPROTO,
    )
    if len(args.packets) == 0
    or any(packet_name.lower().startswith(item.lower()) for item in args.packets)
]

for packet_name, packet_init, packet_init_mqttproto in benchmarks:
    buffer = bytearray()
    packet_init_mqttproto().encode(buffer)
    runner.timeit(
        name=f"mqtt5: Read {packet_name}",
        setup=f"import mqtt5; buffer = bytearray({bytes(buffer)!r})",
        stmt="mqtt5.read(buffer)",
    )
    if args.compare:
        runner.timeit(
            name=f"proto: Read {packet_name}",
            setup=f"import mqttproto; buffer = memoryview(bytearray({bytes(buffer)!r}))",
            stmt="mqttproto._types.decode_packet(buffer)",
        )
    runner.timeit(
        name=f"mqtt5: Write {packet_name}",
        setup="import mqtt5",
        stmt=f"{source(packet_init)}.write()",
    )
    if args.compare:
        runner.timeit(
            name=f"proto: Write {packet_name}",
            setup="import mqttproto; buffer = bytearray()",
            stmt=f"{source(packet_init_mqttproto)}.encode(buffer)",
        )
