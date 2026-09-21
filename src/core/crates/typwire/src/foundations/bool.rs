use serde::{Deserialize, Serialize};

/// A Typst `bool`, wrapped as its own named type.
///
/// Wire-transparent -- see [`crate::Int`]'s doc comment for what that means
/// and why.
#[derive(
    Serialize, Deserialize, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash, Debug,
)]
#[serde(transparent)]
pub struct Bool(bool);

impl Bool {
    /// Creates a new `Bool` from the given value.
    ///
    /// # Examples
    ///
    /// ```
    /// use typwire::Bool;
    ///
    /// let value = Bool::new(true);
    /// assert!(*value);
    /// ```
    pub const fn new(value: bool) -> Self {
        Self(value)
    }

    /// Returns the wrapped `bool`.
    pub const fn into_inner(self) -> bool {
        self.0
    }
}

impl From<bool> for Bool {
    fn from(value: bool) -> Self {
        Self::new(value)
    }
}

impl From<Bool> for bool {
    fn from(value: Bool) -> Self {
        value.into_inner()
    }
}

impl std::ops::Deref for Bool {
    type Target = bool;

    fn deref(&self) -> &bool {
        &self.0
    }
}

impl std::ops::DerefMut for Bool {
    fn deref_mut(&mut self) -> &mut bool {
        &mut self.0
    }
}

#[cfg(test)]
mod tests {
    use crate::{decode, encode};

    use super::*;

    #[test]
    fn wire_shape_is_identical_to_a_bare_bool() {
        assert_eq!(encode(&Bool::new(true)).unwrap(), encode(&true).unwrap());
        assert!(decode::<bool>(&encode(&Bool::new(true)).unwrap()).unwrap());
        assert_eq!(
            decode::<Bool>(&encode(&true).unwrap()).unwrap(),
            Bool::new(true)
        );
    }
}
