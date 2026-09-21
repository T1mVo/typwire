// `include!`d into `typwire-tests-plugin`; see that crate's `src/lib.rs`.
#[typwire::export(format = "yaml", rename = "roundtrip-symbol-yaml")]
fn roundtrip_symbol_yaml(value: Symbol) -> Result<Symbol, String> {
    Ok(value)
}
