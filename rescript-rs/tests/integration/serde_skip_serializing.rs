#![cfg(feature = "serde-compat")]

use rescript_rs::{Config, TS};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize, TS)]
#[rescript(export, export_to = "serde_skip_serializing/")]
pub struct Named {
    #[serde(skip_serializing_if = "Option::is_none")]
    a: Option<u8>,

    #[serde(skip_serializing_if = "std::ops::Not::not", default)]
    b: bool,

    #[serde(skip_serializing_if = "Option::is_none", default)]
    c: Option<i32>,

    #[serde(skip_serializing, default)]
    d: Option<i32>,

    #[serde(skip_serializing, default)]
    #[rescript(optional = false)]
    e: Option<i32>,

    #[serde(skip_serializing, default)]
    #[rescript(optional)]
    f: Option<i32>,
}

#[test]
fn named() {
    let a = "a: option<int>";
    let b = "b?: bool";
    let c = "c?: option<int>";
    let d = "d?: option<int>";
    let e = "e: option<int>";
    let f = "f?: int";
    let cfg = Config::from_env();

    assert_eq!(
        Named::decl(&cfg),
        format!("type named = {{\n  {a},\n  {b},\n  {c},\n  {d},\n  {e},\n  {f},\n}}")
    );
}

#[derive(Debug, Clone, Deserialize, Serialize, TS)]
#[rescript(export, export_to = "serde_skip_serializing/")]
pub struct Tuple(
    Option<i32>,
    #[rescript(optional)] Option<i32>,
    #[serde(skip_serializing, default)] Option<i32>,
);

#[test]
fn tuple() {
    let cfg = Config::from_env();
    assert_eq!(
        Tuple::decl(&cfg),
        "type tuple = (option<int>, (int)?, (option<int>)?)"
    );
}

#[derive(Debug, Clone, Deserialize, Serialize, TS)]
#[rescript(export, export_to = "serde_skip_serializing/")]
#[rescript(optional_fields = false)]
pub struct Overrides {
    #[serde(skip_serializing, default)]
    x: Option<i32>,
    y: Option<i32>,
    #[rescript(optional)]
    z: Option<i32>,
}

#[test]
fn overrides() {
    let x = "x: option<int>";
    let y = "y: option<int>";
    let z = "z?: int";
    let cfg = Config::from_env();
    assert_eq!(
        Overrides::decl(&cfg),
        format!("type overrides = {{\n  {x},\n  {y},\n  {z},\n}}")
    );
}
