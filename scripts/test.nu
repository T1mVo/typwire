#!/usr/bin/env nu

use ./common.nu *

const build_plugin_path: string = $project_dir | path join "src" "core" "target" "wasm32-unknown-unknown" "release" "typwire_tests_plugin.wasm"

export def --wrapped main [...rest] {
    cargo build --manifest-path $cargo_manifest_path --package typwire-tests-plugin --target wasm32-unknown-unknown --release

    $test_plugin_path | path dirname | mkdir $in
    wasm-opt -O4 --enable-bulk-memory-opt --enable-simd $build_plugin_path -o $test_plugin_path

    tt run ...$rest
}
