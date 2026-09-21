// `include!`d into `typwire-tests-plugin`; see that crate's `src/lib.rs`.
#[typwire::export]
fn roundtrip_symbol(value: Symbol) -> Result<Symbol, String> {
    Ok(value)
}
