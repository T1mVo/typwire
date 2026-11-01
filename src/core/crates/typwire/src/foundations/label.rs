use std::borrow::Cow;

use serde::{Deserialize, Serialize, Serializer};

use crate::tag::{Tag, Tagged};

const TYPE_NAME: &str = "label";

/// A structure representing a Typst label.
///
/// `name` is the label's resolved name, the same string `str(value)` gives
/// back in Typst -- there is no `repr()` trick involved, unlike `Path`.
#[derive(Deserialize, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
#[serde(from = "LabelTypst<'static>")]
pub struct Label {
    pub name: String,
}

impl Label {
    /// Creates a new `Label` instance from the given name.
    ///
    /// # Examples
    ///
    /// ```
    /// use typwire::Label;
    ///
    /// let label = Label::new(String::from("intro"));
    /// assert_eq!(label.name, "intro");
    /// ```
    pub const fn new(name: String) -> Self {
        Self { name }
    }
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
struct LabelTypst<'a> {
    typwire_type: Tag<Label>,
    name: Cow<'a, str>,
}

impl Tagged for Label {
    const TYPE_NAME: &'static str = TYPE_NAME;
}

// Serialize through a borrowing LabelTypst -- `#[serde(into = ...)]`
// would clone the whole value first.
impl Serialize for Label {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        LabelTypst::from(self).serialize(serializer)
    }
}

impl<'a> From<&'a Label> for LabelTypst<'a> {
    fn from(value: &'a Label) -> Self {
        Self {
            typwire_type: Tag::new(),
            name: Cow::Borrowed(&value.name),
        }
    }
}

// Deserialize from LabelTypst
impl From<LabelTypst<'_>> for Label {
    fn from(value: LabelTypst<'_>) -> Self {
        Self::new(value.name.into_owned())
    }
}
