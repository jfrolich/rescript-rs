#![allow(dead_code)]

use rescript_rs::{Config, TS};
#[cfg(feature = "serde-compat")]
use serde::{Deserialize, Serialize};

#[derive(TS)]
#[cfg_attr(feature = "serde-compat", derive(Deserialize))]
#[cfg_attr(feature = "serde-compat", serde(tag = "kind", content = "d"))]
#[cfg_attr(not(feature = "serde-compat"), ts(tag = "kind", content = "d"))]
#[rescript(export, export_to = "union_serde/")]
enum SimpleEnum {
    A,
    B,
}

#[derive(TS)]
#[cfg_attr(feature = "serde-compat", derive(Deserialize))]
#[cfg_attr(feature = "serde-compat", serde(tag = "kind", content = "data"))]
#[cfg_attr(not(feature = "serde-compat"), ts(tag = "kind", content = "data"))]
#[rescript(export, export_to = "union_serde/")]
enum ComplexEnum {
    A,
    B { foo: String, bar: f64 },
    W(SimpleEnum),
    F { nested: SimpleEnum },
    T(i32, SimpleEnum),
}

#[derive(TS)]
#[cfg_attr(feature = "serde-compat", derive(Deserialize))]
#[cfg_attr(feature = "serde-compat", serde(untagged))]
#[cfg_attr(not(feature = "serde-compat"), ts(untagged))]
#[rescript(export, export_to = "union_serde/")]
enum Untagged {
    Foo(String),
    Bar(i32),
    None,
}

#[test]
fn test_serde_enum() {
    let cfg = Config::from_env();
    assert_eq!(
        SimpleEnum::decl(&cfg),
        "@tag(\"kind\")\ntype simpleEnum = \n  | A\n  | B"
    );
    assert_eq!(
        ComplexEnum::decl(&cfg),
        "@tag(\"kind\")\ntype complexEnum = \n  | A\n  | B({ data: { foo: string, bar: float, } })\n  | W({ data: simpleEnum })\n  | F({ data: { nested: simpleEnum, } })\n  | T({ data: (int, simpleEnum) })"
    );

    assert_eq!(
        Untagged::decl(&cfg),
        "type untagged = \n  | Foo(string)\n  | Bar(int)\n  | None "
    )
}

#[derive(TS)]
#[cfg_attr(feature = "serde-compat", derive(Serialize))]
#[cfg_attr(
    feature = "serde-compat",
    serde(deny_unknown_fields, rename_all = "camelCase")
)]
#[cfg_attr(not(feature = "serde-compat"), ts(rename_all = "camelCase"))]
enum Enum {
    FirstOption,
    SecondOption,
}

#[test]
fn test_rename_all() {
    let cfg = Config::from_env();
    assert_eq!(
        Enum::inline(&cfg),
        "\n  | @as(\"firstOption\") FirstOption\n  | @as(\"secondOption\") SecondOption"
    );
}
