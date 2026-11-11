use serde::{Deserialize, Serialize};

use crate::Typwire;

/// A sequence of values.
///
/// Wire-identical to a bare `Vec<T>`. Typst arrays may mix types; `T` fixes the
/// element type here.
///
/// See the [Typst documentation](https://typst.app/docs/reference/foundations/array/).
///
/// # Examples
///
/// ```
/// use typwire::{Array, initiate_protocol};
///
/// # use typwire::{Cbor, __private};
/// # fn main() {}
/// initiate_protocol!();
///
/// #[typwire::export]
/// fn sum(values: Array<i64>) -> Result<i64, String> {
///     Ok(values.iter().sum())
/// }
/// ```
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
