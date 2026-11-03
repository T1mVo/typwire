#!/usr/bin/env nu

use ./common.nu *

def main [] {
    cargo clean --manifest-path $cargo_manifest_path e>| ignore
    rm --recursive --permanent --force $package_dir
}
