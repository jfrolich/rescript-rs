#![allow(non_snake_case)]
#![allow(dead_code)]

use rescript_rs::{Config, TS};

#[derive(TS)]
#[rescript(export, export_to = "struct_rename/", rename_all = "UPPERCASE")]
struct RenameAllUpper {
    a: i32,
    b: i32,
}

#[test]
fn rename_all() {
    let cfg = Config::from_env();
    assert_eq!(RenameAllUpper::inline(&cfg), "{ A: int, B: int, }");
}

#[derive(TS)]
#[rescript(export, export_to = "struct_rename/", rename_all = "camelCase")]
struct RenameAllCamel {
    crc32c_hash: i32,
    b: i32,
    alreadyCamelCase: i32,
}

#[test]
fn rename_all_camel_case() {
    let cfg = Config::from_env();
    assert_eq!(
        RenameAllCamel::inline(&cfg),
        "{ crc32cHash: int, b: int, alreadyCamelCase: int, }"
    );
}

#[derive(TS)]
#[rescript(export, export_to = "struct_rename/", rename_all = "PascalCase")]
struct RenameAllPascal {
    crc32c_hash: i32,
    b: i32,
}

#[test]
fn rename_all_pascal_case() {
    let cfg = Config::from_env();
    assert_eq!(
        RenameAllPascal::inline(&cfg),
        "{ Crc32cHash: int, B: int, }"
    );
}

#[derive(TS, Default, serde::Serialize)]
#[rescript(export, export_to = "struct_rename/")]
#[cfg_attr(feature = "serde-compat", serde(rename_all = "SCREAMING-KEBAB-CASE"))]
#[cfg_attr(not(feature = "serde-compat"), ts(rename_all = "SCREAMING-KEBAB-CASE"))]
struct RenameAllScreamingKebab {
    crc32c_hash: i32,
    some_field: i32,
    some_other_field: i32,
}

#[test]
fn rename_all_screaming_kebab_case() {
    let rename_all = RenameAllScreamingKebab::default();
    let cfg = Config::from_env();
    assert_eq!(
        RenameAllScreamingKebab::inline(&cfg),
        "{ @as(\"CRC32C-HASH\") CRC32C-HASH_: int, @as(\"SOME-FIELD\") SOME-FIELD_: int, @as(\"SOME-OTHER-FIELD\") SOME-OTHER-FIELD_: int, }"
    );
}

#[derive(serde::Serialize, TS)]
#[rescript(export, export_to = "struct_rename/", rename_all = "camelCase")]
struct RenameSerdeSpecialChar {
    #[serde(rename = "a/b")]
    b: i32,
}

#[cfg(feature = "serde-compat")]
#[test]
fn serde_rename_special_char() {
    let cfg = Config::from_env();
    assert_eq!(
        RenameSerdeSpecialChar::inline(&cfg),
        "{ @as(\"a/b\") a/b_: int, }"
    );
}

// struct-level renames

#[derive(TS)]
#[rescript(export, export_to = "struct_rename/")]
#[rescript(rename = "RenamedWithStrLiteral")]
enum WithStrLiteral {
    A,
    B,
    C,
}

#[test]
fn test_rename_with_str_literal() {
    let cfg = Config::from_env();
    assert_eq!(
        WithStrLiteral::decl(&cfg),
        "type renamedWithStrLiteral = \n  | A\n  | B\n  | C"
    )
}

#[derive(TS)]
#[rescript(export, export_to = "struct_rename/")]
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
        "type renamedWithStringExpression = \n  | A\n  | B\n  | C"
    )
}

#[derive(TS)]
#[rescript(export, export_to = "struct_rename/")]
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
        "type renamedWithStrExpression = \n  | A\n  | B\n  | C"
    )
}

#[derive(TS)]
#[rescript(export, export_to = "struct_rename/")]
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
        "type i_am_inside_module_struct_rename = \n  | A\n  | B\n  | C"
    )
}
