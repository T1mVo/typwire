export const script_dir: string = path self | path dirname
export const project_dir: string = $script_dir | path dirname
export const package_dir: string = $project_dir | path join target
export const resources_dir: string = $project_dir | path join resources
export const fonts_dir: string = $project_dir | path join fonts

export const cargo_manifest_path: string = $project_dir | path join src core Cargo.toml
export const typst_manifest_path: string = $project_dir | path join typst.toml

export const test_plugin_path: string = $project_dir | path join tests target plugin.wasm
