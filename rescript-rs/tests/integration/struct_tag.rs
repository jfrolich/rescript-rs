#![allow(dead_code)]

use rescript_rs::{Config, TS};
#[cfg(feature = "serde-compat")]
use serde::Serialize;

#[derive(TS)]
#[cfg_attr(feature = "serde-compat", derive(Serialize))]
#[cfg_attr(feature = "serde-compat", serde(tag = "type"))]
#[cfg_attr(not(feature = "serde-compat"), ts(tag = "type"))]
struct TaggedType {
    a: i32,
    b: i32,
}

#[derive(TS)]
#[cfg_attr(feature = "serde-compat", derive(Serialize))]
#[cfg_attr(feature = "serde-compat", serde(tag = "type"))]
#[cfg_attr(not(feature = "serde-compat"), ts(tag = "type"))]
struct EmptyTaggedType {}

#[test]
fn test() {
    let cfg = Config::from_env();
    // In ReScript mode, struct tags are dropped (ReScript has no equivalent)
    assert_eq!(TaggedType::inline(&cfg), "{ a: int, b: int, }");

    assert_eq!(EmptyTaggedType::inline(&cfg), "{  }");
}
