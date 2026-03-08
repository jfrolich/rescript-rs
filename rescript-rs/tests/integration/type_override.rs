#![allow(dead_code)]

use std::time::Instant;

#[cfg(feature = "serde-compat")]
use serde::Serialize;
use rescript_rs::{Config, TS};

struct Unsupported<T>(T);
struct Unsupported2;

#[derive(TS)]
#[rescript(export, export_to = "type_override/")]
struct Override {
    a: i32,
    #[rescript(type = "0 | 1 | 2")]
    b: i32,
    #[rescript(type = "string")]
    x: Instant,
    #[rescript(type = "string")]
    y: Unsupported<Unsupported<Unsupported2>>,
    #[rescript(type = "string | null")]
    z: Option<Unsupported2>,
}

#[test]
fn simple() {
    let cfg = Config::from_env();
    assert_eq!(
        Override::inline(&cfg),
        "{ a: number, b: 0 | 1 | 2, x: string, y: string, z: string | null, }"
    )
}

#[derive(TS)]
#[rescript(export, export_to = "type_override/")]
struct New1(#[rescript(type = "string")] Unsupported2);

#[derive(TS)]
#[rescript(export, export_to = "type_override/")]
struct New2(#[rescript(type = "string | null")] Unsupported<Unsupported2>);

#[test]
fn newtype() {
    let cfg = Config::from_env();
    assert_eq!(New1::inline(&cfg), r#"string"#);
    assert_eq!(New2::inline(&cfg), r#"string | null"#);
}

#[cfg_attr(feature = "serde-compat", derive(Serialize))]
struct S;

#[derive(TS)]
#[cfg_attr(feature = "serde-compat", derive(Serialize))]
#[cfg_attr(feature = "serde-compat", serde(tag = "t"))]
#[cfg_attr(not(feature = "serde-compat"), ts(tag = "t"))]
#[rescript(export, export_to = "type_override/")]
enum Internal {
    Newtype(#[rescript(type = "unknown")] S),
}

#[derive(TS)]
#[cfg_attr(feature = "serde-compat", derive(Serialize))]
#[cfg_attr(feature = "serde-compat", serde(tag = "t", content = "c"))]
#[cfg_attr(not(feature = "serde-compat"), ts(tag = "t", content = "c"))]
#[rescript(export, export_to = "type_override/")]
enum Adjacent {
    Newtype(#[rescript(type = "unknown")] S),
}

#[test]
fn enum_newtype_representations() {
    // regression test for https://github.com/Aleph-Alpha/ts-rs/issues/126
    let cfg = Config::from_env();
    assert_eq!(Internal::inline(&cfg), r#"{ "t": "Newtype" } & unknown"#);
    assert_eq!(Adjacent::inline(&cfg), r#"{ "t": "Newtype", "c": unknown }"#);
}
