// `include!`d into `typwire-tests-plugin`; see that crate's `src/lib.rs`.
#[typwire::export(format = "json", rename = "roundtrip-str-wrapper-json")]
fn roundtrip_str_wrapper_json(value: typwire::Str) -> Result<typwire::Str, String> {
    Ok(value)
}
