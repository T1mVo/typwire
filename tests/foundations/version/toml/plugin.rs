// `include!`d into `typwire-tests-plugin`; see that crate's `src/lib.rs`.
#[typwire::export(format = "toml", rename = "roundtrip-version-toml")]
fn roundtrip_version_toml(value: Version) -> Result<Version, String> {
    Ok(value)
}
