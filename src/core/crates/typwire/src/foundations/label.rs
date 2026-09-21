use serde::{Deserialize, Serialize};

const TYPE_NAME: &str = "label";

/// A structure representing a Typst label.
///
/// `name` is the label's resolved name, the same string `str(value)` gives
/// back in Typst -- there is no `repr()` trick involved, unlike `Path`.
#[derive(Serialize, Deserialize, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
#[serde(try_from = "LabelCbor", into = "LabelCbor")]
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
struct LabelCbor {
    typwire_type: String,
    name: String,
}

// Serialize into LabelCbor
impl From<Label> for LabelCbor {
    fn from(value: Label) -> Self {
        Self {
            typwire_type: TYPE_NAME.to_string(),
            name: value.name,
        }
    }
}

// Deserialize from LabelCbor
impl TryFrom<LabelCbor> for Label {
    type Error = String;

    fn try_from(value: LabelCbor) -> Result<Self, Self::Error> {
        if value.typwire_type != TYPE_NAME {
            return Err(format!(
                "Invalid typwire-type for Label: {}",
                value.typwire_type
            ));
        }

        Ok(Self::new(value.name))
    }
}
