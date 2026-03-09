#![allow(dead_code)]
use rescript_rs::{Config, TS};

#[derive(TS)]
#[rescript(export, export_to = "list/")]
struct List {
    data: Option<Vec<u32>>,
}

#[test]
fn list() {
    let cfg = Config::from_env();
    assert_eq!(
        List::decl(&cfg),
        "type list = {\n  data: option<array<int>>,\n}"
    );
}
