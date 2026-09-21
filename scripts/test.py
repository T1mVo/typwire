#!/usr/bin/env python3
"""Build the test plugin and run the Typst roundtrip suite with Tytanic.

Tytanic has no build hook, so this script is the entry point for the suite in
`<repo>/tests`: it compiles `typwire-tests-plugin` to wasm, drops the artifact
where the tests load it from (`<repo>/target/typwire-tests/plugin.wasm`, which
is git-ignored), then invokes `tt`.

With `--release` and `wasm-opt` (from Binaryen) on PATH, the release artifact
is additionally optimized with `wasm-opt -O4 -Oz` before being placed at that
path; without `wasm-opt` installed, the plain `cargo build --release` output
is used as-is instead.

Usage:
    scripts/test.py [--release] [--no-build] [TT_ARG ...]
    scripts/test.py length ratio         # -> tt run length ratio
    scripts/test.py -e 'length or ratio' # -> tt run -e 'length or ratio'
    scripts/test.py update               # -> tt update

Arguments other than this script's own flags are forwarded to `tt`; if the
first one is not a `tt` subcommand, `run` is assumed.
"""

from __future__ import annotations

import argparse
import shutil
import subprocess
import sys
from pathlib import Path
from typing import NoReturn

ROOT = Path(__file__).resolve().parent.parent
CARGO_MANIFEST = ROOT / "src" / "core" / "Cargo.toml"
CRATE = "typwire-tests-plugin"
WASM_NAME = "typwire_tests_plugin.wasm"
TARGET_TRIPLE = "wasm32-unknown-unknown"
PLUGIN_DEST = ROOT / "target" / "typwire-tests" / "plugin.wasm"


def die(message: str) -> NoReturn:
    print(f"error: {message}", file=sys.stderr)
    raise SystemExit(1)


def require(tool: str, hint: str) -> None:
    if shutil.which(tool) is None:
        die(f"`{tool}` not found on PATH. {hint}")


def build_plugin(release: bool) -> Path:
    require("cargo", "Install the Rust toolchain from https://rustup.rs.")

    cmd = [
        "cargo",
        "build",
        "--manifest-path",
        str(CARGO_MANIFEST),
        "-p",
        CRATE,
        "--target",
        TARGET_TRIPLE,
    ]
    if release:
        cmd.append("--release")

    print(f"$ {' '.join(cmd)}")
    result = subprocess.run(cmd)
    if result.returncode != 0:
        die(
            "plugin build failed. If the target is missing, run "
            f"`rustup target add {TARGET_TRIPLE}`."
        )

    profile = "release" if release else "debug"
    artifact = (
        ROOT
        / "src"
        / "core"
        / "target"
        / TARGET_TRIPLE
        / profile
        / WASM_NAME
    )
    if not artifact.is_file():
        die(f"expected wasm artifact not found: {artifact}")
    return artifact


def optimize_with_wasm_opt(source: Path, dest: Path) -> bool:
    """Run `wasm-opt -O4 -Oz` from `source` into `dest`.

    Returns whether `wasm-opt` was found and used; the caller falls back to
    a plain copy of `source` otherwise, since `wasm-opt` is an optional,
    separately-installed tool (part of Binaryen), not a Rust toolchain
    dependency.
    """
    wasm_opt = shutil.which("wasm-opt")
    if wasm_opt is None:
        return False

    cmd = [wasm_opt, "-O4", "-Oz", str(source), "-o", str(dest)]
    print(f"$ {' '.join(cmd)}")
    result = subprocess.run(cmd)
    if result.returncode != 0:
        die("wasm-opt failed")
    return True


def main() -> int:
    parser = argparse.ArgumentParser(
        description=__doc__,
        formatter_class=argparse.RawDescriptionHelpFormatter,
    )
    parser.add_argument(
        "--release",
        action="store_true",
        help="build the plugin with the release profile",
    )
    parser.add_argument(
        "--no-build",
        dest="build",
        action="store_false",
        help="skip the cargo build and reuse the existing plugin.wasm",
    )
    parser.usage = "%(prog)s [--release] [--no-build] [TT_ARG ...]"
    args, forwarded_raw = parser.parse_known_args()

    if args.build:
        artifact = build_plugin(args.release)
        PLUGIN_DEST.parent.mkdir(parents=True, exist_ok=True)
        if args.release and optimize_with_wasm_opt(artifact, PLUGIN_DEST):
            print(
                f"optimized {artifact.relative_to(ROOT)} -> "
                f"{PLUGIN_DEST.relative_to(ROOT)} (wasm-opt -O4 -Oz)"
            )
        else:
            shutil.copy2(artifact, PLUGIN_DEST)
            print(f"copied {artifact.relative_to(ROOT)} -> {PLUGIN_DEST.relative_to(ROOT)}")
    elif not PLUGIN_DEST.is_file():
        die(f"--no-build given but {PLUGIN_DEST.relative_to(ROOT)} does not exist")

    require("tt", "Install Tytanic with `cargo install --locked tytanic`.")

    forwarded = [arg for arg in forwarded_raw if arg != "--"]
    subcommands = {"run", "update", "list", "add", "new", "remove", "util", "status"}
    if not forwarded:
        forwarded = ["run", "--no-fail-fast"]
    elif forwarded[0] not in subcommands:
        # Bare filters/options with no subcommand -> assume `run`.
        forwarded = ["run", *forwarded]

    cmd = ["tt", *forwarded]
    print(f"$ {' '.join(cmd)}")
    return subprocess.run(cmd, cwd=ROOT).returncode


if __name__ == "__main__":
    raise SystemExit(main())
