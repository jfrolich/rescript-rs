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
        "type rangetest = { \
            a: { start: int, end: int }, \
            b: { start: string, end: string }, \
            c: { start: { start: int, end: int }, end: { start: int, end: int } }, \
            d: { start: int, end: int }, \
            e: { start: Inner, end: Inner }, \
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
