#![cfg(feature = "regex")]

use std::borrow::Cow;

use serde::{Deserialize, Serialize, Serializer};

use crate::tag::{Tag, Tagged};

const TYPE_NAME: &str = "regex";

/// A regular expression.
///
/// Backed by [`regex_lite::Regex`] and derefs to it. The pattern is compiled on
/// decode, so a malformed one fails there. Only available with the `regex`
/// feature.
///
/// See the [Typst documentation](https://typst.app/docs/reference/foundations/regex/).
///
/// # Examples
///
/// ```
/// use typwire::{Regex, initiate_protocol};
///
/// # use typwire::{Cbor, __private};
/// # fn main() {}
/// initiate_protocol!();
///
/// #[typwire::export]
/// fn matches(pattern: Regex, text: String) -> Result<bool, String> {
///     Ok(pattern.is_match(&text))
/// }
/// ```
#[derive(Clone, Debug, Deserialize)]
#[serde(try_from = "RegexTypst<'static>")]
pub struct Regex(regex_lite::Regex);

impl Regex {
    /// Compiles a new `Regex` from the given pattern.
    ///
    /// # Examples
    ///
    /// ```
    /// use typwire::Regex;
    ///
    /// let re = Regex::new("a.*b").unwrap();
    /// assert!(re.is_match("axxb"));
    /// ```
    pub fn new(pattern: &str) -> Result<Self, regex_lite::Error> {
        regex_lite::Regex::new(pattern).map(Self)
    }
}

impl std::ops::Deref for Regex {
    type Target = regex_lite::Regex;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl PartialEq for Regex {
    fn eq(&self, other: &Self) -> bool {
        self.0.as_str() == other.0.as_str()
    }
}

impl Eq for Regex {}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
struct RegexTypst<'a> {
    typwire_type: Tag<Regex>,
    pattern: Cow<'a, str>,
}

impl Tagged for Regex {
    const TYPE_NAME: &'static str = TYPE_NAME;
}

// Serialize through a borrowing RegexTypst -- `#[serde(into = ...)]`
// would clone the whole value first.
impl Serialize for Regex {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        RegexTypst::from(self).serialize(serializer)
    }
}

impl<'a> From<&'a Regex> for RegexTypst<'a> {
    fn from(value: &'a Regex) -> Self {
        Self {
            typwire_type: Tag::new(),
            pattern: Cow::Borrowed(value.0.as_str()),
        }
    }
}

// Deserialize from RegexTypst
impl TryFrom<RegexTypst<'_>> for Regex {
    type Error = String;

    fn try_from(value: RegexTypst<'_>) -> Result<Self, Self::Error> {
        Regex::new(&value.pattern).map_err(|error| format!("invalid regex pattern: {error}"))
    }
}

#[cfg(test)]
mod tests {
    use crate::decode;

    use super::*;

    #[test]
    fn invalid_pattern_fails_to_decode() {
        // Bypasses `Regex::new`'s own validation on purpose, to exercise
        // `TryFrom<RegexTypst>`'s.
        let bytes = cbor4ii::serde::to_vec(
            Vec::new(),
            &RegexTypst {
                typwire_type: Tag::new(),
                pattern: Cow::Borrowed("(unclosed"),
            },
        )
        .unwrap();

        assert!(decode::<Regex>(&bytes).is_err());
    }
}
