// `include!`d into `typwire-tests-plugin`; see that crate's `src/lib.rs`.
#[typwire::export]
fn roundtrip_char(value: char) -> Result<char, String> {
    Ok(value)
}
