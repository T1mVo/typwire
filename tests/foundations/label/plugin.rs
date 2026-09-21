// `include!`d into `typwire-tests-plugin`; see that crate's `src/lib.rs`.
#[typwire::export]
fn roundtrip_label(value: Label) -> Result<Label, String> {
    Ok(value)
}
