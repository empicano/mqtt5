
```console
$ uv run maturin develop --uv
```

```console
uv run --no-sync bench.py --quiet -o stats.json
uv run --no-sync bench.py --quiet --fast --tracemalloc -o stats.json

uv run --no-sync python chart.py

uv run pyperf stats stats.json

uv run pyperf hist stats.json
```

### Benchmark chart

- README chart done in Vega Lite: https://vega.github.io/editor
- Die Datei ist in `chart.vl.json`
- Dann direkt im Vega Lite Editor als SVG exportiert
