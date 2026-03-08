#![allow(dead_code)]

#[cfg(feature = "serde-compat")]
use serde::{Deserialize, Serialize};
use rescript_rs::{Config, TS};

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
        "@tag(\"kind\")\ntype simpleenum = | A | B"
    );
    assert_eq!(
        ComplexEnum::decl(&cfg),
        "@tag(\"kind\")\ntype complexenum = | A | B({ data: { foo: string, bar: float, } }) | W({ data: SimpleEnum }) | F({ data: { nested: SimpleEnum, } }) | T({ data: (int, SimpleEnum) })"
    );

    assert_eq!(
        Untagged::decl(&cfg),
        "type untagged = | Foo(string) | Bar(int) | None "
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
    assert_eq!(Enum::inline(&cfg), "| @as(\"firstOption\") FirstOption | @as(\"secondOption\") SecondOption");
}
