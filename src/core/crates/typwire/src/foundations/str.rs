use serde::{Deserialize, Serialize};

/// A sequence of Unicode codepoints.
///
/// Wire-identical to a bare `String`; the wrapper only names the Typst type in a
/// signature.
///
/// See the [Typst documentation](https://typst.app/docs/reference/foundations/str/).
///
/// # Examples
///
/// ```
/// use typwire::{Str, initiate_protocol};
///
/// # use typwire::{Cbor, __private};
/// # fn main() {}
/// initiate_protocol!();
///
/// #[typwire::export]
/// fn shout(text: Str) -> Result<Str, String> {
///     Ok(Str::new(text.to_uppercase()))
/// }
/// ```
#[derive(Serialize, Deserialize, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
#[serde(transparent)]
pub struct Str(String);

impl Str {
    /// Creates a new `Str` from the given value.
    ///
    /// # Examples
    ///
    /// ```
    /// use typwire::Str;
    ///
    /// let value = Str::new(String::from("hello"));
    /// assert_eq!(&*value, "hello");
    /// ```
    pub const fn new(value: String) -> Self {
        Self(value)
    }

    /// Returns the wrapped `String`.
    pub fn into_inner(self) -> String {
        self.0
    }
}

impl From<String> for Str {
    fn from(value: String) -> Self {
        Self::new(value)
    }
}

impl From<&str> for Str {
    fn from(value: &str) -> Self {
        Self::new(value.to_owned())
    }
}

impl From<Str> for String {
    fn from(value: Str) -> Self {
        value.into_inner()
    }
}

impl std::ops::Deref for Str {
    type Target = String;

    fn deref(&self) -> &String {
        &self.0
    }
}

impl std::ops::DerefMut for Str {
    fn deref_mut(&mut self) -> &mut String {
        &mut self.0
    }
}

#[cfg(test)]
mod tests {
    use crate::{decode, encode};

    use super::*;

    #[test]
    fn wire_shape_is_identical_to_a_bare_string() {
        let value = Str::from("hello");
        let raw = String::from("hello");

        assert_eq!(encode(&value).unwrap(), encode(&raw).unwrap());
        assert_eq!(decode::<String>(&encode(&value).unwrap()).unwrap(), raw);
        assert_eq!(decode::<Str>(&encode(&raw).unwrap()).unwrap(), value);
    }
}
