# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

- Use pyo3's `rename_all` instead of specifying Python enum member names manually
- Implement `__str__` and `__repr__` for IntEnum classes
- Implement `PingReq` and `PingResp` packets
