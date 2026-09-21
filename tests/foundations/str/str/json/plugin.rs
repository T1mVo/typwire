// `include!`d into `typwire-tests-plugin`; see that crate's `src/lib.rs`.
#[typwire::export(format = "json", rename = "roundtrip-str-json")]
fn roundtrip_str_json(value: String) -> Result<String, String> {
    Ok(value)
}
