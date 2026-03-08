#![allow(dead_code, clippy::box_collection)]

use std::borrow::Cow;

use rescript_rs::{Config, TS};

#[derive(TS)]
#[rescript(export, export_to = "generic_fields/")]
struct Newtype(Vec<Cow<'static, i32>>);

#[test]
fn newtype() {
    let cfg = Config::from_env();
    assert_eq!(Newtype::inline(&cfg), "array<int>");
}

#[derive(TS)]
#[rescript(export, export_to = "generic_fields/")]
struct NewtypeNested(Vec<Vec<i32>>);

#[test]
fn newtype_nested() {
    let cfg = Config::from_env();
    assert_eq!(NewtypeNested::inline(&cfg), "array<array<int>>");
}

#[test]
fn alias() {
    type Alias = Vec<String>;
    let cfg = Config::from_env();
    assert_eq!(Alias::inline(&cfg), "array<string>");
}

#[test]
fn alias_nested() {
    type Alias = Vec<Vec<String>>;
    let cfg = Config::from_env();
    assert_eq!(Alias::inline(&cfg), "array<array<string>>");
}

#[derive(TS)]
#[rescript(export, export_to = "generic_fields/")]
struct Struct {
    a: Box<Vec<String>>,
    b: (Vec<String>, Vec<String>),
    c: [Vec<String>; 3],
}

#[test]
fn named() {
    let cfg = Config::from_env();
    assert_eq!(
        Struct::inline(&cfg),
        "{ a: array<string>, b: (array<string>, array<string>), c: (array<string>, array<string>, array<string>), }"
    );
}

#[derive(TS)]
#[rescript(export, export_to = "generic_fields/")]
struct StructNested {
    a: Vec<Vec<String>>,
    b: (Vec<Vec<String>>, Vec<Vec<String>>),
    c: [Vec<Vec<String>>; 3],
}

#[test]
fn named_nested() {
    let cfg = Config::from_env();
    assert_eq!(
        StructNested::inline(&cfg),
        "{ a: array<array<string>>, b: (array<array<string>>, array<array<string>>), c: (array<array<string>>, array<array<string>>, array<array<string>>), }"
    );
}

#[derive(TS)]
#[rescript(export, export_to = "generic_fields/")]
struct Tuple(Vec<i32>, (Vec<i32>, Vec<i32>), [Vec<i32>; 3]);

#[test]
fn tuple() {
    let cfg = Config::from_env();
    assert_eq!(
        Tuple::inline(&cfg),
        "(array<int>, (array<int>, array<int>), (array<int>, array<int>, array<int>))"
    );
}

#[derive(TS)]
#[rescript(export, export_to = "generic_fields/")]
struct TupleNested(
    Vec<Vec<i32>>,
    (Vec<Vec<i32>>, Vec<Vec<i32>>),
    [Vec<Vec<i32>>; 3],
);

#[test]
fn tuple_nested() {
    let cfg = Config::from_env();
    assert_eq!(
        TupleNested::inline(&cfg),
        "(array<array<int>>, (array<array<int>>, array<array<int>>), (array<array<int>>, array<array<int>>, array<array<int>>))"
    );
}
