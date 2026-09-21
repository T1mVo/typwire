// `include!`d into `typwire-tests-plugin`; see that crate's `src/lib.rs`.
#[typwire::export]
fn roundtrip_path(value: Path) -> Result<Path, String> {
    Ok(value)
}
