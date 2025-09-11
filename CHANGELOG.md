# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

- Return bytes object on `write()` instead of writing to existing bytearray
- Implement `.value` and `.name` for IntEnum classes
- Return number of bytes read from `read()` instead of index
- Add `Packet` TypeAlias in custom `__init__.py`
- Test for error cases

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
