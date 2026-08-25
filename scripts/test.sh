#!/bin/sh

set -eu

DIR=$(cd "$(dirname "$0")" || exit 1; pwd)

cargo build --manifest-path "$DIR/../test/Cargo.toml" --target "wasm32-unknown-unknown"
cp "$DIR/../test/target/wasm32-unknown-unknown/debug/typwire_test.wasm" "$DIR/../test/src-typst/typwire_test.wasm"
typst compile --root "$DIR/.." "$DIR/../test/src-typst/roundtrip.typ"
typst compile --root "$DIR/.." "$DIR/../test/src-typst/test.typ"

if typst compile --root "$DIR/.." "$DIR/../test/src-typst/error.typ" 2>"$DIR/../test/error.log"; then
  echo "expected error.typ to fail" >&2
  exit 1
fi
grep -q "expected Rust failure" "$DIR/../test/error.log"
rm "$DIR/../test/error.log"

if typst compile --root "$DIR/.." "$DIR/../test/src-typst/invalid-tag.typ" 2>"$DIR/../test/error.log"; then
  echo "expected invalid-tag.typ to fail" >&2
  exit 1
fi
grep -q 'unknown typwire-type tag `color-future`' "$DIR/../test/error.log"
rm "$DIR/../test/error.log"

if typst compile --root "$DIR/.." "$DIR/../test/src-typst/content-error.typ" 2>"$DIR/../test/error.log"; then
  echo "expected content-error.typ to fail" >&2
  exit 1
fi
grep -q "content values are not supported" "$DIR/../test/error.log"
rm "$DIR/../test/error.log"

if typst compile --root "$DIR/.." "$DIR/../test/src-typst/named-argument.typ" 2>"$DIR/../test/error.log"; then
  echo "expected named-argument.typ to fail" >&2
  exit 1
fi
grep -q "named arguments are not supported" "$DIR/../test/error.log"
rm "$DIR/../test/error.log"
