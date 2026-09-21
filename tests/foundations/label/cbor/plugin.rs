// `include!`d into `typwire-tests-plugin`; see that crate's `src/lib.rs`.
#[typwire::export(rename = "roundtrip-label-cbor")]
fn roundtrip_label_cbor(value: Label) -> Result<Label, String> {
    Ok(value)
}
