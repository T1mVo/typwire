// `include!`d into `typwire-tests-plugin`; see that crate's `src/lib.rs`.
#[typwire::export(rename = "roundtrip-path-cbor")]
fn roundtrip_path_cbor(value: Path) -> Result<Path, String> {
    Ok(value)
}
