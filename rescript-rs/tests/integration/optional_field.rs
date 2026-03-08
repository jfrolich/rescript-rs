#![allow(dead_code)]

use serde::Serialize;
use rescript_rs::{Config, TS};

#[derive(Serialize, TS)]
#[rescript(export, export_to = "optional_field/")]
struct OptionalInStruct {
    #[rescript(optional)]
    a: Option<i32>,
    #[rescript(optional = nullable)]
    b: Option<i32>,
    c: Option<i32>,
}

#[test]
fn in_struct() {
    let a = "a?: int";
    let b = "b?: option<int>";
    let c = "c: option<int>";
    let cfg = Config::from_env();
    assert_eq!(OptionalInStruct::inline(&cfg), format!("{{ {a}, {b}, {c}, }}"));
}

#[derive(Serialize, TS)]
#[rescript(export, export_to = "optional_field/")]
struct GenericOptionalStruct<T> {
    #[rescript(optional)]
    a: Option<T>,
}

#[test]
fn in_generic_struct() {
    let cfg = Config::from_env();
    assert_eq!(
        GenericOptionalStruct::<()>::decl(&cfg),
        "type genericoptionalstruct<T> = { a?: T, }"
    )
}

#[derive(Serialize, TS)]
#[serde(tag = "type")]
#[rescript(export, export_to = "optional_field/")]
enum OptionalInEnum {
    A {
        #[rescript(optional)]
        a: Option<i32>,
    },
    B {
        b: Option<String>,
    },
}

#[test]
fn in_enum() {
    let cfg = Config::from_env();
    assert_eq!(
        OptionalInEnum::inline(&cfg),
        "| A({ a?: int, }) | B({ b: option<string>, })"
    );
}

#[derive(Serialize, TS)]
#[rescript(export, export_to = "optional_field/")]
struct OptionalFlatten {
    #[rescript(optional)]
    a: Option<i32>,
    #[rescript(optional = nullable)]
    b: Option<i32>,
    c: Option<i32>,
}

#[derive(Serialize, TS)]
#[rescript(export, export_to = "optional_field/")]
struct Flatten {
    #[rescript(flatten)]
    x: OptionalFlatten,
}

#[test]
fn flatten() {
    let cfg = Config::from_env();
    assert_eq!(Flatten::inline(&cfg), OptionalFlatten::inline(&cfg));
}

#[derive(Serialize, TS)]
#[rescript(export, export_to = "optional_field/")]
struct OptionalInline {
    #[rescript(optional)]
    a: Option<i32>,
    #[rescript(optional = nullable)]
    b: Option<i32>,
    c: Option<i32>,
}

#[derive(Serialize, TS)]
#[rescript(export, export_to = "optional_field/")]
struct Inline {
    #[rescript(inline)]
    x: OptionalInline,
}

#[test]
fn inline() {
    let a = "a?: int";
    let b = "b?: option<int>";
    let c = "c: option<int>";
    let cfg = Config::from_env();
    assert_eq!(Inline::inline(&cfg), format!("{{ x: {{ {a}, {b}, {c}, }}, }}"));
}

type Foo = Option<i32>;
type Bar<T> = Option<T>;

#[derive(TS)]
#[rescript(export, export_to = "optional_field/", optional_fields)]
struct OptionalStruct {
    a: Option<i32>,
    b: Option<i32>,

    #[rescript(optional = nullable)]
    c: Option<i32>,

    d: i32,

    e: Foo,
    f: Bar<i32>,

    #[rescript(type = "string")]
    g: Option<i32>,

    #[rescript(as = "String")]
    h: Option<i32>,
}

#[test]
fn struct_optional() {
    let cfg = Config::from_env();
    assert_eq!(
        OptionalStruct::inline(&cfg),
        format!(
            "{{ a?: int, b?: int, c?: option<int>, d: int, e?: int, f?: int, g: string, h: string, }}"
        )
    )
}

#[derive(TS)]
#[rescript(export, export_to = "optional_field/", optional_fields = nullable)]
struct NullableStruct {
    a: Option<i32>,
    b: Option<i32>,

    #[rescript(optional = nullable)]
    c: Option<i32>,

    d: i32,

    e: Foo,
    f: Bar<i32>,

    #[rescript(type = "string")]
    g: Option<i32>,

    #[rescript(as = "String")]
    h: Option<i32>,

    // not nullable
    #[rescript(optional)]
    i: Option<i32>,

    // not optional
    #[rescript(optional = false)]
    j: Option<i32>,
}

#[test]
fn struct_nullable() {
    let cfg = Config::from_env();
    assert_eq!(
        NullableStruct::inline(&cfg),
        format!(
            "{{ a?: option<int>, b?: option<int>, c?: option<int>, d: int, e?: option<int>, f?: option<int>, g: string, h: string, i?: int, j: option<int>, }}"
        )
    )
}

#[derive(Serialize, TS)]
#[rescript(export, export_to = "optional_field/")]
struct OptionalInTuple(
    Option<i32>,
    #[rescript(optional)] Option<i32>,
    #[rescript(optional = nullable)] Option<i32>,
);

#[test]
fn in_tuple() {
    let cfg = Config::from_env();
    assert_eq!(
        OptionalInTuple::inline(&cfg),
        format!("(option<int>, (int)?, (option<int>)?)")
    );
}

#[derive(Serialize, TS)]
#[rescript(export, export_to = "optional_field/")]
#[rescript(optional_fields)]
struct OptionalTuple(
    i32,
    #[rescript(type = "string")] Option<i32>,
    #[rescript(as = "String")] Option<i32>,
    Option<i32>,
    #[rescript(optional)] Option<i32>,
    #[rescript(optional = nullable)] Option<i32>,
);

#[test]
fn tuple_optional() {
    let cfg = Config::from_env();
    assert_eq!(
        OptionalTuple::inline(&cfg),
        "(int, string, string, (int)?, (int)?, (option<int>)?)"
    );
}

#[derive(Serialize, TS)]
#[rescript(export, export_to = "optional_field/")]
#[rescript(optional_fields = nullable)]
struct NullableTuple(
    i32,
    #[rescript(type = "string")] Option<i32>,
    #[rescript(as = "String")] Option<i32>,
    Option<i32>,
    #[rescript(optional)] Option<i32>,
    #[rescript(optional = nullable)] Option<i32>,
);

#[test]
fn tuple_nullable() {
    let cfg = Config::from_env();
    assert_eq!(
        NullableTuple::inline(&cfg),
        "(int, string, string, (option<int>)?, (int)?, (option<int>)?)"
    );
}

#[derive(Serialize, TS)]
#[serde(tag = "type")]
#[rescript(export, export_to = "optional_field/", optional_fields)]
enum OptionalFieldsEnum {
    A { a: Option<i32> },
    B { b: String, c: Option<bool> },
}

#[derive(Serialize, TS)]
#[serde(tag = "type")]
#[rescript(export, export_to = "optional_field/", optional_fields = nullable)]
enum OptionalFieldsEnumVariantOverride {
    // Disable `nullable`
    #[rescript(optional_fields)]
    A { a: Option<i32> },

    // Disable `optional_fields`
    #[rescript(optional_fields = false)]
    B { b: String, c: Option<bool> },
}

#[derive(Serialize, TS)]
#[serde(tag = "type")]
#[rescript(export, export_to = "optional_field/", optional_fields)]
enum OptionalFieldsEnumNotNullableVariantOverride {
    // Disable `nullable`
    #[rescript(optional_fields = nullable)]
    A { a: Option<i32> },

    // Disable `optional_fields`
    #[rescript(optional_fields = false)]
    B { b: String, c: Option<bool> },
}

#[derive(Serialize, TS)]
#[rescript(export, export_to = "optional_field/", optional_fields, tag = "type")]
enum OptionalFieldsTaggedEnum {
    A { a: Option<i32> },
    B { b: String, c: Option<bool> },
}

#[derive(Serialize, TS)]
#[rescript(
    export,
    export_to = "optional_field/",
    optional_fields,
    tag = "type",
    content = "data"
)]
enum OptionalFieldsExternallyTaggedEnum {
    A { a: Option<i32> },
    B { b: String, c: Option<bool> },
}

#[test]
fn optional_fields_enum() {
    let cfg = Config::from_env();
    assert_eq!(
        OptionalFieldsEnum::inline(&cfg),
        "| A({ a?: int, }) | B({ b: string, c?: bool, })"
    );

    assert_eq!(
        OptionalFieldsEnumVariantOverride::inline(&cfg),
        "| A({ a?: int, }) | B({ b: string, c: option<bool>, })"
    );

    assert_eq!(
        OptionalFieldsEnumNotNullableVariantOverride::inline(&cfg),
        "| A({ a?: option<int>, }) | B({ b: string, c: option<bool>, })"
    );

    assert_eq!(
        OptionalFieldsTaggedEnum::inline(&cfg),
        "| A({ a?: int, }) | B({ b: string, c?: bool, })"
    );

    assert_eq!(
        OptionalFieldsExternallyTaggedEnum::inline(&cfg),
        "| A({ data: { a?: int, } }) | B({ data: { b: string, c?: bool, } })"
    );
}
