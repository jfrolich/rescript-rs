#![allow(non_camel_case_types, dead_code)]

use rescript_rs::{Config, TS};

#[derive(TS)]
#[rescript(export, export_to = "raw_idents/")]
struct r#struct {
    r#type: i32,
    r#use: i32,
    r#struct: i32,
    r#let: i32,
    r#enum: i32,
}

#[test]
fn raw_idents() {
    let cfg = Config::from_env();
    let out = <r#struct as TS>::decl(&cfg);
    assert_eq!(
        out,
        "type struct = {\n  @as(\"type\") type_: int,\n  use: int,\n  struct: int,\n  @as(\"let\") let_: int,\n  enum: int,\n}"
    );
}
