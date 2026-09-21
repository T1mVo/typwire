use serde::{Deserialize, Serialize};

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
#[derive(Serialize, Deserialize, Clone, PartialEq, Eq, PartialOrd, Ord, Debug)]
#[serde(try_from = "PathTypst", into = "PathTypst")]
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
struct PathTypst {
    typwire_type: String,
    path: String,
}

// Serialize into PathTypst
impl From<Path> for PathTypst {
    fn from(value: Path) -> Self {
        Self {
            typwire_type: TYPE_NAME.to_string(),
            path: value.path,
        }
    }
}

// Deserialize from PathTypst
impl TryFrom<PathTypst> for Path {
    type Error = String;

    fn try_from(value: PathTypst) -> Result<Self, Self::Error> {
        if value.typwire_type != TYPE_NAME {
            return Err(format!(
                "Invalid typwire-type for Path: {}",
                value.typwire_type
            ));
        }

        Ok(Self::new(value.path))
    }
}
