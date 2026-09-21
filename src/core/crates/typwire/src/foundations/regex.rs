#![cfg(feature = "regex")]

use serde::{Deserialize, Serialize};

const TYPE_NAME: &str = "regex";

/// A structure representing a Typst regular expression.
///
/// Backed by [`regex_lite::Regex`]: decoding a `Regex` off the wire compiles
/// the pattern immediately, so a malformed pattern fails at decode time
/// rather than producing a value that silently can't be used as advertised.
/// `Regex` derefs to `regex_lite::Regex`, so `is_match`/`find`/`captures`/
/// etc. are all directly available.
///
/// This whole module -- the type, `Any::Regex`, and every `Typwire`/`From`
/// impl for it -- only exists when this crate's `regex` feature is enabled.
/// There's no placeholder type for the disabled case: a plugin that
/// references `Regex` without the feature gets a compile error, not a value
/// that builds fine and only fails at wire-decode time. A `regex`-tagged
/// value reaching `Any` without the feature falls through to `Any`'s normal
/// "unknown typwire-type tag" error instead of a dedicated message -- see
/// `any.rs`.
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(try_from = "RegexTypst", into = "RegexTypst")]
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
struct RegexTypst {
    typwire_type: String,
    pattern: String,
}

// Serialize into RegexTypst
impl From<Regex> for RegexTypst {
    fn from(value: Regex) -> Self {
        Self {
            typwire_type: TYPE_NAME.to_string(),
            pattern: value.0.as_str().to_string(),
        }
    }
}

// Deserialize from RegexTypst
impl TryFrom<RegexTypst> for Regex {
    type Error = String;

    fn try_from(value: RegexTypst) -> Result<Self, Self::Error> {
        if value.typwire_type != TYPE_NAME {
            return Err(format!(
                "Invalid typwire-type for Regex: {}",
                value.typwire_type
            ));
        }

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
                typwire_type: TYPE_NAME.to_string(),
                pattern: "(unclosed".to_string(),
            },
        )
        .unwrap();

        assert!(decode::<Regex>(&bytes).is_err());
    }
}
