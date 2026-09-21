// `include!`d into `typwire-tests-plugin`; see that crate's `src/lib.rs`.
#[typwire::export]
fn roundtrip_color_space(value: ColorSpace) -> Result<ColorSpace, String> {
    Ok(value)
}
