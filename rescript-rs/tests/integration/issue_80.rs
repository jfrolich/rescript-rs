use rescript_rs::{Config, TS};
use serde::Serialize;

#[derive(TS, Serialize)]
#[serde(tag = "type")]
#[rescript(export, export_to = "issue_80/")]
pub enum SomeTypeList {
    Value1 {
        #[serde(skip)]
        #[rescript(skip)]
        skip_this: String,
    },
    Value2,
}

#[test]
fn issue_80() {
    let cfg = Config::from_env();
    let ty = SomeTypeList::inline(&cfg);
    assert_eq!(ty, "\n  | Value1({  })\n  | Value2");
}
