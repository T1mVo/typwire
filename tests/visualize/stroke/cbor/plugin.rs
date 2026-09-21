// `include!`d into `typwire-tests-plugin`; see that crate's `src/lib.rs`.
#[typwire::export(rename = "roundtrip-stroke-cbor")]
fn roundtrip_stroke_cbor(value: Stroke) -> Result<Stroke, String> {
    Ok(value)
}
