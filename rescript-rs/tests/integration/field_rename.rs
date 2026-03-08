#![allow(dead_code)]

use rescript_rs::{Config, TS};

#[derive(TS)]
#[cfg_attr(feature = "serde-compat", derive(serde::Serialize, serde::Deserialize))]
struct Rename {
    #[cfg_attr(
        feature = "serde-compat",
        serde(rename = "c", skip_serializing_if = "String::is_empty")
    )]
    a: String,
    #[rescript(rename = "bb")]
    b: i32,
}

#[test]
fn test() {
    let cfg = Config::from_env();
    if (cfg!(feature = "serde-compat")) {
        assert_eq!(Rename::inline(&cfg), "{ c: string, bb: int, }")
    } else {
        assert_eq!(Rename::inline(&cfg), "{ a: string, bb: int, }")
    }
}
