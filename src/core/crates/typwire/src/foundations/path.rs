use std::borrow::Cow;

use serde::{Deserialize, Serialize, Serializer};

use crate::tag::{Tag, Tagged};

const TYPE_NAME: &str = "path";

/// A file system path.
///
/// Always the absolute, root-resolved form (leading `/`). Shadows
/// [`std::path::Path`] if both are imported unqualified.
///
/// See the [Typst documentation](https://typst.app/docs/reference/foundations/path/).
///
/// # Examples
///
/// ```
/// use typwire::{Path, initiate_protocol};
///
/// # use typwire::{Cbor, __private};
/// # fn main() {}
/// initiate_protocol!();
///
/// #[typwire::export]
/// fn file_name(path: Path) -> Result<String, String> {
///     Ok(path.path.rsplit('/').next().unwrap_or_default().to_owned())
/// }
/// ```
#[derive(Deserialize, Clone, PartialEq, Eq, PartialOrd, Ord, Debug)]
#[serde(from = "PathTypst<'static>")]
pub struct Path {
    /// The path as a string.
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
