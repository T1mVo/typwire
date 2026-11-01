use std::borrow::Cow;

use serde::{Deserialize, Serialize, Serializer};

use crate::tag::{Tag, Tagged};

const TYPE_NAME: &str = "path";

/// A structure representing a Typst file system path.
///
/// Named to match Typst's own `path` type; shadows [`std::path::Path`] if
/// both are imported unqualified.
///
/// Always the absolute, root-resolved form (leading `/`) that Typst's own
/// `repr()` of a path uses -- that's the only form that's meaningful
/// independent of which file constructed it, since a relative path string
/// only means something relative to the file it was written in.
#[derive(Deserialize, Clone, PartialEq, Eq, PartialOrd, Ord, Debug)]
#[serde(from = "PathTypst<'static>")]
pub struct Path {
    pub path: String,
}

impl Path {
    /// Creates a new `Path` instance from the given absolute path string.
    ///
    /// # Examples
    ///
    /// ```
    /// use typwire::Path;
    ///
    /// let path = Path::new(String::from("/assets/logo.png"));
    /// assert_eq!(path.path, "/assets/logo.png");
    /// ```
    pub const fn new(path: String) -> Self {
        Self { path }
    }
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
struct PathTypst<'a> {
    typwire_type: Tag<Path>,
    path: Cow<'a, str>,
}

impl Tagged for Path {
    const TYPE_NAME: &'static str = TYPE_NAME;
}

// Serialize through a borrowing PathTypst -- `#[serde(into = ...)]`
// would clone the whole value first.
impl Serialize for Path {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        PathTypst::from(self).serialize(serializer)
    }
}

impl<'a> From<&'a Path> for PathTypst<'a> {
    fn from(value: &'a Path) -> Self {
        Self {
            typwire_type: Tag::new(),
            path: Cow::Borrowed(&value.path),
        }
    }
}

// Deserialize from PathTypst
impl From<PathTypst<'_>> for Path {
    fn from(value: PathTypst<'_>) -> Self {
        Self::new(value.path.into_owned())
    }
}
