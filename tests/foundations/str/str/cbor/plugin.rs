// `include!`d into `typwire-tests-plugin`; see that crate's `src/lib.rs`.
#[typwire::export(rename = "roundtrip-str-cbor")]
fn roundtrip_str_cbor(value: String) -> Result<String, String> {
    Ok(value)
}
