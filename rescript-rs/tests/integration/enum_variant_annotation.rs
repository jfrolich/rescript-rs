#![allow(dead_code)]

use rescript_rs::{Config, TS};
#[cfg(feature = "serde-compat")]
use serde::Serialize;

#[derive(TS)]
#[rescript(export, export_to = "enum_variant_anotation/")]
#[cfg_attr(feature = "serde-compat", derive(Serialize))]
#[cfg_attr(feature = "serde-compat", serde(rename_all = "SCREAMING_SNAKE_CASE"))]
#[cfg_attr(
    not(feature = "serde-compat"),
    rescript(rename_all = "SCREAMING_SNAKE_CASE")
)]
#[cfg_attr(feature = "serde-compat", serde(tag = "type"))]
#[cfg_attr(not(feature = "serde-compat"), rescript(tag = "type"))]
enum A {
    MessageOne {
        sender_id: String,
        number_of_snakes: u64,
    },
    #[cfg_attr(feature = "serde-compat", serde(rename_all = "camelCase"))]
    #[cfg_attr(not(feature = "serde-compat"), rescript(rename_all = "camelCase"))]
    MessageTwo {
        sender_id: String,
        number_of_camels: u64,
    },
}

#[test]
fn test_enum_variant_rename_all() {
    let cfg = Config::from_env();
    assert_eq!(
        A::inline(&cfg),
        "\n  | @as(\"MESSAGE_ONE\") MessageOne({ sender_id: string, number_of_snakes: BigInt.t, })\n  | @as(\"MESSAGE_TWO\") MessageTwo({ senderId: string, numberOfCamels: BigInt.t, })",
    );
}

#[derive(TS)]
#[rescript(export, export_to = "enum_variant_anotation/")]
#[cfg_attr(feature = "serde-compat", derive(Serialize))]
#[cfg_attr(feature = "serde-compat", serde(tag = "type"))]
#[cfg_attr(not(feature = "serde-compat"), rescript(tag = "type"))]
enum B {
    #[cfg_attr(feature = "serde-compat", serde(rename = "SnakeMessage"))]
    #[cfg_attr(not(feature = "serde-compat"), rescript(rename = "SnakeMessage"))]
    MessageOne {
        sender_id: String,
        number_of_snakes: u64,
    },
    #[cfg_attr(feature = "serde-compat", serde(rename = "CamelMessage"))]
    #[cfg_attr(not(feature = "serde-compat"), rescript(rename = "CamelMessage"))]
    MessageTwo {
        sender_id: String,
        number_of_camels: u64,
    },
}

#[test]
fn test_enum_variant_rename() {
    let cfg = Config::from_env();
    assert_eq!(
        B::inline(&cfg),
        "\n  | @as(\"SnakeMessage\") MessageOne({ sender_id: string, number_of_snakes: BigInt.t, })\n  | @as(\"CamelMessage\") MessageTwo({ sender_id: string, number_of_camels: BigInt.t, })",
    );
}

#[derive(TS)]
#[rescript(export, export_to = "enum_variant_anotation/")]
#[cfg_attr(feature = "serde-compat", derive(Serialize))]
#[cfg_attr(feature = "serde-compat", serde(tag = "kind"))]
#[cfg_attr(not(feature = "serde-compat"), rescript(tag = "kind"))]
pub enum C {
    #[cfg_attr(feature = "serde-compat", serde(rename = "SQUARE_THING"))]
    #[cfg_attr(not(feature = "serde-compat"), rescript(rename = "SQUARE_THING"))]
    SquareThing {
        name: String,
        // ...
    },
}

#[test]
fn test_enum_variant_with_tag() {
    let cfg = Config::from_env();
    assert_eq!(
        C::inline(&cfg),
        "\n  | @as(\"SQUARE_THING\") SquareThing({ name: string, })"
    );
}

#[cfg(feature = "serde-compat")]
#[test]
fn test_tag_and_content_quoted() {
    #[derive(Serialize, TS)]
    #[serde(tag = "kebab-cased-tag", content = "whitespace in content")]
    enum E {
        V { f: String },
    }
    let cfg = Config::from_env();
    assert_eq!(
        E::inline(&cfg),
        "\n  | V({ whitespace in content: { f: string, } })"
    )
}

#[cfg(feature = "serde-compat")]
#[test]
fn test_variant_quoted() {
    #[derive(Serialize, TS)]
    #[serde(rename_all = "kebab-case", tag = "type")]
    enum E {
        VariantName { f: String },
    }
    let cfg = Config::from_env();
    assert_eq!(
        E::inline(&cfg),
        "\n  | @as(\"variant-name\") VariantName({ f: string, })"
    )
}

#[derive(TS)]
#[rescript(export, export_to = "enum_variant_anotation/", tag = "type")]
enum D {
    Foo {},
}

#[derive(TS)]
#[rescript(export, export_to = "enum_variant_anotation/", tag = "type")]
enum E {
    Foo {},
    Bar {},
    Biz { x: i32 },
}

#[test]
fn test_empty_struct_variant_with_tag() {
    let cfg = Config::from_env();
    assert_eq!(
        E::inline(&cfg),
        "\n  | Foo({  })\n  | Bar({  })\n  | Biz({ x: int, })"
    )
}
