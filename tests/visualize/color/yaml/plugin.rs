// `include!`d into `typwire-tests-plugin`; see that crate's `src/lib.rs`.
#[typwire::export(format = "yaml", rename = "roundtrip-color-yaml")]
fn roundtrip_color_yaml(value: Color) -> Result<Color, String> {
    Ok(value)
}
