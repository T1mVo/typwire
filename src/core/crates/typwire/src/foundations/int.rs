use serde::{Deserialize, Serialize};

/// A Typst `int`, wrapped as its own named type.
///
/// Wire-transparent: `#[serde(transparent)]` makes this encode/decode
/// exactly like a bare `i64` -- `i64` is already `Typwire` in its own right
/// (see `wire.rs`) and stays that way; `Int` exists purely so a plugin's
/// exported function signature can read `Int` instead of `i64`, matching
/// Typst's own reference type name. Because the wire shape is identical,
/// nothing on the Typst side needs to know or care which one a plugin
/// author chose.
#[derive(
    Serialize, Deserialize, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash, Debug,
)]
#[serde(transparent)]
pub struct Int(i64);

impl Int {
    /// Creates a new `Int` from the given value.
    ///
    /// # Examples
    ///
    /// ```
    /// use typwire::Int;
    ///
    /// let value = Int::new(42);
    /// assert_eq!(*value, 42);
    /// ```
    pub const fn new(value: i64) -> Self {
        Self(value)
    }

    /// Returns the wrapped `i64`.
    pub const fn into_inner(self) -> i64 {
        self.0
    }
}

impl From<i64> for Int {
    fn from(value: i64) -> Self {
        Self::new(value)
    }
}

impl From<Int> for i64 {
    fn from(value: Int) -> Self {
        value.into_inner()
    }
}

impl std::ops::Deref for Int {
    type Target = i64;

    fn deref(&self) -> &i64 {
        &self.0
    }
}

impl std::ops::DerefMut for Int {
    fn deref_mut(&mut self) -> &mut i64 {
        &mut self.0
    }
}

#[cfg(test)]
mod tests {
    use crate::{decode, encode};

    use super::*;

    #[test]
    fn wire_shape_is_identical_to_a_bare_i64() {
        assert_eq!(encode(&Int::new(7)).unwrap(), encode(&7_i64).unwrap());
        assert_eq!(decode::<i64>(&encode(&Int::new(7)).unwrap()).unwrap(), 7);
        assert_eq!(
            decode::<Int>(&encode(&7_i64).unwrap()).unwrap(),
            Int::new(7)
        );
    }
}
