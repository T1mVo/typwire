// `include!`d into `typwire-tests-plugin`; see that crate's `src/lib.rs`.
#[typwire::export(format = "json", rename = "roundtrip-int-wrapper-json")]
fn roundtrip_int_wrapper_json(value: typwire::Int) -> Result<typwire::Int, String> {
    Ok(value)
}
