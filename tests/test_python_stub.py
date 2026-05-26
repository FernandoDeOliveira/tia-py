import pytest

import tia_py


def test_import_works() -> None:
    assert tia_py.__version__ == '0.1.0'


def test_version_requires_native() -> None:
    with pytest.raises(RuntimeError) as exc:
        tia_py.version()
    assert 'native extension is not built yet' in str(exc.value)


def test_analyzer_requires_native() -> None:
    with pytest.raises(RuntimeError) as exc:
        tia_py.Analyzer()
    assert 'native extension is not built yet' in str(exc.value)
