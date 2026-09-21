// `include!`d into `typwire-tests-plugin`; see that crate's `src/lib.rs`.
#[typwire::export(format = "json", rename = "roundtrip-version-json")]
fn roundtrip_version_json(value: Version) -> Result<Version, String> {
    Ok(value)
}
