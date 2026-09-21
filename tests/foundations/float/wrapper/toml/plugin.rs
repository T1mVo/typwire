// `include!`d into `typwire-tests-plugin`; see that crate's `src/lib.rs`.
#[typwire::export(format = "toml", rename = "roundtrip-float-wrapper-toml")]
fn roundtrip_float_wrapper_toml(value: typwire::Float) -> Result<typwire::Float, String> {
    Ok(value)
}
