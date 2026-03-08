#![allow(dead_code)]

use rescript_rs::{Config, TS};

#[derive(TS)]
#[rescript(export, export_to = "union_rename/")]
#[rescript(rename_all = "lowercase", rename = "SimpleEnum")]
enum RenamedEnum {
    #[rescript(rename = "ASDF")]
    A,
    #[rescript(rename = &"BB")]
    B,
    #[rescript(rename = "C".repeat(2))]
    C,
}

#[test]
fn test_simple_enum() {
    let cfg = Config::from_env();
    assert_eq!(
        RenamedEnum::decl(&cfg),
        "type simpleenum = | @as(\"ASDF\") A | @as(\"BB\") B | @as(\"CC\") C"
    )
}

#[derive(TS)]
#[rescript(export, export_to = "union_rename/")]
#[rescript(rename = format!("{}With{}", "Renamed", "StringExpression"))]
enum WithStringExpression {
    A,
    B,
    C,
}

#[test]
fn test_rename_with_string_expression() {
    let cfg = Config::from_env();
    assert_eq!(
        WithStringExpression::decl(&cfg),
        "type renamedwithstringexpression = | A | B | C"
    )
}

#[derive(TS)]
#[rescript(export, export_to = "union_rename/")]
#[rescript(rename = &"RenamedWithStrExpression")]
enum WithStrExpression {
    A,
    B,
    C,
}

#[test]
fn test_rename_with_str_expression() {
    let cfg = Config::from_env();
    assert_eq!(
        WithStrExpression::decl(&cfg),
        "type renamedwithstrexpression = | A | B | C"
    )
}

#[derive(TS)]
#[rescript(export, export_to = "union_rename/")]
#[rescript(rename = format!("i_am_inside_module_{}", module_path!().rsplit_once("::").unwrap().1))]
enum RenameUsingModuleName {
    A,
    B,
    C,
}

#[test]
fn test_rename_using_module_name() {
    let cfg = Config::from_env();
    assert_eq!(
        RenameUsingModuleName::decl(&cfg),
        "type i_am_inside_module_union_rename = | A | B | C"
    )
}
