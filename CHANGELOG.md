# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/), and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

- Improve read-path error messages

## [0.6.0] - 2026-04-16

- Implement `__repr__` (and thus `__str__`) for packets
- Store protocol name as bytes to remove Readable/Writable traits for `&str`
- Validate that `PublishPacket` topic is only empty when `topic_alias` is set
- Benchmark against zmqtt
- Skip `reason_code` on write when default and there are no properties

## [0.5.0] - 2026-03-23

- Type password field in `ConnectPacket` as `bytes` following the specification
- Make `PublishPacket` payload field required and type as `bytes`

## [0.4.0] - 2026-02-08

- Return bytes object on `write()` instead of writing to existing bytearray
- Implement `.value` and `.name` for IntEnum classes
- Return number of bytes read from `read()` instead of index
- Add `Packet` TypeAlias in custom `__init__.py`
- Test for error cases
- Fix missing user properties for `SubscribePacket`
- Pass `memoryview` instead of `bytearray` to `read()`
- Drop Python 3.10 support

## [0.3.0] - 2025-08-26

- Implement `UnsubscribePacket` and `UnsubAckPacket`
- Make integer enums comparable to `int`
- Implement user properties
- Deviate from the spec's variable names for common abbreviations
- Implement `PubRecPacket`, `PubRelPacket`, and `PubCompPacket`
- Implement `AuthPacket`

## [0.2.0] - 2025-08-17

- Move properties from nested classes to top-level attributes
- Switch properties that can only be 0 or 1 to `bool`
- Use pyo3's `rename_all` instead of specifying Python enum member names manually
- Optimize build configuration
- Implement `__str__` and `__repr__` for IntEnum classes
- Implement `PingReq` and `PingResp` packets
- Validate duplication in properties
