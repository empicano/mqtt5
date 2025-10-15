# mqtt5

<a href="https://pypi.org/project/mqtt5"><img alt="PyPI downloads" src="https://img.shields.io/pypi/dm/mqtt5"></a> <a href="https://pypi.org/project/mqtt5"><img alt="PyPI version" src="https://img.shields.io/pypi/v/mqtt5"></a> <a href="https://pypi.org/project/mqtt5"><img alt="Supported Python versions" src="https://img.shields.io/pypi/pyversions/mqtt5"></a>

A [sans-I/O](https://sans-io.readthedocs.io/how-to-sans-io.html#what-is-an-i-o-free-protocol-implementation) implementation of the MQTTv5 protocol for Python, written in Rust. Serialization is ~5x faster and deserialization ~20x faster than comparable Python code (benchmarked against [mqttproto](https://github.com/agronholm/mqttproto)).

<p align="center">
    <img src="https://github.com/empicano/mqtt5/blob/main/chart.svg?raw=true" width="650px" />
</p>

<p align="center">
  <i>Reading/Writing a QoS=1 Publish packet with 256 bytes payload.</i>
</p>

**Serialize a packet to bytes**

```py
import mqtt5

packet = mqtt5.ConnectPacket(client_id="Bulbasaur")
data = packet.write()
```

**Deserialize a packet from bytes**

```py
import mqtt5

buffer = bytearray(b"\x20\x03\x00\x00\x00")
packet, nbytes = mqtt5.read(buffer)
```

## Key features

- Complete MQTTv5 support (user properties, QoS, topic aliases, flow control, ...)
- Packets are serialized to minimal wire format
- Strict validation on both outgoing and incoming packets
- Fully type-annotated

## Installation

```bash
pip install mqtt5
```

Note that mqtt5 implements only the low-level packet de/serialization. If you're looking for a complete MQTT client, check out [aiomqtt](https://github.com/empicano/aiomqtt).

## Documentation

See the [stub file](https://github.com/empicano/mqtt5/blob/main/mqtt5/mqtt5.pyi) for an API reference and the [MQTTv5 specification](https://docs.oasis-open.org/mqtt/mqtt/v5.0/os/mqtt-v5.0-os.html) for details about the de/serialization.

## Versioning

This project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## Changelog

See [CHANGELOG.md](https://github.com/empicano/mqtt5/blob/main/CHANGELOG.md), which follows the principles of [Keep a Changelog](https://keepachangelog.com/en/1.1.0/).

## Acknowledgments

mqtt5 was inspired by [Cory Benfield's talk at PyCon 2016](https://www.youtube.com/watch?v=7cC3_jGwl_U). I've also learned a lot from Alex Grönholm's [mqttproto](https://github.com/agronholm/mqttproto), which is an excellent pure-Python MQTTv5 protocol implementation.
