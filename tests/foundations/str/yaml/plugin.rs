// `include!`d into `typwire-tests-plugin`; see that crate's `src/lib.rs`.
#[typwire::export(format = "yaml", rename = "roundtrip-str-yaml")]
fn roundtrip_str_yaml(value: String) -> Result<String, String> {
    Ok(value)
}
