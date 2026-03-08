#![allow(dead_code)]

#[cfg(feature = "serde-compat")]
use serde::Serialize;
use rescript_rs::{Config, TS};

#[derive(TS)]
#[cfg_attr(feature = "serde-compat", derive(Serialize))]
#[rescript(export, export_to = "enum_flattening_nested/externally_tagged/")]
struct FooExternally {
    #[cfg_attr(feature = "serde-compat", serde(flatten))]
    #[cfg_attr(not(feature = "serde-compat"), rescript(flatten))]
    baz: BarExternally,
}

#[derive(TS)]
#[cfg_attr(feature = "serde-compat", derive(Serialize))]
#[rescript(export, export_to = "enum_flattening_nested/externally_tagged/")]
#[cfg_attr(feature = "serde-compat", serde(tag = "type"))]
#[cfg_attr(not(feature = "serde-compat"), rescript(tag = "type"))]
enum BarExternally {
    Baz { a: i32, a2: String },
    Biz { b: bool },
    Buz { c: String, d: Option<i32> },
}

#[derive(TS)]
#[cfg_attr(feature = "serde-compat", derive(Serialize))]
#[rescript(export, export_to = "enum_flattening_nested/externally_tagged/")]
struct NestedExternally {
    #[cfg_attr(feature = "serde-compat", serde(flatten))]
    #[cfg_attr(not(feature = "serde-compat"), rescript(flatten))]
    a: FooExternally,
    u: u32,
}

#[derive(TS)]
#[cfg_attr(feature = "serde-compat", derive(Serialize))]
#[rescript(export, export_to = "enum_flattening_nested/externally_tagged/")]
struct NestedExternallyLonely {
    #[cfg_attr(feature = "serde-compat", serde(flatten))]
    #[cfg_attr(not(feature = "serde-compat"), rescript(flatten))]
    a: FooExternally,
}

#[test]
fn externally_tagged() {
    // Notice here that baz is the only field inside `FooExternally`, so the parenthesis
    // aren't needed
    let cfg = Config::from_env();
    assert_eq!(
        FooExternally::inline(&cfg),
        "| Baz({ a: int, a2: string, }) | Biz({ b: bool, }) | Buz({ c: string, d: option<int>, })"
    );

    assert_eq!(
        NestedExternally::inline(&cfg),
        "{ u: int, } & (| Baz({ a: int, a2: string, }) | Biz({ b: bool, }) | Buz({ c: string, d: option<int>, }))"
    );

    assert_eq!(
        NestedExternallyLonely::inline(&cfg),
        "| Baz({ a: int, a2: string, }) | Biz({ b: bool, }) | Buz({ c: string, d: option<int>, })"
    );
}

#[derive(TS)]
#[rescript(export, export_to = "enum_flattening_nested/adjacently_tagged/")]
#[cfg_attr(feature = "serde-compat", derive(Serialize))]
struct FooAdjecently {
    #[cfg_attr(feature = "serde-compat", serde(flatten))]
    #[cfg_attr(not(feature = "serde-compat"), rescript(flatten))]
    baz: BarAdjecently,
}

#[derive(TS)]
#[rescript(export, export_to = "enum_flattening_nested/adjacently_tagged/")]
#[cfg_attr(feature = "serde-compat", derive(Serialize))]
#[cfg_attr(feature = "serde-compat", serde(tag = "type", content = "stuff"))]
#[cfg_attr(not(feature = "serde-compat"), rescript(tag = "type", content = "stuff"))]
enum BarAdjecently {
    Baz {
        a: i32,
        a2: String,
    },
    Biz {
        b: bool,
    },

    #[cfg_attr(feature = "serde-compat", serde(untagged))]
    #[cfg_attr(not(feature = "serde-compat"), rescript(untagged))]
    Buz {
        c: String,
        d: Option<i32>,
    },
}

#[derive(TS)]
#[cfg_attr(feature = "serde-compat", derive(Serialize))]
struct NestedAdjecently {
    #[cfg_attr(feature = "serde-compat", serde(flatten))]
    #[cfg_attr(not(feature = "serde-compat"), rescript(flatten))]
    a: FooAdjecently,
    u: u32,
}

#[derive(TS)]
#[cfg_attr(feature = "serde-compat", derive(Serialize))]
#[rescript(export, export_to = "enum_flattening_nested/externally_tagged/")]
struct NestedAdjecentlyLonely {
    #[cfg_attr(feature = "serde-compat", serde(flatten))]
    #[cfg_attr(not(feature = "serde-compat"), rescript(flatten))]
    a: FooAdjecently,
}

#[test]
fn adjacently_tagged() {
    let cfg = Config::from_env();
    assert_eq!(
        FooAdjecently::inline(&cfg),
        "| Baz({ stuff: { a: int, a2: string, } }) | Biz({ stuff: { b: bool, } }) | Buz({ c: string, d: option<int>, })"
    );

    assert_eq!(
        NestedAdjecently::inline(&cfg),
        "{ u: int, } & (| Baz({ stuff: { a: int, a2: string, } }) | Biz({ stuff: { b: bool, } }) | Buz({ c: string, d: option<int>, }))"
    );

    assert_eq!(
        NestedAdjecentlyLonely::inline(&cfg),
        "| Baz({ stuff: { a: int, a2: string, } }) | Biz({ stuff: { b: bool, } }) | Buz({ c: string, d: option<int>, })"
    );
}

#[derive(TS)]
#[rescript(export, export_to = "enum_flattening_nested/internally_tagged/")]
#[cfg_attr(feature = "serde-compat", derive(Serialize))]
struct FooInternally {
    #[cfg_attr(feature = "serde-compat", serde(flatten))]
    #[cfg_attr(not(feature = "serde-compat"), rescript(flatten))]
    baz: BarInternally,
}

#[derive(TS)]
#[rescript(export, export_to = "enum_flattening_nested/internally_tagged/")]
#[cfg_attr(feature = "serde-compat", derive(Serialize))]
#[cfg_attr(feature = "serde-compat", serde(tag = "type"))]
#[cfg_attr(not(feature = "serde-compat"), rescript(tag = "type"))]
enum BarInternally {
    Baz { a: i32, a2: String },
    Biz { b: bool },
    Buz { c: String, d: Option<i32> },
}

#[derive(TS)]
#[cfg_attr(feature = "serde-compat", derive(Serialize))]
struct NestedInternally {
    #[cfg_attr(feature = "serde-compat", serde(flatten))]
    #[cfg_attr(not(feature = "serde-compat"), rescript(flatten))]
    a: FooInternally,
    u: u32,
}

#[derive(TS)]
#[cfg_attr(feature = "serde-compat", derive(Serialize))]
#[rescript(export, export_to = "enum_flattening_nested/externally_tagged/")]
struct NestedInternallyLonely {
    #[cfg_attr(feature = "serde-compat", serde(flatten))]
    #[cfg_attr(not(feature = "serde-compat"), rescript(flatten))]
    a: FooInternally,
}

#[test]
fn internally_tagged() {
    let cfg = Config::from_env();
    assert_eq!(
        FooInternally::inline(&cfg),
        "| Baz({ a: int, a2: string, }) | Biz({ b: bool, }) | Buz({ c: string, d: option<int>, })"
    );

    assert_eq!(
        NestedInternally::inline(&cfg),
        "{ u: int, } & (| Baz({ a: int, a2: string, }) | Biz({ b: bool, }) | Buz({ c: string, d: option<int>, }))"
    );

    assert_eq!(
        NestedInternallyLonely::inline(&cfg),
        "| Baz({ a: int, a2: string, }) | Biz({ b: bool, }) | Buz({ c: string, d: option<int>, })"
    );
}

#[derive(TS)]
#[rescript(export, export_to = "enum_flattening_nested/untagged/")]
#[cfg_attr(feature = "serde-compat", derive(Serialize))]
struct FooUntagged {
    #[cfg_attr(feature = "serde-compat", serde(flatten))]
    #[cfg_attr(not(feature = "serde-compat"), rescript(flatten))]
    baz: BarUntagged,
}

#[derive(TS)]
#[rescript(export, export_to = "enum_flattening_nested/untagged/")]
#[cfg_attr(feature = "serde-compat", derive(Serialize))]
#[cfg_attr(feature = "serde-compat", serde(untagged))]
#[cfg_attr(not(feature = "serde-compat"), rescript(untagged))]
enum BarUntagged {
    Baz { a: i32, a2: String },
    Biz { b: bool },
    Buz { c: String },
}

#[derive(TS)]
#[cfg_attr(feature = "serde-compat", derive(Serialize))]
struct NestedUntagged {
    #[cfg_attr(feature = "serde-compat", serde(flatten))]
    #[cfg_attr(not(feature = "serde-compat"), rescript(flatten))]
    a: FooUntagged,
    u: u32,
}

#[derive(TS)]
#[cfg_attr(feature = "serde-compat", derive(Serialize))]
#[rescript(export, export_to = "enum_flattening_nested/externally_tagged/")]
struct NestedUntaggedLonely {
    #[cfg_attr(feature = "serde-compat", serde(flatten))]
    #[cfg_attr(not(feature = "serde-compat"), rescript(flatten))]
    a: FooUntagged,
}

#[test]
fn untagged() {
    let cfg = Config::from_env();
    assert_eq!(
        FooUntagged::inline(&cfg),
        "| Baz({ a: int, a2: string, }) | Biz({ b: bool, }) | Buz({ c: string, })"
    );

    assert_eq!(
        NestedUntagged::inline(&cfg),
        "{ u: int, } & (| Baz({ a: int, a2: string, }) | Biz({ b: bool, }) | Buz({ c: string, }))"
    );

    assert_eq!(
        NestedUntaggedLonely::inline(&cfg),
        "| Baz({ a: int, a2: string, }) | Biz({ b: bool, }) | Buz({ c: string, })"
    );
}
