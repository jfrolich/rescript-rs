#![allow(unused)]

use std::collections::HashMap;

use rescript_rs::{Config, TS};
#[cfg(feature = "serde-compat")]
use serde::Serialize;

type TypeAlias = HashMap<String, String>;

#[derive(TS)]
#[cfg_attr(feature = "serde-compat", derive(Serialize))]
#[cfg_attr(feature = "serde-compat", serde(tag = "type", content = "value"))]
#[cfg_attr(
    not(feature = "serde-compat"),
    rescript(tag = "type", content = "value")
)]
#[rescript(export, export_to = "issue_70/")]
enum Enum {
    A(TypeAlias),
    B(HashMap<String, String>),
}

#[derive(TS)]
#[rescript(export, export_to = "issue_70/")]
struct Struct {
    a: TypeAlias,
    b: HashMap<String, String>,
}

#[test]
fn issue_70() {
    let cfg = Config::from_env();
    assert_eq!(
        Enum::decl(&cfg),
        "@tag(\"type\")\ntype enum = \n  | A({ value: Dict.t<string> })\n  | B({ value: Dict.t<string> })"
    );
    assert_eq!(
        Struct::decl(&cfg),
        "type struct = {\n  a: Dict.t<string>,\n  b: Dict.t<string>,\n}"
    );
}

#[derive(TS)]
#[rescript(export, export_to = "issue_70/")]
struct GenericType<T, U> {
    foo: T,
    bar: U,
}

type GenericAlias<A = String, B = String> = GenericType<(A, String), Vec<(B, i32)>>;

#[derive(TS)]
#[rescript(export, export_to = "issue_70/")]
struct Container {
    a: GenericAlias<Vec<i32>, Vec<String>>,
    b: GenericAlias,
}

#[derive(TS)]
#[rescript(export, export_to = "issue_70/")]
struct GenericContainer<A, B = i32> {
    a: GenericAlias,
    b: GenericAlias<A, B>,
    c: GenericAlias<A, GenericAlias<A, B>>,
}

#[test]
fn generic() {
    let cfg = Config::from_env();
    assert_eq!(
        Container::decl(&cfg),
        "type container = {\n  \
            a: genericType<(array<int>, string), array<(array<string>, int)>>,\n  \
            b: genericType<(string, string), array<(string, int)>>,\n\
        }"
    );

    assert_eq!(
        GenericContainer::<(), ()>::decl(&cfg),
        "type genericContainer<A, B = int> = {\n  \
            a: genericType<(string, string), array<(string, int)>>,\n  \
            b: genericType<(A, string), array<(B, int)>>,\n  \
            c: genericType<(A, string), array<(genericType<(A, string), array<(B, int)>>, int)>>,\n\
        }"
    );
}
