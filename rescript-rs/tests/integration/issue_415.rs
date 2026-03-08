#![cfg(feature = "serde-compat")]

use rescript_rs::{Config, TS};

struct Foreign;

#[derive(TS)]
#[rescript(export, export_to = "issue_415/")]
struct Issue415 {
    #[rescript(optional, type = "Date")]
    a: Option<Foreign>,
}

#[test]
fn issue_415() {
    let cfg = Config::from_env();
    assert_eq!(Issue415::decl(&cfg), "type issue415 = { a?: Date, }");
}

#[derive(TS)]
#[rescript(export, export_to = "issue_415/")]
struct InTuple(i32, #[rescript(optional, type = "Date")] Option<Foreign>);

#[test]
fn in_tuple() {
    let cfg = Config::from_env();
    assert_eq!(InTuple::decl(&cfg), "type intuple = (int, (Date)?)");
}
