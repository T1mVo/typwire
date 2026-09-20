// `include!`d into `typwire-tests-plugin`; see that crate's `src/lib.rs`.
// No `use` items here (they'd collide across cases) -- fully qualified paths
// only.

#[typwire::export]
fn roundtrip_vec(value: Vec<i64>) -> Result<Vec<i64>, String> {
    Ok(value)
}

#[typwire::export]
fn roundtrip_vec_deque(
    value: std::collections::VecDeque<i64>,
) -> Result<std::collections::VecDeque<i64>, String> {
    Ok(value)
}

#[typwire::export]
fn roundtrip_linked_list(
    value: std::collections::LinkedList<i64>,
) -> Result<std::collections::LinkedList<i64>, String> {
    Ok(value)
}

#[typwire::export]
fn roundtrip_btree_set(
    value: std::collections::BTreeSet<i64>,
) -> Result<std::collections::BTreeSet<i64>, String> {
    Ok(value)
}

#[typwire::export]
fn roundtrip_hash_set(
    value: std::collections::HashSet<i64>,
) -> Result<std::collections::HashSet<i64>, String> {
    Ok(value)
}

#[typwire::export]
fn roundtrip_array(value: [i64; 3]) -> Result<[i64; 3], String> {
    Ok(value)
}

#[typwire::export]
fn roundtrip_box(value: Box<i64>) -> Result<Box<i64>, String> {
    Ok(value)
}

#[typwire::export]
fn roundtrip_btree_map(
    value: std::collections::BTreeMap<String, i64>,
) -> Result<std::collections::BTreeMap<String, i64>, String> {
    Ok(value)
}

#[typwire::export]
fn roundtrip_hash_map(
    value: std::collections::HashMap<String, i64>,
) -> Result<std::collections::HashMap<String, i64>, String> {
    Ok(value)
}

#[typwire::export]
fn roundtrip_tuple(value: (i64, String, f64)) -> Result<(i64, String, f64), String> {
    Ok(value)
}
