## tia-py

Static **Test Impact Analysis (TIA)** for Python: discover which tests are impacted by code changes **without coverage**, using fast static analysis.

Status: Commit #1 — Python stub only (no Rust/native extension yet).

### Install (dev)

```bash
pip install -e ".[dev]"
pytest
```
Native build (later commits)
This project will ship a native extension built with maturin + PyO3. In later commits you'll be able to run:

```bash
pip install maturin
maturin develop
pytest
```

### Naming
- Distribution name (PyPI): tia-py
- Import name: tia_py
- Native module (planned): tia_py._native