#![allow(dead_code)]

use rescript_rs::{Config, TS};

#[derive(TS)]
#[rescript(export, export_to = "references/")]
struct FullOfRefs<'a> {
    str_slice: &'a str,
    ref_slice: &'a [&'a str],
    num_ref: &'a i32,
}

#[test]
fn references() {
    let cfg = Config::from_env();
    assert_eq!(
        FullOfRefs::inline(&cfg),
        "{ str_slice: string, ref_slice: array<string>, num_ref: int, }"
    )
}
