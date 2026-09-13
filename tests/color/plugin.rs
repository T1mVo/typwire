// `include!`d into `typwire-tests-plugin`; see that crate's `src/lib.rs`.
#[typwire::export]
fn roundtrip_color(value: Color) -> Result<Color, String> {
    Ok(value)
}
