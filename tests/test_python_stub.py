import pytest

import tia_py


def test_import_works() -> None:
    assert tia_py.__version__ == '0.1.0'


def test_hello_from_rust() -> None:
    assert tia_py.hello() == 'hello from rust'


def test_version_matches_package() -> None:
    assert tia_py.version() == tia_py.__version__


def test_analyzer_constructible() -> None:
    analyzer = tia_py.Analyzer()
    assert repr(analyzer) == 'Analyzer()'
