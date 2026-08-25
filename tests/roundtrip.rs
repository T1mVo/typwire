use std::collections::{BTreeMap, HashMap, VecDeque};
use std::fmt::Debug;

use serde::{Deserialize, Serialize};
use typwire::{
    Angle, Any, Bytes, Center, Color, ColorGradient, ColorSpace, DateTime, Duration, Gradient,
    Length, LengthRadius, OneOf2, OneOf3, OneOf4, OneOf5, OneOf6, OneOf7, OneOf8, Radius, Ratio,
    Rgb, Stop, Type, Typwire, Version, decode, encode,
};

#[derive(Debug, Serialize, Deserialize, Typwire)]
#[serde(rename_all = "kebab-case")]
struct Model<T> {
    name: String,
    value: T,
}

fn roundtrip<T>(value: &T)
where
    T: Typwire + Debug,
{
    let encoded = encode(value).unwrap();
    let decoded: T = decode(&encoded).unwrap();
    assert_eq!(encode(&decoded).unwrap(), encoded);
}

#[test]
fn primitives_derived_models_and_containers_roundtrip() {
    roundtrip(&());
    roundtrip(&true);
    roundtrip(&-42_i64);
    roundtrip(&42_u32);
    roundtrip(&1.5_f64);
    roundtrip(&'x');
    roundtrip(&"text".to_owned());
    roundtrip(&Bytes::new(vec![0, 1, 255]));
    roundtrip(&Some("value".to_owned()));
    roundtrip(&vec![1_i64, 2, 3]);
    roundtrip(&VecDeque::from([1_i64, 2, 3]));
    roundtrip(&[1_i64, 2, 3]);
    roundtrip(&(1_i64, "two".to_owned(), true));
    roundtrip(&Box::new(42_i64));
    roundtrip(&BTreeMap::from([("key".to_owned(), 42_i64)]));
    roundtrip(&HashMap::from([("key".to_owned(), 42_i64)]));
    roundtrip(&Model {
        name: "answer".to_owned(),
        value: 42_i64,
    });
}

#[test]
fn typst_wrappers_roundtrip() {
    let ratio = Ratio::new(0.5);
    let angle = Angle::new(std::f64::consts::FRAC_PI_2);
    let color = Color::Rgb(Rgb::new(ratio, ratio, ratio, Ratio::new(1.0)));
    let center = Center::new(ratio, ratio);
    let stop_a = Stop::new(color.clone(), Ratio::new(0.0));
    let stop_b = Stop::new(color.clone(), Ratio::new(1.0));
    let gradient = Gradient::radial(
        vec![stop_a, stop_b],
        center.clone(),
        Ratio::new(1.0),
        center.clone(),
        Ratio::new(0.0),
        ColorSpace::Rgb,
    );

    roundtrip(&angle);
    roundtrip(&center);
    roundtrip(&color);
    roundtrip(&ColorGradient::Color(color.clone()));
    roundtrip(&DateTime::builder().year(2026).month(8).day(25).build());
    roundtrip(&Duration::new(90.5));
    roundtrip(&gradient);
    roundtrip(&Length::new(12.0));
    roundtrip(&LengthRadius::Length(Length::new(4.0)));
    roundtrip(
        &Radius::builder()
            .top(Length::new(2.0))
            .rest(Length::new(1.0))
            .build(),
    );
    roundtrip(&ratio);
    roundtrip(&Type::new("integer".to_owned()));
    roundtrip(&Version::new(1, 2, 3, 4, 5));
}

#[test]
fn any_preserves_native_tagged_values_and_nested_data() {
    let value = Any::Dictionary(BTreeMap::from([
        ("angle".to_owned(), Any::Angle(Angle::new(1.0))),
        (
            "items".to_owned(),
            Any::Array(vec![
                Any::Integer(1),
                Any::Length(Length::new(12.0)),
                Any::Bytes(Bytes::new(vec![1, 2, 3])),
            ]),
        ),
    ]));

    roundtrip(&value);
}

#[test]
fn all_one_of_arities_roundtrip() {
    type Two = OneOf2<i64, String>;
    type Three = OneOf3<i64, bool, String>;
    type Four = OneOf4<i64, bool, String, Vec<i64>>;
    type Five = OneOf5<i64, bool, String, Vec<i64>, Length>;
    type Six = OneOf6<i64, bool, String, Vec<i64>, Length, Ratio>;
    type Seven = OneOf7<i64, bool, String, Vec<i64>, Length, Ratio, Type>;
    type Eight = OneOf8<i64, bool, String, Vec<i64>, Length, Ratio, Type, Angle>;

    roundtrip(&Two::Second("two".to_owned()));
    roundtrip(&Three::Third("three".to_owned()));
    roundtrip(&Four::Fourth(vec![4]));
    roundtrip(&Five::Fifth(Length::new(5.0)));
    roundtrip(&Six::Sixth(Ratio::new(0.6)));
    roundtrip(&Seven::Seventh(Type::new("integer".to_owned())));
    roundtrip(&Eight::Eighth(Angle::new(0.8)));
}

#[test]
fn one_of_uses_order_and_reports_total_failure() {
    let integer = encode(&42_i64).unwrap();
    assert!(matches!(
        decode::<OneOf2<i64, f64>>(&integer).unwrap(),
        OneOf2::First(42)
    ));

    let boolean = encode(&true).unwrap();
    assert!(decode::<OneOf2<i64, String>>(&boolean).is_err());
}

#[test]
fn malformed_and_invalid_wire_values_fail() {
    assert!(decode::<String>(&[0xff]).is_err());

    let mut non_string_map = Vec::new();
    ciborium::into_writer(
        &ciborium::Value::Map(vec![(
            ciborium::Value::Integer(1.into()),
            ciborium::Value::Null,
        )]),
        &mut non_string_map,
    )
    .unwrap();
    assert!(
        decode::<Any>(&non_string_map)
            .unwrap_err()
            .to_string()
            .contains("string keys")
    );

    let unknown = Any::Dictionary(BTreeMap::from([(
        "typwire-type".to_owned(),
        Any::String("unknown".to_owned()),
    )]));
    assert!(
        decode::<Any>(&encode(&unknown).unwrap())
            .unwrap_err()
            .to_string()
            .contains("unknown typwire-type")
    );
}
