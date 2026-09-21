// `include!`d into `typwire-tests-plugin`; see that crate's `src/lib.rs`.
#[typwire::export(format = "json", rename = "roundtrip-path-json")]
fn roundtrip_path_json(value: Path) -> Result<Path, String> {
    Ok(value)
}
