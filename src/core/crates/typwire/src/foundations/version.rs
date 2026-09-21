use serde::{Deserialize, Serialize};

const TYPE_NAME: &str = "version";

/// A structure representing a Typst version.
///
/// Typst's `version(...)` constructor accepts any number of components
/// (`version()`, `version(1)`, `version(1, 2, 3, 4, 5, 6, 7)`, ... are all
/// valid) -- there is no fixed major/minor/patch/revision/build scheme, so
/// `components` is a plain `Vec<i64>` rather than named fields.
#[derive(Serialize, Deserialize, Clone, PartialEq, Eq, Debug)]
#[serde(try_from = "VersionTypst", into = "VersionTypst")]
pub struct Version {
    pub components: Vec<i64>,
}

impl Version {
    /// Creates a new `Version` from the given components.
    ///
    /// # Examples
    ///
    /// ```
    /// use typwire::Version;
    ///
    /// let version = Version::new(vec![1, 2, 3]);
    /// assert_eq!(version.components, vec![1, 2, 3]);
    /// ```
    pub const fn new(components: Vec<i64>) -> Self {
        Self { components }
    }
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
struct VersionTypst {
    typwire_type: String,
    components: Vec<i64>,
}

// Serialize into VersionTypst
impl From<Version> for VersionTypst {
    fn from(value: Version) -> Self {
        Self {
            typwire_type: TYPE_NAME.to_string(),
            components: value.components,
        }
    }
}

// Deserialize from VersionTypst
impl TryFrom<VersionTypst> for Version {
    type Error = String;

    fn try_from(value: VersionTypst) -> Result<Self, Self::Error> {
        if value.typwire_type != TYPE_NAME {
            return Err(format!(
                "Invalid typwire-type for Version: {}",
                value.typwire_type
            ));
        }

        Ok(Self::new(value.components))
    }
}
