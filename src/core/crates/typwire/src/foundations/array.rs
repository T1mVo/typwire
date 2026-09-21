use serde::{Deserialize, Serialize};

use crate::Typwire;

/// A Typst `array`, wrapped as its own named generic type.
///
/// Wire-transparent: `#[serde(transparent)]` makes this encode/decode
/// exactly like a bare `Vec<T>` -- `Vec<T>` is already `Typwire` for any
/// `T: Typwire` in its own right (see `wire.rs`) and stays that way;
/// `Array<T>` exists purely so a plugin's exported function signature can
/// read `Array<T>` instead of `Vec<T>`, matching Typst's own reference type
/// name. Because the wire shape is identical, nothing on the Typst side
/// needs to know or care which one a plugin author chose.
#[derive(Serialize, Deserialize, Clone, Default, PartialEq, Debug)]
#[serde(transparent)]
pub struct Array<T>(Vec<T>);

impl<T> Array<T> {
    /// Creates a new `Array` from the given values.
    ///
    /// # Examples
    ///
    /// ```
    /// use typwire::Array;
    ///
    /// let value = Array::new(vec![1, 2, 3]);
    /// assert_eq!(&*value, &[1, 2, 3]);
    /// ```
    pub const fn new(values: Vec<T>) -> Self {
        Self(values)
    }

    /// Returns the wrapped `Vec<T>`.
    pub fn into_inner(self) -> Vec<T> {
        self.0
    }
}

impl<T: Typwire> Typwire for Array<T> {}

impl<T> From<Vec<T>> for Array<T> {
    fn from(value: Vec<T>) -> Self {
        Self::new(value)
    }
}

impl<T> From<Array<T>> for Vec<T> {
    fn from(value: Array<T>) -> Self {
        value.into_inner()
    }
}

impl<T> std::ops::Deref for Array<T> {
    type Target = Vec<T>;

    fn deref(&self) -> &Vec<T> {
        &self.0
    }
}

impl<T> std::ops::DerefMut for Array<T> {
    fn deref_mut(&mut self) -> &mut Vec<T> {
        &mut self.0
    }
}

#[cfg(test)]
mod tests {
    use crate::{decode, encode};

    use super::*;

    #[test]
    fn wire_shape_is_identical_to_a_bare_vec() {
        let value = Array::new(vec![1_i64, 2, 3]);
        let raw = vec![1_i64, 2, 3];

        assert_eq!(encode(&value).unwrap(), encode(&raw).unwrap());
        assert_eq!(decode::<Vec<i64>>(&encode(&value).unwrap()).unwrap(), raw);
        assert_eq!(decode::<Array<i64>>(&encode(&raw).unwrap()).unwrap(), value);
    }
}
