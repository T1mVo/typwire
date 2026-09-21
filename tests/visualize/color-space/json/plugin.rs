// `include!`d into `typwire-tests-plugin`; see that crate's `src/lib.rs`.
#[typwire::export(format = "json", rename = "roundtrip-color-space-json")]
fn roundtrip_color_space_json(value: ColorSpace) -> Result<ColorSpace, String> {
    Ok(value)
}
