#![allow(dead_code, unused_imports)]

use std::error::Error;

use rescript_rs::{Config, TS};
use serde::Serialize;

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
        "@tag(\"type\")\ntype externally = \n  | A\n  | B({ value: (int) })\n  | C({ value: {  } })\n  | D({ value: { y: int, } })"
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
        "@tag(\"t\")\ntype internally = \n  | A\n  | B({  })\n  | C({ y: int, })"
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
        "@tag(\"t\")\ntype adjacently = \n  | A\n  | B({ c: (int) })\n  | C({ c: {  } })\n  | D({ c: { y: int, } })"
    );
}
