// `include!`d into `typwire-tests-plugin`; see that crate's `src/lib.rs`.
// Exercises `#[typwire::export(rename = "...")]` combined with `format`: the
// wasm export name differs from the Rust identifier and need not be a valid
// identifier. Covers the hand-rolled protocol glue across 0, 1 and 2
// arguments.

#[typwire::export(rename = "rename-nullary-cbor")]
fn renamed_nullary_cbor() -> Result<i64, String> {
    Ok(7)
}

#[typwire::export(rename = "round-trip-cbor")]
fn renamed_length_cbor(value: Length) -> Result<Length, String> {
    Ok(value)
}

#[typwire::export(rename = "add-lengths-cbor")]
fn renamed_add_cbor(first: Length, second: Length) -> Result<Length, String> {
    Ok(Length::new(first.pt() + second.pt()))
}
