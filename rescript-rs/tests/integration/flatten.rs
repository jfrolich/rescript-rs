#![allow(dead_code)]

use std::collections::HashMap;

use rescript_rs::{Config, TS};

#[derive(TS)]
#[rescript(export, export_to = "flatten/")]
struct A {
    a: i32,
    b: i32,
    #[rescript(flatten)]
    c: HashMap<String, i32>,
}

#[derive(TS)]
#[rescript(export, export_to = "flatten/")]
struct B {
    #[rescript(flatten)]
    a: A,
    c: i32,
}

#[derive(TS)]
#[rescript(export, export_to = "flatten/")]
struct C {
    #[rescript(inline)]
    b: B,
    d: i32,
}

#[test]
fn test_def() {
    let cfg = Config::from_env();
    assert_eq!(
        C::inline(&cfg),
        "{ b: { c: int, a: int, b: int, } & (Dict.t<int>), d: int, }"
    );
}
