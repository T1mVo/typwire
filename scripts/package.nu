#!/usr/bin/env nu

use ./common.nu *

export def main [] {
    let version: string = open $typst_manifest_path | get package.version
    let package_dir: string = $package_dir | path join typwire $version

    mkdir $package_dir

    cp --update LICENSE-APACHE LICENSE-MIT README.md typst.toml $package_dir

    let package_src_dir: string = $package_dir | path join src
    mkdir $package_src_dir
    ls src | where name !~ core | each {|path| cp --recursive --update $path.name $package_src_dir } | ignore
}
