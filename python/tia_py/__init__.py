'''
tia-py: Static Test Impact Analysis (TIA) for Python.
'''

from __future__ import annotations

from typing import Any, NoReturn

__all__ = ['Analyzer', 'hello', 'version', '__version__']

_NATIVE_HELLO = None
_NATIVE_VERSION = None

try:
    from tia_py._native import hello as _NATIVE_HELLO
    from tia_py._native import version as _NATIVE_VERSION
except ImportError:
    pass

try:
    from tia_py._native import Analyzer
except ImportError:
    Analyzer = None

def _missing_native() -> NoReturn:
    raise RuntimeError(
        'tia-py native extension is not built yet.\n\n'
        'If you\'re developing locally, run: \n'
        '    pip install maturin\n'
        'maturin develop\n\n'
        'Then retry import tia_py / using Analyzer()'
    )

def hello() -> str:
    if _NATIVE_HELLO is None:
        _missing_native()
    return _NATIVE_HELLO()


def version() -> str:
    if _NATIVE_VERSION is None:
        _missing_native()
    return _NATIVE_VERSION()

if _NATIVE_VERSION is not None:
    __version__: str = version()
else:
    __version__: str = '0.1.0'


if Analyzer is None:
    class Analyzer:
        def __init__(self, *args: Any, **kwargs: Any) -> None:
            _missing_native()

        def __repr__(self) -> str:
            return 'Analyzer(<native-not-built>)'
