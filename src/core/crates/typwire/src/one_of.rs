use serde::{Deserialize, Serialize};

use crate::Typwire;

macro_rules! one_of {
    ($name:ident, $(($variant:ident, $type:ident, $constructor:ident)),+ $(,)?) => {
        #[doc = "An ordered set of alternative wire representations."]
        #[derive(Clone, Debug, Serialize, Deserialize)]
        #[serde(untagged)]
        pub enum $name<$($type),+> {
            $($variant($type)),+
        }

        impl<$($type: Typwire),+> Typwire for $name<$($type),+> {}

        impl<$($type),+> $name<$($type),+> {
            $(
                pub fn $constructor(value: $type) -> Self {
                    Self::$variant(value)
                }
            )+
        }
    };
}

one_of!(OneOf2, (First, A, first), (Second, B, second));
one_of!(
    OneOf3,
    (First, A, first),
    (Second, B, second),
    (Third, C, third)
);
one_of!(
    OneOf4,
    (First, A, first),
    (Second, B, second),
    (Third, C, third),
    (Fourth, D, fourth)
);
one_of!(
    OneOf5,
    (First, A, first),
    (Second, B, second),
    (Third, C, third),
    (Fourth, D, fourth),
    (Fifth, E, fifth)
);
one_of!(
    OneOf6,
    (First, A, first),
    (Second, B, second),
    (Third, C, third),
    (Fourth, D, fourth),
    (Fifth, E, fifth),
    (Sixth, F, sixth)
);
one_of!(
    OneOf7,
    (First, A, first),
    (Second, B, second),
    (Third, C, third),
    (Fourth, D, fourth),
    (Fifth, E, fifth),
    (Sixth, F, sixth),
    (Seventh, G, seventh)
);
one_of!(
    OneOf8,
    (First, A, first),
    (Second, B, second),
    (Third, C, third),
    (Fourth, D, fourth),
    (Fifth, E, fifth),
    (Sixth, F, sixth),
    (Seventh, G, seventh),
    (Eighth, H, eighth)
);

#[cfg(test)]
mod tests {
    use crate::{decode, encode};

    use super::*;

    #[test]
    fn chooses_first_compatible_variant() {
        let bytes = encode(&42_i64).unwrap();
        let decoded: OneOf2<i64, f64> = decode(&bytes).unwrap();
        assert!(matches!(decoded, OneOf2::First(42)));
    }

    #[test]
    fn falls_through_in_order() {
        let bytes = encode(&"value".to_owned()).unwrap();
        let decoded: OneOf3<i64, bool, String> = decode(&bytes).unwrap();
        assert!(matches!(decoded, OneOf3::Third(value) if value == "value"));
    }
}
