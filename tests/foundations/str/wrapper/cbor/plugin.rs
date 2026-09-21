// `include!`d into `typwire-tests-plugin`; see that crate's `src/lib.rs`.
#[typwire::export(rename = "roundtrip-str-wrapper-cbor")]
fn roundtrip_str_wrapper_cbor(value: typwire::Str) -> Result<typwire::Str, String> {
    Ok(value)
}
