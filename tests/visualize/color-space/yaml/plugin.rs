// `include!`d into `typwire-tests-plugin`; see that crate's `src/lib.rs`.
#[typwire::export(format = "yaml", rename = "roundtrip-color-space-yaml")]
fn roundtrip_color_space_yaml(value: ColorSpace) -> Result<ColorSpace, String> {
    Ok(value)
}
