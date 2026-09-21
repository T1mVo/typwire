// `include!`d into `typwire-tests-plugin`; see that crate's `src/lib.rs`.
#[typwire::export(format = "json", rename = "roundtrip-symbol-json")]
fn roundtrip_symbol_json(value: Symbol) -> Result<Symbol, String> {
    Ok(value)
}
