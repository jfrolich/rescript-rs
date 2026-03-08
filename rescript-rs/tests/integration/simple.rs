#![allow(dead_code)]

use std::cell::RefCell;

use rescript_rs::{Config, TS};

#[derive(TS)]
#[rescript(export, export_to = "simple/")]
struct Simple {
    a: i32,
    b: String,
    c: (i32, String, RefCell<i32>),
    d: Vec<String>,
    e: Option<String>,
    f: char,
    g: Option<char>,
}

#[test]
fn test_def() {
    let cfg = Config::from_env();
    assert_eq!(
        Simple::inline(&cfg),
        "{ a: int, b: string, c: (int, string, int), d: array<string>, e: option<string>, f: string, g: option<string>, }"
    )
}
