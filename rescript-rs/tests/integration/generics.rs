#![allow(clippy::box_collection, clippy::enum_variant_names, dead_code)]

use std::{
    collections::{BTreeMap, HashSet},
    fmt::Debug,
    rc::Rc,
};

#[cfg(feature = "serde-compat")]
use serde::Serialize;
use rescript_rs::{Config, TS};

#[derive(TS)]
#[rescript(export, export_to = "generics/")]
struct Generic<T>
where
    T: TS,
{
    value: T,
    values: Vec<T>,
}

#[derive(TS)]
#[rescript(export, export_to = "generics/")]
struct GenericAutoBound<T> {
    value: T,
    values: Vec<T>,
}

#[derive(TS)]
#[rescript(export, export_to = "generics/")]
struct GenericAutoBound2<T>
where
    T: PartialEq,
{
    value: T,
    values: Vec<T>,
}

#[derive(TS)]
#[rescript(export, export_to = "generics/")]
struct Container {
    foo: Generic<u32>,
    bar: Box<HashSet<Generic<u32>>>,
    baz: Box<BTreeMap<String, Rc<Generic<String>>>>,
}

macro_rules! declare {
    ($(#[$meta:meta])* $name:ident { $($fident:ident: $t:ty),+ $(,)? }) => {
        $(#[$meta])*
        struct $name {
            $(pub $fident: $t),+
        }
    }
}

declare! {
    #[derive(TS)]
    #[rescript(export, export_to = "generics/")]
    TypeGroup {
        foo: Vec<Container>,
    }
}

#[test]
fn test() {
    let cfg = Config::from_env();
    assert_eq!(
        TypeGroup::decl(&cfg),
        "type typegroup = { foo: array<Container>, }",
    );

    assert_eq!(
        Generic::<()>::decl(&cfg),
        "type generic<T> = { value: T, values: array<T>, }"
    );

    assert_eq!(
        GenericAutoBound::<()>::decl(&cfg),
        "type genericautobound<T> = { value: T, values: array<T>, }"
    );

    assert_eq!(
        GenericAutoBound2::<()>::decl(&cfg),
        "type genericautobound2<T> = { value: T, values: array<T>, }"
    );

    assert_eq!(
        Container::decl(&cfg),
        "type container = { foo: Generic<int>, bar: array<Generic<int>>, baz: Dict.t<Generic<string>>, }"
    );
}

#[derive(TS)]
#[cfg_attr(feature = "serde-compat", derive(Serialize))]
#[cfg_attr(feature = "serde-compat", serde(tag = "type", content = "value"))]
#[cfg_attr(not(feature = "serde-compat"), rescript(tag = "type", content = "value"))]
#[rescript(export, export_to = "generics/")]
enum GenericEnum<A, B, C> {
    A(A),
    B(B, B, B),
    C(Vec<C>),
    D(Vec<Vec<Vec<A>>>),
    E { a: A, b: B, c: C },
    X(Vec<i32>),
    Y(i32),
    Z(Vec<Vec<i32>>),
}

#[test]
fn generic_enum() {
    let cfg = Config::from_env();
    assert_eq!(
        GenericEnum::<(), (), ()>::decl(&cfg),
        "@tag(\"type\")\ntype genericenum<A, B, C> = | A({ value: A }) | B({ value: (B, B, B) }) | C({ value: array<C> }) | D({ value: array<array<array<A>>> }) | E({ value: { a: A, b: B, c: C, } }) | X({ value: array<int> }) | Y({ value: int }) | Z({ value: array<array<int>> })"
    )
}

#[derive(TS)]
#[rescript(export, export_to = "generics/")]
struct NewType<T>(Vec<Vec<T>>);

#[test]
fn generic_newtype() {
    let cfg = Config::from_env();
    assert_eq!(
        NewType::<()>::decl(&cfg),
        "type newtype<T> = array<array<T>>"
    );
}

#[derive(TS)]
#[rescript(export, export_to = "generics/")]
struct Tuple<T>(T, Vec<T>, Vec<Vec<T>>);

#[test]
fn generic_tuple() {
    let cfg = Config::from_env();
    assert_eq!(
        Tuple::<()>::decl(&cfg),
        "type tuple<T> = (T, array<T>, array<array<T>>)"
    );
}

#[derive(TS)]
#[rescript(export, export_to = "generics/")]
struct Struct<T> {
    a: T,
    b: (T, T),
    c: (T, (T, T)),
    d: [T; 3],
    e: [(T, T); 3],
    f: Vec<T>,
    g: Vec<Vec<T>>,
    h: Vec<[(T, T); 3]>,
}

#[test]
fn generic_struct() {
    let cfg = Config::from_env();
    assert_eq!(
        Struct::<()>::decl(&cfg),
        "type struct<T> = { a: T, b: (T, T), c: (T, (T, T)), d: (T, T, T), e: ((T, T), (T, T), (T, T)), f: array<T>, g: array<array<T>>, h: array<((T, T), (T, T), (T, T))>, }"
    )
}

#[derive(TS)]
#[rescript(export, export_to = "generics/")]
struct GenericInline<T> {
    t: T,
}

#[derive(TS)]
#[rescript(export, export_to = "generics/")]
struct ContainerInline {
    g: GenericInline<String>,
    #[rescript(inline)]
    gi: GenericInline<String>,
    #[rescript(flatten)]
    t: GenericInline<Vec<String>>,
}

#[test]
fn inline() {
    let cfg = Config::from_env();
    assert_eq!(
        GenericInline::<()>::decl(&cfg),
        "type genericinline<T> = { t: T, }"
    );
    assert_eq!(
        ContainerInline::decl(&cfg),
        "type containerinline = { g: GenericInline<string>, gi: { t: string, }, t: array<string>, }"
    );
}

#[derive(TS)]
#[rescript(export, export_to = "generics/")]
struct GenericWithBounds<T: ToString> {
    t: T,
}

#[derive(TS)]
#[rescript(export, export_to = "generics/")]
struct ContainerWithBounds {
    g: GenericWithBounds<String>,

    #[rescript(inline)]
    gi: GenericWithBounds<String>,

    #[rescript(flatten)]
    t: GenericWithBounds<u32>,
}

#[test]
fn inline_with_bounds() {
    let cfg = Config::from_env();
    assert_eq!(
        GenericWithBounds::<&'static str>::decl(&cfg),
        "type genericwithbounds<T> = { t: T, }"
    );
    assert_eq!(
        ContainerWithBounds::decl(&cfg),
        "type containerwithbounds = { g: GenericWithBounds<string>, gi: { t: string, }, t: int, }"
    );
}

#[derive(TS)]
#[rescript(export, export_to = "generics/")]
struct GenericWithDefault<T = String> {
    t: T,
}

#[derive(TS)]
#[rescript(export, export_to = "generics/")]
struct ContainerWithDefault {
    g: GenericWithDefault<String>,

    #[rescript(inline)]
    gi: GenericWithDefault<String>,

    #[rescript(flatten)]
    t: GenericWithDefault<u32>,
}

#[test]
fn inline_with_default() {
    let cfg = Config::from_env();
    assert_eq!(
        GenericWithDefault::<()>::decl(&cfg),
        "type genericwithdefault<T = string> = { t: T, }"
    );
    assert_eq!(
        ContainerWithDefault::decl(&cfg),
        "type containerwithdefault = { g: GenericWithDefault<string>, gi: { t: string, }, t: int, }"
    );
}

#[derive(TS)]
#[rescript(export, export_to = "generics/")]
struct ADefault<T = String> {
    t: T,
}

#[derive(TS)]
#[rescript(export, export_to = "generics/")]
struct BDefault<U = Option<ADefault<i32>>> {
    u: U,
}

#[derive(TS)]
#[rescript(export, export_to = "generics/")]
struct YDefault {
    a1: ADefault,
    a2: ADefault<i32>,
}

#[test]
fn default() {
    let cfg = Config::from_env();
    assert_eq!(
        ADefault::<()>::decl(&cfg),
        "type adefault<T = string> = { t: T, }"
    );

    assert_eq!(
        BDefault::<()>::decl(&cfg),
        "type bdefault<U = option<ADefault<int>>> = { u: U, }"
    );
    assert!(BDefault::<()>::dependencies(&cfg)
        .iter()
        .any(|dep| dep.ts_name == "ADefault"));

    assert_eq!(
        YDefault::decl(&cfg),
        "type ydefault = { a1: ADefault<string>, a2: ADefault<int>, }"
    )
}

#[derive(TS)]
#[rescript(export, export_to = "generics/")]
struct ATraitBounds<T: ToString = i32> {
    t: T,
}

#[derive(TS)]
#[rescript(export, export_to = "generics/")]
struct BTraitBounds<T: ToString + Debug + Clone + 'static>(T);

#[derive(TS)]
#[cfg_attr(feature = "serde-compat", derive(Serialize))]
#[cfg_attr(feature = "serde-compat", serde(tag = "type", content = "value"))]
#[cfg_attr(not(feature = "serde-compat"), rescript(tag = "type", content = "value"))]
#[rescript(export, export_to = "generics/")]
enum CTraitBounds<T: Copy + Clone + PartialEq, K: Copy + PartialOrd = i32> {
    A { t: T },
    B(T),
    C,
    D(T, K),
}

// Types with const generics can't be exported
#[derive(TS)]
struct DTraitBounds<T: ToString, const N: usize> {
    t: [T; N],
}

#[test]
fn trait_bounds() {
    let cfg = Config::from_env();
    assert_eq!(
        ATraitBounds::<i32>::decl(&cfg),
        "type atraitbounds<T = int> = { t: T, }"
    );

    assert_eq!(
        BTraitBounds::<&'static str>::decl(&cfg),
        "type btraitbounds<T> = T"
    );

    assert_eq!(
        CTraitBounds::<&'static str, i32>::decl(&cfg),
        "@tag(\"type\")\ntype ctraitbounds<T, K = int> = | A({ value: { t: T, } }) | B({ value: T }) | C | D({ value: (T, K) })"
    );

    let ty = format!(
        "type dtraitbounds<T> = {{ t: ({}), }}",
        "T, ".repeat(41).trim_end_matches(", ")
    );
    assert_eq!(DTraitBounds::<&str, 41>::decl(&cfg), ty)
}

#[derive(TS)]
#[rescript(export, export_to = "generics/")]
struct T0<T> {
    t0: T,
}

#[derive(TS)]
#[rescript(export, export_to = "generics/")]
struct P0<T> {
    p0: T,
}

#[derive(TS)]
#[rescript(export, export_to = "generics/")]
struct T1<T> {
    t0: T,
}

#[derive(TS)]
#[rescript(export, export_to = "generics/")]
struct P1<T> {
    p0: T,
}

#[derive(TS)]
#[rescript(export, export_to = "generics/")]
struct Parent {
    a: T1<T0<u32>>,
    b: T1<P1<T0<P0<u32>>>>,
    c: T1<P1<()>>,
}

#[derive(TS)]
#[rescript(export, export_to = "generics/")]
struct GenericParent<T> {
    a_t: T1<T0<T>>,
    b_t: T1<P1<T0<P0<T>>>>,
    c_t: T1<P1<T>>,
    a_null: T1<T0<()>>,
    b_null: T1<P1<T0<P0<()>>>>,
    c_null: T1<P1<()>>,
}

#[test]
fn deeply_nested() {
    let cfg = Config::from_env();
    assert_eq!(
        Parent::inline(&cfg),
        "{ a: T1<T0<int>>, b: T1<P1<T0<P0<int>>>>, c: T1<P1<unit>>, }"
    );
    assert_eq!(
        GenericParent::<()>::decl(&cfg),
        "type genericparent<T> = { \
            a_t: T1<T0<T>>, \
            b_t: T1<P1<T0<P0<T>>>>, \
            c_t: T1<P1<T>>, \
            a_null: T1<T0<unit>>, \
            b_null: T1<P1<T0<P0<unit>>>>, \
            c_null: T1<P1<unit>>, \
         }"
    );
}

#[derive(TS)]
#[rescript(export, export_to = "generics/")]
struct SomeType(String);

#[derive(TS)]
#[cfg_attr(feature = "serde-compat", derive(Serialize))]
#[cfg_attr(feature = "serde-compat", serde(tag = "type", content = "value"))]
#[cfg_attr(not(feature = "serde-compat"), rescript(tag = "type", content = "value"))]
#[rescript(export, export_to = "generics/")]
enum MyEnum<A, B> {
    VariantA(A),
    VariantB(B),
}

#[derive(TS)]
#[rescript(export, export_to = "generics/")]
struct ParentEnum {
    e: MyEnum<i32, i32>,
    #[rescript(inline)]
    e1: MyEnum<i32, SomeType>,
}

#[test]
fn inline_generic_enum() {
    let cfg = Config::from_env();
    assert_eq!(
        ParentEnum::decl(&cfg),
        "type parentenum = { \
            e: MyEnum<int, int>, \
            e1: | VariantA({ value: int }) | VariantB({ value: SomeType }), \
        }"
    );
}
