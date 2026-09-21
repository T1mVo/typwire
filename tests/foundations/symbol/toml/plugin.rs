// `include!`d into `typwire-tests-plugin`; see that crate's `src/lib.rs`.
#[typwire::export(format = "toml", rename = "roundtrip-symbol-toml")]
fn roundtrip_symbol_toml(value: Symbol) -> Result<Symbol, String> {
    Ok(value)
}
