// `include!`d into `typwire-tests-plugin`; see that crate's `src/lib.rs`.
#[typwire::export(format = "yaml", rename = "roundtrip-length-yaml")]
fn roundtrip_length_yaml(value: Length) -> Result<Length, String> {
    Ok(value)
}
