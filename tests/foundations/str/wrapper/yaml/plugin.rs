// `include!`d into `typwire-tests-plugin`; see that crate's `src/lib.rs`.
#[typwire::export(format = "yaml", rename = "roundtrip-str-wrapper-yaml")]
fn roundtrip_str_wrapper_yaml(value: typwire::Str) -> Result<typwire::Str, String> {
    Ok(value)
}
