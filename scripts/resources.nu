#!/usr/bin/env nu

use ./common.nu *

export def main [] {
    for resource in (glob ($resources_dir | path join "*.typ") | sort) {
        let parsed = $resource | path parse
        for theme in [light dark] {
            let output: string = $parsed | update stem $"($parsed.stem)-($theme)" | update extension svg | path join
            ^typst compile --root $project_dir --ignore-system-fonts --font-path $fonts_dir --input $"theme=($theme)" $resource $output
        }
    }
}
