// `include!`d into `typwire-tests-plugin`; see that crate's `src/lib.rs`.
// Exercises `#[typwire::export(rename = "...")]` combined with `format`: the
// wasm export name differs from the Rust identifier and need not be a valid
// identifier. Covers the hand-rolled protocol glue across 0, 1 and 2
// arguments.

#[typwire::export(format = "yaml", rename = "rename-nullary-yaml")]
fn renamed_nullary_yaml() -> Result<i64, String> {
    Ok(7)
}

#[typwire::export(format = "yaml", rename = "round-trip-yaml")]
fn renamed_length_yaml(value: Length) -> Result<Length, String> {
    Ok(value)
}

#[typwire::export(format = "yaml", rename = "add-lengths-yaml")]
fn renamed_add_yaml(first: Length, second: Length) -> Result<Length, String> {
    Ok(Length::new(first.pt() + second.pt()))
}
