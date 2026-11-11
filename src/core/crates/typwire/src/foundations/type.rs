use std::borrow::Cow;

use serde::{Deserialize, Serialize, Serializer};

use crate::tag::{Tag, Tagged};

const TYPE_NAME: &str = "type";

/// Describes a kind of value, such as `int` or `str`.
///
/// See the [Typst documentation](https://typst.app/docs/reference/foundations/type/).
///
/// # Examples
///
/// ```
/// use typwire::{Type, initiate_protocol};
///
/// # use typwire::{Cbor, __private};
/// # fn main() {}
/// initiate_protocol!();
///
/// #[typwire::export]
/// fn type_name(ty: Type) -> Result<String, String> {
///     Ok(ty.ty)
/// }
/// ```
#[derive(Deserialize, Clone, PartialEq, Eq, PartialOrd, Ord, Debug)]
#[serde(from = "TypeTypst<'static>")]
pub struct Type {
    pub ty: String,
}

impl Type {
    /// Creates a new `Type` instance from the given string.
    ///
    /// # Arguments
    ///
    /// * `r#type` - A string representing the type.
    ///
    /// # Examples
    ///
    /// ```
    /// use typwire::Type;
    ///
    /// let custom_type = Type::new(String::from("CustomType"));
    /// assert_eq!(custom_type.ty, "CustomType");
    /// ```
    pub const fn new(ty: String) -> Self {
        Self { ty }
    }
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
struct TypeTypst<'a> {
    typwire_type: Tag<Type>,
    ty: Cow<'a, str>,
}

impl Tagged for Type {
    const TYPE_NAME: &'static str = TYPE_NAME;
}

// Serialize through a borrowing TypeTypst -- `#[serde(into = ...)]`
// would clone the whole value first.
impl Serialize for Type {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        TypeTypst::from(self).serialize(serializer)
    }
}

impl<'a> From<&'a Type> for TypeTypst<'a> {
    fn from(value: &'a Type) -> Self {
        Self {
            typwire_type: Tag::new(),
            ty: Cow::Borrowed(&value.ty),
        }
    }
}

// Deserialize from TypeTypst
impl From<TypeTypst<'_>> for Type {
    fn from(value: TypeTypst<'_>) -> Self {
        Self::new(value.ty.into_owned())
    }
}
