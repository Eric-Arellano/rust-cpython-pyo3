#!/usr/bin/env python3.9

import os
import subprocess
import sys
import shutil
from pathlib import Path


def main() -> None:
    build_extensions()
    run_python_app()


def build_extensions() -> None:
    subprocess.run(
        ["./cargo", "build", "-p", "ffi_bridge", "-p", "pyo3_extension"],
        check=True,
    )

    pyo3_dest = Path("src/python/ffi_bridge/pyo3_extension.so")
    cpython_dest = Path("src/python/ffi_bridge/cpython_extension.so")
    pyo3_dest.unlink()
    cpython_dest.unlink()

    extension = "so" if sys.platform == "linux" else "dylib"
    shutil.copy(
        f"src/rust/ffi_bridge/target/debug/libpyo3_extension.{extension}",
        "src/python/ffi_bridge/pyo3_extension.so",
    )
    shutil.copy(
        f"src/rust/ffi_bridge/target/debug/libffi_bridge.{extension}",
        "src/python/ffi_bridge/cpython_extension.so",
    )


def run_python_app() -> None:
    result = subprocess.run(
        [sys.executable, "src/python/ffi_bridge/main.py"],
        env={**os.environ, "PYTHONPATH": "src/python"},
    )
    if result.returncode != 0:
        sys.exit(result.returncode)


if __name__ == "__main__":
    main()
