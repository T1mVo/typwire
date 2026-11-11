use std::borrow::Cow;

use serde::{Deserialize, Serialize, Serializer};

use crate::tag::{Tag, Tagged};

const TYPE_NAME: &str = "label";

/// A label for an element.
///
/// `name` is the same string `str(label)` gives in Typst.
///
/// See the [Typst documentation](https://typst.app/docs/reference/foundations/label/).
///
/// # Examples
///
/// ```
/// use typwire::{Label, initiate_protocol};
///
/// # use typwire::{Cbor, __private};
/// # fn main() {}
/// initiate_protocol!();
///
/// #[typwire::export]
/// fn name(label: Label) -> Result<String, String> {
///     Ok(label.name)
/// }
/// ```
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
