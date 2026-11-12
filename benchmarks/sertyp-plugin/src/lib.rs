use sertyp::typst_func;
use wasm_minimal_protocol::*;

#[cfg(target_arch = "wasm32")]
initiate_protocol!();

#[typst_func]
pub fn bench<'a>(value: sertyp::Item<'a>) -> sertyp::Item<'a> {
    value
}

#[typst_func]
pub fn ints(value: sertyp::TypedArray<sertyp::Integer>) -> sertyp::TypedArray<sertyp::Integer> {
    value
}

#[typst_func]
pub fn string<'a>(value: sertyp::String<'a>) -> sertyp::String<'a> {
    value
}

#[typst_func]
pub fn bytes<'a>(value: sertyp::Bytes<'a>) -> sertyp::Bytes<'a> {
    value
}

#[typst_func]
pub fn objects<'a>(
    value: sertyp::TypedArray<sertyp::Dictionary<'a>>,
) -> sertyp::TypedArray<sertyp::Dictionary<'a>> {
    value
}

#[typst_func]
pub fn natives<'a>(
    value: sertyp::TypedArray<sertyp::Array<'a>>,
) -> sertyp::TypedArray<sertyp::Array<'a>> {
    value
}

#[typst_func]
pub fn gradient<'a>(value: Box<sertyp::Gradient<'a>>) -> Box<sertyp::Gradient<'a>> {
    value
}

#[typst_func]
pub fn object<'a>(value: sertyp::Dictionary<'a>) -> sertyp::Dictionary<'a> {
    value
}
