// `include!`d into `typwire-tests-plugin`; see that crate's `src/lib.rs`.
#[typwire::export]
fn roundtrip_version(value: Version) -> Result<Version, String> {
    Ok(value)
}
