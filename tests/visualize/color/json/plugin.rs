// `include!`d into `typwire-tests-plugin`; see that crate's `src/lib.rs`.
#[typwire::export(format = "json", rename = "roundtrip-color-json")]
fn roundtrip_color_json(value: Color) -> Result<Color, String> {
    Ok(value)
}
