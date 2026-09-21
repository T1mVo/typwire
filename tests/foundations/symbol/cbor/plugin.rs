// `include!`d into `typwire-tests-plugin`; see that crate's `src/lib.rs`.
#[typwire::export(rename = "roundtrip-symbol-cbor")]
fn roundtrip_symbol_cbor(value: Symbol) -> Result<Symbol, String> {
    Ok(value)
}
