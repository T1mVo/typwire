// `include!`d into `typwire-tests-plugin`; see that crate's `src/lib.rs`.
#[typwire::export]
fn roundtrip_alignment(value: Alignment) -> Result<Alignment, String> {
    Ok(value)
}
