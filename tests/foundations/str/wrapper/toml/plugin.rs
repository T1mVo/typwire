// `include!`d into `typwire-tests-plugin`; see that crate's `src/lib.rs`.
#[typwire::export(format = "toml", rename = "roundtrip-str-wrapper-toml")]
fn roundtrip_str_wrapper_toml(value: typwire::Str) -> Result<typwire::Str, String> {
    Ok(value)
}
