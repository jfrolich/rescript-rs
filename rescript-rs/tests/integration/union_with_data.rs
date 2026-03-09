#![allow(dead_code)]

use rescript_rs::{Config, Dependency, TS};
#[cfg(feature = "serde-compat")]
use serde::Serialize;

#[derive(TS)]
#[cfg_attr(feature = "serde-compat", derive(Serialize))]
#[rescript(export, export_to = "union_with_data/")]
struct Bar {
    field: i32,
}

#[derive(TS)]
#[cfg_attr(feature = "serde-compat", derive(Serialize))]
#[rescript(export, export_to = "union_with_data/")]
struct Foo {
    bar: Bar,
}

#[derive(TS)]
#[cfg_attr(feature = "serde-compat", derive(Serialize))]
#[cfg_attr(feature = "serde-compat", serde(tag = "type", content = "value"))]
#[cfg_attr(not(feature = "serde-compat"), ts(tag = "type", content = "value"))]
#[rescript(export, export_to = "union_with_data/")]
enum SimpleEnum {
    A(String),
    B(i32),
    C,
    D(String, i32),
    E(Foo),
    F { a: i32, b: String },
}

#[test]
fn test_stateful_enum() {
    let cfg = Config::from_env();
    assert_eq!(Bar::decl(&cfg), "type bar = {\n  field: int,\n}");
    assert_eq!(Bar::dependencies(&cfg), vec![]);

    assert_eq!(Foo::decl(&cfg), "type foo = {\n  bar: bar,\n}");
    assert_eq!(
        Foo::dependencies(&cfg),
        vec![Dependency::from_ty::<Bar>(&cfg).unwrap()]
    );

    assert_eq!(
        SimpleEnum::decl(&cfg),
        "@tag(\"type\")\ntype simpleEnum = \n  | A({ value: string })\n  | B({ value: int })\n  | C\n  | D({ value: (string, int) })\n  | E({ value: foo })\n  | F({ value: { a: int, b: string, } })"
    );
    assert!(SimpleEnum::dependencies(&cfg)
        .into_iter()
        .all(|dep| dep == Dependency::from_ty::<Foo>(&cfg).unwrap()));
}
