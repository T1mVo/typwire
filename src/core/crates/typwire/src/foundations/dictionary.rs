use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

use crate::Typwire;

/// A map from string keys to values.
///
/// Wire-identical to a bare `BTreeMap<String, T>`. Typst dictionaries may mix
/// value types; `T` fixes the value type here.
///
/// See the [Typst documentation](https://typst.app/docs/reference/foundations/dictionary/).
///
/// # Examples
///
/// ```
/// use typwire::{Dictionary, initiate_protocol};
///
/// # use typwire::{Cbor, __private};
/// # fn main() {}
/// initiate_protocol!();
///
/// #[typwire::export]
/// fn count(entries: Dictionary<i64>) -> Result<i64, String> {
///     Ok(entries.len() as i64)
/// }
/// ```
#[derive(Serialize, Deserialize, Clone, Default, PartialEq, Debug)]
#[serde(transparent)]
pub struct Dictionary<T>(BTreeMap<String, T>);

impl<T> Dictionary<T> {
    /// Creates a new `Dictionary` from the given entries.
    ///
    /// # Examples
    ///
    /// ```
    /// use std::collections::BTreeMap;
    ///
    /// use typwire::Dictionary;
    ///
    /// let value = Dictionary::new(BTreeMap::from([("a".to_owned(), 1)]));
    /// assert_eq!(value.get("a"), Some(&1));
    /// ```
    pub const fn new(entries: BTreeMap<String, T>) -> Self {
        Self(entries)
    }

    /// Returns the wrapped `BTreeMap<String, T>`.
    pub fn into_inner(self) -> BTreeMap<String, T> {
        self.0
    }
}

impl<T: Typwire> Typwire for Dictionary<T> {}

impl<T> From<BTreeMap<String, T>> for Dictionary<T> {
    fn from(value: BTreeMap<String, T>) -> Self {
        Self::new(value)
    }
}

impl<T> From<Dictionary<T>> for BTreeMap<String, T> {
    fn from(value: Dictionary<T>) -> Self {
        value.into_inner()
    }
}

impl<T> std::ops::Deref for Dictionary<T> {
    type Target = BTreeMap<String, T>;

    fn deref(&self) -> &BTreeMap<String, T> {
        &self.0
    }
}

impl<T> std::ops::DerefMut for Dictionary<T> {
    fn deref_mut(&mut self) -> &mut BTreeMap<String, T> {
        &mut self.0
    }
}

#[cfg(test)]
mod tests {
    use crate::{decode, encode};

    use super::*;

    #[test]
    fn wire_shape_is_identical_to_a_bare_btree_map() {
        let value = Dictionary::new(BTreeMap::from([("a".to_owned(), 1_i64)]));
        let raw = BTreeMap::from([("a".to_owned(), 1_i64)]);

        assert_eq!(encode(&value).unwrap(), encode(&raw).unwrap());
        assert_eq!(
            decode::<BTreeMap<String, i64>>(&encode(&value).unwrap()).unwrap(),
            raw
        );
        assert_eq!(
            decode::<Dictionary<i64>>(&encode(&raw).unwrap()).unwrap(),
            value
        );
    }
}
