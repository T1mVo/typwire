// `include!`d into `typwire-tests-plugin`; see that crate's `src/lib.rs`.
#[typwire::export(format = "toml", rename = "roundtrip-path-toml")]
fn roundtrip_path_toml(value: Path) -> Result<Path, String> {
    Ok(value)
}
