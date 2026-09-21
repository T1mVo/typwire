// `include!`d into `typwire-tests-plugin`; see that crate's `src/lib.rs`.
#[typwire::export(format = "toml", rename = "roundtrip-length-toml")]
fn roundtrip_length_toml(value: Length) -> Result<Length, String> {
    Ok(value)
}
