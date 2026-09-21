// `include!`d into `typwire-tests-plugin`; see that crate's `src/lib.rs`.
#[typwire::export(format = "yaml", rename = "roundtrip-path-yaml")]
fn roundtrip_path_yaml(value: Path) -> Result<Path, String> {
    Ok(value)
}
