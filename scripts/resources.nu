#!/usr/bin/env nu

use ./common.nu *

export def main [] {
    glob ($resources_dir | path join "*.typ") | sort | each {|resource|
        let output: string = $resource | path parse | update extension svg | path join
        ^typst compile --root $project_dir --ignore-system-fonts --font-path $fonts_dir $resource $output
    } | ignore
}
