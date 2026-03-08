#![allow(dead_code)]

use std::{
    cell::UnsafeCell, mem::MaybeUninit, ptr::NonNull, sync::atomic::AtomicPtr, time::Instant,
};

use serde::Serialize;
use rescript_rs::{Config, TS};

type Unsupported = UnsafeCell<MaybeUninit<NonNull<AtomicPtr<i32>>>>;

#[derive(TS)]
#[rescript(export, export_to = "type_as/")]
struct ExternalTypeDef {
    a: i32,
    b: i32,
    c: i32,
}

#[derive(TS)]
#[rescript(export, export_to = "type_as/")]
struct Override {
    a: i32,
    #[rescript(as = "ExternalTypeDef")]
    #[rescript(inline)]
    x: Instant,
    // here, 'as' just behaves like 'type' (though it adds a dependency!)
    #[rescript(as = "ExternalTypeDef")]
    y: Unsupported,
    #[rescript(as = "(i32, ExternalTypeDef, i32)")]
    z: Unsupported,
}

#[test]
fn struct_properties() {
    let cfg = Config::from_env();
    assert_eq!(
        Override::inline(&cfg),
        "{ \
           a: int, \
           x: { a: int, b: int, c: int, }, \
           y: ExternalTypeDef, \
           z: (int, ExternalTypeDef, int), \
        }"
    );
    assert!(Override::dependencies(&cfg)
        .iter()
        .any(|d| d.ts_name == "ExternalTypeDef"));
}

#[derive(TS)]
#[rescript(export, export_to = "type_as/", tag = "type", content = "value")]
enum OverrideEnum {
    A(#[rescript(as = "ExternalTypeDef")] Instant),
    B {
        #[rescript(as = "ExternalTypeDef")]
        x: Unsupported,
        y: i32,
        z: i32,
    },
}

mod deser {
    use serde::{Serialize, Serializer};

    use super::Instant;
    pub fn serialize<S: Serializer>(field: &Instant, serializer: S) -> Result<S::Ok, S::Error> {
        #[derive(Serialize)]
        struct Foo {
            x: i32,
        }
        Foo { x: 0 }.serialize(serializer)
    }
}

#[derive(TS)]
struct OverrideVariantDef {
    x: i32,
}

#[derive(TS, Serialize)]
#[serde(tag = "type", content = "value")]
#[rescript(export, export_to = "type_as/")]
enum OverrideVariant {
    #[rescript(as = "OverrideVariantDef")]
    #[serde(with = "deser")]
    A {
        x: Instant,
    },
    B {
        y: i32,
        z: i32,
    },
}

#[test]
fn enum_variants() {
    let a = OverrideVariant::A { x: Instant::now() };
    assert_eq!(serde_json::to_string(&a).unwrap(), r#"{"type":"A","value":{"x":0}}"#);
    let cfg = Config::from_env();
    assert_eq!(
        OverrideEnum::inline(&cfg),
        "| A({ value: ExternalTypeDef }) | B({ value: { x: ExternalTypeDef, y: int, z: int, } })"
    );

    assert_eq!(
        OverrideVariant::inline(&cfg),
        "| A({ value: OverrideVariantDef }) | B({ value: { y: int, z: int, } })"
    );
}

#[derive(TS)]
#[rescript(export, export_to = "type_as/")]
struct Outer {
    #[rescript(as = "Option<ExternalTypeDef>")]
    #[rescript(optional = nullable, inline)]
    x: Unsupported,
    #[rescript(as = "Option<ExternalTypeDef>")]
    #[rescript(optional = nullable)]
    y: Unsupported,
}

#[test]
fn complex() {
    let cfg = Config::from_env();
    let external = ExternalTypeDef::inline(&cfg);
    assert_eq!(
        Outer::inline(&cfg),
        format!("{{ x?: option<{external}>, y?: option<ExternalTypeDef>, }}")
    )
}
