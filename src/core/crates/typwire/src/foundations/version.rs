use std::borrow::Cow;

use serde::{Deserialize, Serialize, Serializer};

use crate::tag::{Tag, Tagged};

const TYPE_NAME: &str = "version";

/// A structure representing a Typst version.
///
/// Typst's `version(...)` constructor accepts any number of components
/// (`version()`, `version(1)`, `version(1, 2, 3, 4, 5, 6, 7)`, ... are all
/// valid) -- there is no fixed major/minor/patch/revision/build scheme, so
/// `components` is a plain `Vec<i64>` rather than named fields.
#[derive(Deserialize, Clone, PartialEq, Eq, Debug)]
#[serde(from = "VersionTypst<'static>")]
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
struct VersionTypst<'a> {
    typwire_type: Tag<Version>,
    components: Cow<'a, [i64]>,
}

impl Tagged for Version {
    const TYPE_NAME: &'static str = TYPE_NAME;
}

// Serialize through a borrowing VersionTypst -- `#[serde(into = ...)]`
// would clone the whole value first.
impl Serialize for Version {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        VersionTypst::from(self).serialize(serializer)
    }
}

impl<'a> From<&'a Version> for VersionTypst<'a> {
    fn from(value: &'a Version) -> Self {
        Self {
            typwire_type: Tag::new(),
            components: Cow::Borrowed(&value.components),
        }
    }
}

// Deserialize from VersionTypst
impl From<VersionTypst<'_>> for Version {
    fn from(value: VersionTypst<'_>) -> Self {
        Self::new(value.components.into_owned())
    }
}
