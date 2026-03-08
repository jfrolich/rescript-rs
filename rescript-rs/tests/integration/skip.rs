#![allow(dead_code, unused_imports)]

use std::error::Error;

use serde::Serialize;
use rescript_rs::{Config, TS};

struct Unsupported;

#[derive(TS)]
#[rescript(export, export_to = "skip/")]
struct Skip {
    a: i32,
    b: i32,
    #[rescript(skip)]
    c: String,
    #[rescript(skip)]
    d: Box<dyn Error>,
}

#[test]
fn simple() {
    let cfg = Config::from_env();
    assert_eq!(Skip::inline(&cfg), "{ a: int, b: int, }");
}

#[derive(TS)]
#[cfg_attr(feature = "serde-compat", derive(Serialize))]
#[cfg_attr(feature = "serde-compat", serde(tag = "type", content = "value"))]
#[cfg_attr(not(feature = "serde-compat"), ts(tag = "type", content = "value"))]
#[rescript(export, export_to = "skip/")]
enum Externally {
    A(
        #[cfg_attr(feature = "serde-compat", serde(skip))]
        #[cfg_attr(not(feature = "serde-compat"), ts(skip))]
        Unsupported,
    ),
    B(
        #[cfg_attr(feature = "serde-compat", serde(skip))]
        #[cfg_attr(not(feature = "serde-compat"), ts(skip))]
        Unsupported,
        i32,
    ),
    C {
        #[cfg_attr(feature = "serde-compat", serde(skip))]
        #[cfg_attr(not(feature = "serde-compat"), ts(skip))]
        x: Unsupported,
    },
    D {
        #[cfg_attr(feature = "serde-compat", serde(skip))]
        #[cfg_attr(not(feature = "serde-compat"), ts(skip))]
        x: Unsupported,
        y: i32,
    },
}

#[test]
fn externally_tagged() {
    // TODO: variant C should probably not generate `{}`
    let cfg = Config::from_env();
    assert_eq!(
        Externally::decl(&cfg),
        "@tag(\"type\")\ntype externally = | A | B({ value: (int) }) | C({ value: {  } }) | D({ value: { y: int, } })"
    );
}

#[derive(TS)]
#[cfg_attr(feature = "serde-compat", derive(Serialize))]
#[cfg_attr(feature = "serde-compat", serde(tag = "t"))]
#[cfg_attr(not(feature = "serde-compat"), ts(tag = "t"))]
#[rescript(export, export_to = "skip/")]
enum Internally {
    A(
        #[cfg_attr(feature = "serde-compat", serde(skip))]
        #[cfg_attr(not(feature = "serde-compat"), ts(skip))]
        Unsupported,
    ),
    B {
        #[cfg_attr(feature = "serde-compat", serde(skip))]
        #[cfg_attr(not(feature = "serde-compat"), ts(skip))]
        x: Unsupported,
    },
    C {
        #[cfg_attr(feature = "serde-compat", serde(skip))]
        #[cfg_attr(not(feature = "serde-compat"), ts(skip))]
        x: Unsupported,
        y: i32,
    },
}

#[test]
fn internally_tagged() {
    let cfg = Config::from_env();
    assert_eq!(
        Internally::decl(&cfg),
        "@tag(\"t\")\ntype internally = | A | B({  }) | C({ y: int, })"
    );
}

#[derive(TS)]
#[cfg_attr(feature = "serde-compat", derive(Serialize))]
#[cfg_attr(feature = "serde-compat", serde(tag = "t", content = "c"))]
#[cfg_attr(not(feature = "serde-compat"), ts(tag = "t", content = "c"))]
#[rescript(export, export_to = "skip/")]
enum Adjacently {
    A(
        #[cfg_attr(feature = "serde-compat", serde(skip))]
        #[cfg_attr(not(feature = "serde-compat"), ts(skip))]
        Unsupported,
    ),
    B(
        #[cfg_attr(feature = "serde-compat", serde(skip))]
        #[cfg_attr(not(feature = "serde-compat"), ts(skip))]
        Unsupported,
        i32,
    ),
    C {
        #[cfg_attr(feature = "serde-compat", serde(skip))]
        #[cfg_attr(not(feature = "serde-compat"), ts(skip))]
        x: Unsupported,
    },
    D {
        #[cfg_attr(feature = "serde-compat", serde(skip))]
        #[cfg_attr(not(feature = "serde-compat"), ts(skip))]
        x: Unsupported,
        y: i32,
    },
}

#[test]
fn adjacently_tagged() {
    // TODO: variant C should probably not generate `{ .., "c": { } }`
    let cfg = Config::from_env();
    assert_eq!(
        Adjacently::decl(&cfg),
        "@tag(\"t\")\ntype adjacently = | A | B({ c: (int) }) | C({ c: {  } }) | D({ c: { y: int, } })"
    );
}
