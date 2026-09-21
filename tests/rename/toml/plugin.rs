// `include!`d into `typwire-tests-plugin`; see that crate's `src/lib.rs`.
// Exercises `#[typwire::export(rename = "...")]` combined with `format`: the
// wasm export name differs from the Rust identifier and need not be a valid
// identifier. Covers the hand-rolled protocol glue across 0, 1 and 2
// arguments.

#[typwire::export(format = "toml", rename = "rename-nullary-toml")]
fn renamed_nullary_toml() -> Result<i64, String> {
    Ok(7)
}

#[typwire::export(format = "toml", rename = "round-trip-toml")]
fn renamed_length_toml(value: Length) -> Result<Length, String> {
    Ok(value)
}

#[typwire::export(format = "toml", rename = "add-lengths-toml")]
fn renamed_add_toml(first: Length, second: Length) -> Result<Length, String> {
    Ok(Length::new(first.pt() + second.pt()))
}
