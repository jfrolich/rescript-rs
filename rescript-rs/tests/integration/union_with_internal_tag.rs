#![allow(dead_code, clippy::disallowed_names)]

use rescript_rs::{Config, TS};
#[cfg(feature = "serde-compat")]
use serde::Serialize;

#[derive(TS)]
#[cfg_attr(feature = "serde-compat", derive(Serialize))]
#[cfg_attr(feature = "serde-compat", serde(tag = "type"))]
#[cfg_attr(not(feature = "serde-compat"), ts(tag = "type"))]
#[rescript(export, export_to = "union_with_internal_tag/")]
enum EnumWithInternalTag {
    A { foo: String },
    B { bar: i32 },
}

#[derive(TS)]
#[cfg_attr(feature = "serde-compat", derive(Serialize))]
#[rescript(export, export_to = "union_with_internal_tag/")]
struct InnerA {
    foo: String,
}

#[derive(TS)]
#[cfg_attr(feature = "serde-compat", derive(Serialize))]
#[rescript(export, export_to = "union_with_internal_tag/")]
struct InnerB {
    bar: i32,
}

#[derive(TS)]
#[cfg_attr(feature = "serde-compat", derive(Serialize))]
#[cfg_attr(feature = "serde-compat", serde(tag = "type"))]
#[cfg_attr(not(feature = "serde-compat"), ts(tag = "type"))]
#[rescript(export, export_to = "union_with_internal_tag/")]
enum EnumWithInternalTag2 {
    A(InnerA),
    B(InnerB),
}

/// Mixed enum: unit variants + payload variants under `@tag`.
/// Unit variants must emit `| Variant({})` so ReScript encodes them as
/// `{"type": "Variant"}` (an object) instead of a plain string, matching
/// what serde's `#[serde(tag = "type")]` expects on the Rust side.
#[derive(TS)]
#[cfg_attr(feature = "serde-compat", derive(Serialize))]
#[cfg_attr(feature = "serde-compat", serde(tag = "type"))]
#[cfg_attr(not(feature = "serde-compat"), ts(tag = "type"))]
#[rescript(export, export_to = "union_with_internal_tag/")]
enum MixedTaggedEnum {
    UnitA,
    UnitB,
    WithData { value: f64 },
}

#[test]
fn test_enums_with_internal_tags() {
    let cfg = Config::from_env();
    assert_eq!(
        EnumWithInternalTag::decl(&cfg),
        "@tag(\"type\")\ntype enumWithInternalTag = \n  | A({ foo: string, })\n  | B({ bar: int, })"
    );

    assert_eq!(
        EnumWithInternalTag2::decl(&cfg),
        "@tag(\"type\")\ntype enumWithInternalTag2 = \n  | A(innerA)\n  | B(innerB)"
    );
}

#[test]
fn test_mixed_tagged_enum_unit_variants_emit_empty_record() {
    let cfg = Config::from_env();
    assert_eq!(
        MixedTaggedEnum::decl(&cfg),
        "@tag(\"type\")\ntype mixedTaggedEnum = \n  | UnitA({})\n  | UnitB({})\n  | WithData({ value: float, })"
    );
}
