# tia-py

Static **Test Impact Analysis (TIA)** for Python: discover which tests are likely affected by code changes **without running coverage**, using fast static analysis.

> **Status:** Phase 0 complete — project scaffolding, maturin + PyO3 bindings, and a minimal public API (`version`, `Analyzer`). No AST parsing or impact logic yet.

## Goals

- Static analysis (no coverage execution required)
- Fast enough for large monorepos
- Incremental cache (planned)
- CI/CD integration (planned)
- Friendly Python API

## Requirements

- Python ≥ 3.9
- Rust ≥ 1.75 ([rustup](https://rustup.rs/))
- [maturin](https://www.maturin.rs/) (installed in your virtualenv)

## Quick start (development)

```bash
# From the repository root
python3 -m venv .venv
source .venv/bin/activate   # Windows: .venv\Scripts\activate

pip install -U pip maturin pytest
pip install -e ".[dev]"

maturin develop
python -m pytest
```

Use **`python -m pytest`** (not bare `pytest`) so tests run with the same interpreter as your venv. On some setups, a global `pytest` shim (e.g. pyenv) does not see the installed package.

### Smoke test

```bash
python -c "import tia_py; print(tia_py.version()); print(tia_py.Analyzer())"
```

Expected output:

```text
0.1.0
Analyzer()
```

## Usage (current API)

```python
import tia_py

tia_py.__version__   # "0.1.0" (from native extension when built)
tia_py.version()     # same as __version__

analyzer = tia_py.Analyzer()  # placeholder — no analysis yet
```

If the native extension is not built, `version()` and `Analyzer()` raise a `RuntimeError` with instructions to run `maturin develop`.

## Architecture

```text
Python API (tia_py)
    ↓
PyO3 bindings (tia_py._native)
    ↓
Rust analysis engine (future)
    ↓
AST parsing + dependency graph (future)
    ↓
Impact analysis engine (future)
```

## Repository layout

| Path | Role |
|------|------|
| `python/tia_py/` | Public Python package |
| `python/tia_py/_native*.so` | Compiled extension (local build artifact, gitignored) |
| `src/` | Rust crate (`lib.rs`, PyO3 bindings) |
| `src/bindings/` | Types and functions exposed to Python |
| `tests/` | pytest integration tests |
| `examples/` | Usage examples (future) |
| `benches/` | Benchmarks (future) |

## Naming

| Context | Name |
|---------|------|
| Distribution / PyPI | `tia-py` |
| `import` | `tia_py` |
| Native module | `tia_py._native` |
| Rust package | `tia-py` |
| Rust library | `tia_py` |

## Tech stack (planned / in use)

| Component | Phase 0 | Later |
|-----------|---------|-------|
| Rust + PyO3 + maturin | ✓ | |
| tree-sitter-python | | ✓ |
| petgraph | | ✓ |
| rayon, serde, anyhow, tracing | | ✓ |

## Running Rust tests

```bash
cargo test
```

## License

MIT
