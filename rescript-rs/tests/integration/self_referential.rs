#![allow(dead_code)]
use std::{collections::HashMap, sync::Arc};

use rescript_rs::{Config, TS};
#[cfg(feature = "serde-compat")]
use serde::Serialize;

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
        "type t = {\n  \
            t_box: t,\n  \
            self_box: t,\n  \
            t_ref: t,\n  \
            self_ref: t,\n  \
            t_arc: t,\n  \
            self_arc: t,\n  \
            has_t: { t: t, },\n\
         }"
    );
}

#[derive(TS)]
#[rescript(
    export,
    export_to = "self_referential/",
    rename = "E",
    tag = "type",
    content = "value"
)]
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
         type e = \n  \
         | A({ value: e })\n  \
         | B({ value: e })\n  \
         | C({ value: e })\n  \
         | D({ value: e })\n  \
         | E({ value: (e, e, e, e) })\n  \
         | F({ value: { a: e, b: e, c: Dict.t<e>, d: option<e>, e?: option<e>, f?: e, } })\n  \
         | G({ value: (array<e>, array<e>, Dict.t<e>) })"
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
         type i = \n  \
         | A(i)\n  \
         | B(i)\n  \
         | C(i)\n  \
         | D(i)\n  \
         | E(array<i>)\n  \
         | F({ a: i, b: i, c: Dict.t<i>, d: option<i>, e?: option<i>, f?: i, })"
    );
}

#[derive(TS)]
#[cfg_attr(feature = "serde-compat", derive(Serialize))]
#[rescript(export, export_to = "self_referential/", rename = "A")]
#[cfg_attr(feature = "serde-compat", serde(tag = "tag", content = "content"))]
#[cfg_attr(
    not(feature = "serde-compat"),
    rescript(tag = "tag", content = "content")
)]
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
         type a = \n  \
         | A({ content: a })\n  \
         | B({ content: a })\n  \
         | C({ content: a })\n  \
         | D({ content: a })\n  \
         | E({ content: array<a> })\n  \
         | F({ content: { a: a, b: a, c: Dict.t<a>, d: option<a>, e?: option<a>, f?: a, } })\n  \
         | G({ content: (array<a>, (a, a, a, a), Dict.t<a>) })"
    );
}
