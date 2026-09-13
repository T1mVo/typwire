// `include!`d into `typwire-tests-plugin`; see that crate's `src/lib.rs`.
// Exercises `#[typwire::export(rename = "...")]`: the wasm export name differs
// from the Rust identifier and need not be a valid identifier. Covers the
// hand-rolled protocol glue across 0, 1 and 2 arguments.

#[typwire::export(rename = "rename-nullary")]
fn renamed_nullary() -> Result<i64, String> {
    Ok(7)
}

#[typwire::export(rename = "round-trip")]
fn renamed_length(value: Length) -> Result<Length, String> {
    Ok(value)
}

#[typwire::export(rename = "add-lengths")]
fn renamed_add(first: Length, second: Length) -> Result<Length, String> {
    Ok(Length::new(first.pt() + second.pt()))
}
