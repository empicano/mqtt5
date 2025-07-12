# Contributing

Building mqtt5 requires Rust and the uv package manager.

## Tests

The tests check for write/read consistency and validate MQTT specification compliance by comparing outputs against [mqttproto](https://github.com/agronholm/mqttproto).

You can run the tests with:

```bash
./scripts/test
```

## Benchmarks

The benchmarks use `pyperf.timeit` to avoid introducing too much overhead (e.g. Python function calls).

You can run the benchmarks with:

```bash
uv run bench.py --fast --quiet
```
