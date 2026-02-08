# How to contribute

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

### Benchmark chart

The chart in the README is generated from the [Vega Lite](https://vega.github.io/editor) spec in `chart.vl.json`. To update it:

1. Run `uv run bench.py --quiet --compare --packets "publish(qos1"` to get the latest numbers
2. Update the values in `chart.vl.json`
3. Paste the spec into the [Vega Lite Editor](https://vega.github.io/editor) and export as SVG

## Release

1. Adjust the version in `Cargo.toml`
1. Update the `CHANGELOG.md`
1. Create a new release on GitHub
