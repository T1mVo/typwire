#!/usr/bin/env nu

use ./common.nu *
use ./package.nu *

export def main [] {
    let typst_data_dir: string = try {
        typst info e>| ansi strip | lines | str trim | where $it starts-with "Package path" | first | str replace --regex "^Package path\\s+" "" | path join local
    } catch {
        match $nu.os-info.name {
            "linux" => (
            $env.XDG_DATA_HOME?
            | default ($nu.home-dir | path join ".local" "share")
        )
            "macos" => ($nu.home-dir | path join "Library" "Application Support")
            windows => $env.APPDATA
            $other => (error make {msg: $"Unsupported platform: ($other)"})
        } | path join typst packages local
    }

    package

    cp --recursive --update ...(ls $package_dir | get name) $typst_data_dir
}
