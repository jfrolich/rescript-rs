#![allow(dead_code)]
use std::{collections::HashMap, sync::Arc};

#[cfg(feature = "serde-compat")]
use serde::Serialize;
use rescript_rs::{Config, TS};

#[derive(TS)]
#[rescript(export, export_to = "self_referential/")]
struct HasT {
    t: &'static T<'static>,
}

#[derive(TS)]
#[rescript(export, export_to = "self_referential/")]
struct T<'a> {
    t_box: Box<T<'a>>,
    self_box: Box<Self>,

    t_ref: &'a T<'a>,
    self_ref: &'a Self,

    t_arc: Arc<T<'a>>,
    self_arc: Arc<Self>,

    #[rescript(inline)]
    has_t: HasT,
}

#[test]
fn named() {
    let cfg = Config::from_env();
    assert_eq!(
        T::decl(&cfg),
        "type t = { \
            t_box: T, \
            self_box: T, \
            t_ref: T, \
            self_ref: T, \
            t_arc: T, \
            self_arc: T, \
            has_t: { t: T, }, \
         }"
    );
}

#[derive(TS)]
#[rescript(export, export_to = "self_referential/", rename = "E", tag = "type", content = "value")]
enum ExternallyTagged {
    A(Box<ExternallyTagged>),
    B(&'static ExternallyTagged),
    C(Box<Self>),
    D(&'static Self),
    E(
        Box<ExternallyTagged>,
        Box<Self>,
        &'static ExternallyTagged,
        &'static Self,
    ),
    F {
        a: Box<Self>,
        b: &'static ExternallyTagged,
        c: HashMap<String, ExternallyTagged>,
        d: Option<Arc<ExternallyTagged>>,
        #[rescript(optional = nullable)]
        e: Option<Arc<ExternallyTagged>>,
        #[rescript(optional)]
        f: Option<Arc<ExternallyTagged>>,
    },

    G(
        Vec<ExternallyTagged>,
        Box<[&'static ExternallyTagged; 1024]>,
        HashMap<String, ExternallyTagged>,
    ),
}

#[test]
fn enum_externally_tagged() {
    let cfg = Config::from_env();
    assert_eq!(
        ExternallyTagged::decl(&cfg),
        "@tag(\"type\")\n\
         type e = \
         | A({ value: E }) \
         | B({ value: E }) \
         | C({ value: E }) \
         | D({ value: E }) \
         | E({ value: (E, E, E, E) }) \
         | F({ value: { a: E, b: E, c: Dict.t<E>, d: option<E>, e?: option<E>, f?: E, } }) \
         | G({ value: (array<E>, array<E>, Dict.t<E>) })"
    );
}

#[derive(TS)]
#[cfg_attr(feature = "serde-compat", derive(Serialize))]
#[rescript(rename = "I")]
#[cfg_attr(feature = "serde-compat", serde(tag = "tag"))]
#[cfg_attr(not(feature = "serde-compat"), rescript(tag = "tag"))]
enum InternallyTagged {
    A(Box<InternallyTagged>),
    B(&'static InternallyTagged),
    C(Box<Self>),
    D(&'static Self),
    E(Vec<Self>),
    F {
        a: Box<Self>,
        b: &'static InternallyTagged,
        c: HashMap<InternallyTagged, InternallyTagged>,
        d: Option<&'static InternallyTagged>,
        #[rescript(optional = nullable)]
        e: Option<&'static InternallyTagged>,
        #[rescript(optional)]
        f: Option<&'static InternallyTagged>,
    },
}

// NOTE: The generated type is actually not valid TS here, since the indirections rust enforces for recursive types
//       gets lost during the translation to TypeScript (e.g "Box<T>" => "T").
#[test]
fn enum_internally_tagged() {
    let cfg = Config::from_env();
    assert_eq!(
        InternallyTagged::decl(&cfg),
        "@tag(\"tag\")\n\
         type i = \
         | A(I) \
         | B(I) \
         | C(I) \
         | D(I) \
         | E(array<I>) \
         | F({ a: I, b: I, c: Dict.t<I>, d: option<I>, e?: option<I>, f?: I, })"
    );
}

#[derive(TS)]
#[cfg_attr(feature = "serde-compat", derive(Serialize))]
#[rescript(export, export_to = "self_referential/", rename = "A")]
#[cfg_attr(feature = "serde-compat", serde(tag = "tag", content = "content"))]
#[cfg_attr(not(feature = "serde-compat"), rescript(tag = "tag", content = "content"))]
enum AdjacentlyTagged {
    A(Box<AdjacentlyTagged>),
    B(&'static AdjacentlyTagged),
    C(Box<Self>),
    D(&'static Self),
    E(Vec<Self>),
    F {
        a: Box<Self>,
        b: &'static AdjacentlyTagged,
        c: HashMap<String, AdjacentlyTagged>,
        d: Option<&'static AdjacentlyTagged>,
        #[rescript(optional = nullable)]
        e: Option<&'static AdjacentlyTagged>,
        #[rescript(optional)]
        f: Option<&'static AdjacentlyTagged>,
    },
    G(
        Vec<Self>,
        [&'static AdjacentlyTagged; 4],
        HashMap<String, AdjacentlyTagged>,
    ),
}

// NOTE: The generated type is actually not valid TS here, since the indirections rust enforces for recursive types
//       gets lost during the translation to TypeScript (e.g "Box<T>" => "T").
#[test]
fn enum_adjacently_tagged() {
    let cfg = Config::from_env();
    assert_eq!(
        AdjacentlyTagged::decl(&cfg),
        "@tag(\"tag\")\n\
         type a = \
         | A({ content: A }) \
         | B({ content: A }) \
         | C({ content: A }) \
         | D({ content: A }) \
         | E({ content: array<A> }) \
         | F({ content: { a: A, b: A, c: Dict.t<A>, d: option<A>, e?: option<A>, f?: A, } }) \
         | G({ content: (array<A>, (A, A, A, A), Dict.t<A>) })"
    );
}
