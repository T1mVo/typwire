use serde::{Deserialize, Serialize};

/// A Typst `float`, wrapped as its own named type.
///
/// Wire-transparent -- see [`crate::Int`]'s doc comment for what that means
/// and why. No `Eq`/`Ord`/`Hash`, for the same reason `f64` itself doesn't
/// have them (`NaN`).
#[derive(Serialize, Deserialize, Clone, Copy, Default, PartialEq, PartialOrd, Debug)]
#[serde(transparent)]
pub struct Float(f64);

impl Float {
    /// Creates a new `Float` from the given value.
    ///
    /// # Examples
    ///
    /// ```
    /// use typwire::Float;
    ///
    /// let value = Float::new(4.2);
    /// assert_eq!(*value, 4.2);
    /// ```
    pub const fn new(value: f64) -> Self {
        Self(value)
    }

    /// Returns the wrapped `f64`.
    pub const fn into_inner(self) -> f64 {
        self.0
    }
}

impl From<f64> for Float {
    fn from(value: f64) -> Self {
        Self::new(value)
    }
}

impl From<Float> for f64 {
    fn from(value: Float) -> Self {
        value.into_inner()
    }
}

impl std::ops::Deref for Float {
    type Target = f64;

    fn deref(&self) -> &f64 {
        &self.0
    }
}

impl std::ops::DerefMut for Float {
    fn deref_mut(&mut self) -> &mut f64 {
        &mut self.0
    }
}

#[cfg(test)]
mod tests {
    use crate::{decode, encode};

    use super::*;

    #[test]
    fn wire_shape_is_identical_to_a_bare_f64() {
        assert_eq!(encode(&Float::new(7.5)).unwrap(), encode(&7.5_f64).unwrap());
        assert_eq!(
            decode::<f64>(&encode(&Float::new(7.5)).unwrap()).unwrap(),
            7.5
        );
        assert_eq!(
            decode::<Float>(&encode(&7.5_f64).unwrap()).unwrap(),
            Float::new(7.5)
        );
    }
}
