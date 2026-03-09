#![allow(clippy::box_collection, clippy::enum_variant_names, dead_code)]

use std::{
    collections::{BTreeMap, HashSet},
    fmt::Debug,
    rc::Rc,
};

use rescript_rs::{Config, TS};
#[cfg(feature = "serde-compat")]
use serde::Serialize;

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
        "type typeGroup = {\n  foo: array<container>,\n}",
    );

    assert_eq!(
        Generic::<()>::decl(&cfg),
        "type generic<T> = {\n  value: T,\n  values: array<T>,\n}"
    );

    assert_eq!(
        GenericAutoBound::<()>::decl(&cfg),
        "type genericAutoBound<T> = {\n  value: T,\n  values: array<T>,\n}"
    );

    assert_eq!(
        GenericAutoBound2::<()>::decl(&cfg),
        "type genericAutoBound2<T> = {\n  value: T,\n  values: array<T>,\n}"
    );

    assert_eq!(
        Container::decl(&cfg),
        "type container = {\n  foo: generic<int>,\n  bar: array<generic<int>>,\n  baz: Dict.t<generic<string>>,\n}"
    );
}

#[derive(TS)]
#[cfg_attr(feature = "serde-compat", derive(Serialize))]
#[cfg_attr(feature = "serde-compat", serde(tag = "type", content = "value"))]
#[cfg_attr(
    not(feature = "serde-compat"),
    rescript(tag = "type", content = "value")
)]
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
        "@tag(\"type\")\ntype genericEnum<A, B, C> = \n  | A({ value: A })\n  | B({ value: (B, B, B) })\n  | C({ value: array<C> })\n  | D({ value: array<array<array<A>>> })\n  | E({ value: { a: A, b: B, c: C, } })\n  | X({ value: array<int> })\n  | Y({ value: int })\n  | Z({ value: array<array<int>> })"
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
        "type newType<T> = array<array<T>>"
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
        "type struct<T> = {\n  a: T,\n  b: (T, T),\n  c: (T, (T, T)),\n  d: (T, T, T),\n  e: ((T, T), (T, T), (T, T)),\n  f: array<T>,\n  g: array<array<T>>,\n  h: array<((T, T), (T, T), (T, T))>,\n}"
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
        "type genericInline<T> = {\n  t: T,\n}"
    );
    assert_eq!(
        ContainerInline::decl(&cfg),
        "type containerInline = { g: genericInline<string>, gi: { t: string, }, t: array<string>, }"
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
        "type genericWithBounds<T> = {\n  t: T,\n}"
    );
    assert_eq!(
        ContainerWithBounds::decl(&cfg),
        "type containerWithBounds = { g: genericWithBounds<string>, gi: { t: string, }, t: int, }"
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
        "type genericWithDefault<T = string> = {\n  t: T,\n}"
    );
    assert_eq!(
        ContainerWithDefault::decl(&cfg),
        "type containerWithDefault = { g: genericWithDefault<string>, gi: { t: string, }, t: int, }"
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
        "type aDefault<T = string> = {\n  t: T,\n}"
    );

    assert_eq!(
        BDefault::<()>::decl(&cfg),
        "type bDefault<U = option<aDefault<int>>> = {\n  u: U,\n}"
    );
    assert!(BDefault::<()>::dependencies(&cfg)
        .iter()
        .any(|dep| dep.ts_name == "ADefault"));

    assert_eq!(
        YDefault::decl(&cfg),
        "type yDefault = {\n  a1: aDefault<string>,\n  a2: aDefault<int>,\n}"
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
#[cfg_attr(
    not(feature = "serde-compat"),
    rescript(tag = "type", content = "value")
)]
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
        "type aTraitBounds<T = int> = {\n  t: T,\n}"
    );

    assert_eq!(
        BTraitBounds::<&'static str>::decl(&cfg),
        "type bTraitBounds<T> = T"
    );

    assert_eq!(
        CTraitBounds::<&'static str, i32>::decl(&cfg),
        "@tag(\"type\")\ntype cTraitBounds<T, K = int> = \n  | A({ value: { t: T, } })\n  | B({ value: T })\n  | C\n  | D({ value: (T, K) })"
    );

    let ty = format!(
        "type dTraitBounds<T> = {{\n  t: ({}),\n}}",
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
        "{ a: t1<t0<int>>, b: t1<p1<t0<p0<int>>>>, c: t1<p1<unit>>, }"
    );
    assert_eq!(
        GenericParent::<()>::decl(&cfg),
        "type genericParent<T> = {\n  \
            a_t: t1<t0<T>>,\n  \
            b_t: t1<p1<t0<p0<T>>>>,\n  \
            c_t: t1<p1<T>>,\n  \
            a_null: t1<t0<unit>>,\n  \
            b_null: t1<p1<t0<p0<unit>>>>,\n  \
            c_null: t1<p1<unit>>,\n\
         }"
    );
}

#[derive(TS)]
#[rescript(export, export_to = "generics/")]
struct SomeType(String);

#[derive(TS)]
#[cfg_attr(feature = "serde-compat", derive(Serialize))]
#[cfg_attr(feature = "serde-compat", serde(tag = "type", content = "value"))]
#[cfg_attr(
    not(feature = "serde-compat"),
    rescript(tag = "type", content = "value")
)]
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
        "type parentEnum = {\n  \
            e: myEnum<int, int>,\n  \
            e1: \n  | VariantA({ value: int })\n  | VariantB({ value: someType }),\n\
        }"
    );
}
