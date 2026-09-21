// `include!`d into `typwire-tests-plugin`; see that crate's `src/lib.rs`.
#[typwire::export(rename = "roundtrip-color-cbor")]
fn roundtrip_color_cbor(value: Color) -> Result<Color, String> {
    Ok(value)
}
