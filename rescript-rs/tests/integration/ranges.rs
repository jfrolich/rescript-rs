#![allow(dead_code)]

use std::{
    collections::BTreeSet,
    ops::{Range, RangeInclusive},
};

use rescript_rs::{Config, Dependency, TS};

#[derive(TS)]
#[rescript(export, export_to = "ranges/")]
struct Inner(i32);

#[derive(TS)]
#[rescript(export, export_to = "ranges/")]
struct RangeTest {
    a: Range<u32>,
    b: Range<&'static str>,
    c: Range<Range<i32>>,
    d: RangeInclusive<u32>,
    e: Range<Inner>,
}

#[test]
fn range() {
    let cfg = Config::from_env();
    assert_eq!(
        RangeTest::decl(&cfg),
        "type rangeTest = {\n  \
            a: { start: int, end: int },\n  \
            b: { start: string, end: string },\n  \
            c: { start: { start: int, end: int }, end: { start: int, end: int } },\n  \
            d: { start: int, end: int },\n  \
            e: { start: inner, end: inner },\n\
        }"
    );
    assert_eq!(
        RangeTest::dependencies(&cfg)
            .into_iter()
            .collect::<BTreeSet<_>>()
            .into_iter()
            .collect::<Vec<_>>(),
        vec![Dependency::from_ty::<Inner>(&cfg).unwrap(),]
    );
}
