#![allow(dead_code)]

use rescript_rs::{Config, TS};

#[derive(TS)]
#[rescript(export, export_to = "union/")]
enum SimpleEnum {
    #[rescript(rename = "asdf")]
    A,
    B,
    C,
    r#D,
}

#[test]
fn test_empty() {
    #[derive(TS)]
    enum Empty {}
    let cfg = Config::from_env();
    assert_eq!(Empty::decl(&cfg), "type empty = never")
}

#[test]
fn test_simple_enum() {
    let cfg = Config::from_env();
    assert_eq!(
        SimpleEnum::decl(&cfg),
        "type simpleEnum = \n  | @as(\"asdf\") A\n  | B\n  | C\n  | D"
    )
}
