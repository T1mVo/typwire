// `include!`d into `typwire-tests-plugin`; see that crate's `src/lib.rs`.
#[typwire::export(rename = "roundtrip-alignment-cbor")]
fn roundtrip_alignment_cbor(value: Alignment) -> Result<Alignment, String> {
    Ok(value)
}
