# Contributing

Building mqtt5 requires Rust and the uv package manager.

## Tests

The tests check for write/read consistency and validate MQTT specification compliance by comparing outputs against [mqttproto](https://github.com/agronholm/mqttproto).

You can run the tests with:

```bash
./scripts/test
```

## Benchmarks

The benchmarks use `pyperf.timeit` to avoid introducing unnecessary overhead (e.g. additional Python function calls).

You can run the benchmarks with:

```bash
uv run bench.py --fast --quiet
```

Add the `--compare` flag to benchmark against [mqttproto](https://github.com/agronholm/mqttproto). Add the `--packets` argument to run the benchmarks only for certain packets e.g. `--packets publish --packets puback`.

## Release

Adjust the package's version in `Cargo.toml`, then create a new release on GitHub.
