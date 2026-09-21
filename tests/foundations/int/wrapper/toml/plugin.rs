// `include!`d into `typwire-tests-plugin`; see that crate's `src/lib.rs`.
#[typwire::export(format = "toml", rename = "roundtrip-int-wrapper-toml")]
fn roundtrip_int_wrapper_toml(value: typwire::Int) -> Result<typwire::Int, String> {
    Ok(value)
}
