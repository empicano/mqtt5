# mqtt5

<a href="https://pypi.org/project/mqtt5"><img alt="PyPI downloads" src="https://img.shields.io/pypi/dm/mqtt5"></a> <a href="https://pypi.org/project/mqtt5"><img alt="PyPI version" src="https://img.shields.io/pypi/v/mqtt5"></a> <a href="https://pypi.org/project/mqtt5"><img alt="Supported Python versions" src="https://img.shields.io/pypi/pyversions/mqtt5"></a>

A high-performance sans-I/O implementation of the MQTTv5 protocol for Python, written in Rust. Serialization is ~5x faster and deserialization ~20x faster than comparable Python code (benchmarked against [mqttproto](https://github.com/agronholm/mqttproto)).

<p align="center">
    <img src="https://github.com/empicano/mqtt5/blob/main/chart.svg" width="650px" />
</p>

<p align="center">
  <i>Reading/Writing a QoS=1 Publish packet with a 256 bytes payload.</i>
</p>

**Write a packet**

```py
import mqtt5

buffer = bytearray(1024)
n = mqtt5.ConnectPacket(client_id="Bulbasaur").write(buffer)
```

**Read a packet**

```py
import mqtt5

buffer = bytearray(b"\x20\x03\x00\x00\x00")
packet, n = mqtt5.read(buffer)
```

## Installation

```bash
pip install mqtt5
```

Note that mqtt5 implements only the low-level packet de/serialization. If you're looking for a complete MQTT client, check out [aiomqtt](https://github.com/empicano/aiomqtt).

## Documentation

See the [stub file](https://github.com/empicano/mqtt5/blob/main/mqtt5.pyi) for an API reference and the [MQTTv5 specification](https://docs.oasis-open.org/mqtt/mqtt/v5.0/os/mqtt-v5.0-os.html) for details about the de/serialization.

Invalid packets raise exceptions rather than being silently ignored or partially parsed.

## Versioning

This project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## Changelog

See [CHANGELOG.md](https://github.com/empicano/mqtt5/blob/main/CHANGELOG.md), which follows the principles of [Keep a Changelog](https://keepachangelog.com/en/1.1.0/).

## Acknowledgments

mqtt5 was inspired by Brett Cannon's [sans-I/O documentation](https://sans-io.readthedocs.io). I've also learned a lot from Alex Grönholm's [mqttproto](https://github.com/agronholm/mqttproto), which is an excellent pure-Python MQTTv5 protocol implementation.
