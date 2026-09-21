// `include!`d into `typwire-tests-plugin`; see that crate's `src/lib.rs`.
#[typwire::export(rename = "roundtrip-color-space-cbor")]
fn roundtrip_color_space_cbor(value: ColorSpace) -> Result<ColorSpace, String> {
    Ok(value)
}
