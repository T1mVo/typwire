// `include!`d into `typwire-tests-plugin`; see that crate's `src/lib.rs`.
// Exercises `#[typwire::export(rename = "...")]` combined with `format`: the
// wasm export name differs from the Rust identifier and need not be a valid
// identifier. Covers the hand-rolled protocol glue across 0, 1 and 2
// arguments.

#[typwire::export(format = "json", rename = "rename-nullary-json")]
fn renamed_nullary_json() -> Result<i64, String> {
    Ok(7)
}

#[typwire::export(format = "json", rename = "round-trip-json")]
fn renamed_length_json(value: Length) -> Result<Length, String> {
    Ok(value)
}

#[typwire::export(format = "json", rename = "add-lengths-json")]
fn renamed_add_json(first: Length, second: Length) -> Result<Length, String> {
    Ok(Length::new(first.pt() + second.pt()))
}
