// `include!`d into `typwire-tests-plugin`; see that crate's `src/lib.rs`.
#[typwire::export(rename = "roundtrip-version-cbor")]
fn roundtrip_version_cbor(value: Version) -> Result<Version, String> {
    Ok(value)
}
