// `include!`d into `typwire-tests-plugin`; see that crate's `src/lib.rs`.
#[typwire::export(format = "yaml", rename = "roundtrip-version-yaml")]
fn roundtrip_version_yaml(value: Version) -> Result<Version, String> {
    Ok(value)
}
