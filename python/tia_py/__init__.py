'''
tia-py: Static Test Impact Analysis (TIA) for Python.
'''

from __future__ import annotations

from typing import Any, NoReturn

__all__ = ['Analyzer', 'version', '__version__']
__version__ = '0.1.0'

def _missing_native() -> NoReturn:
    raise RuntimeError(
        'tia-py native extension is not built yet.\n\n'
        'If you\'re developing locally, run: \n'
        '    pip install maturin\n'
        'maturin develop\n\n'
        'Then retry import tia_py / using Analyzer()'
    )

def version() -> str:
    # Will be provided by the native module in a later commit.
    _missing_native()
class Analyzer:
    # Will become a PyO3-exposed type in a later commit.
    def __init__(self, *args: Any, **kwargs: Any) -> None:
        _missing_native()
    def __repr__(self) -> str:
        return 'Analyzer(<native-not-built>)'
