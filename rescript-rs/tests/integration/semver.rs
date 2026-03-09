#![allow(dead_code)]
#![cfg(feature = "semver-impl")]

use rescript_rs::{Config, TS};
use semver::Version;

#[derive(TS)]
#[rescript(export, export_to = "semver/")]
struct Semver {
    version: Version,
}

#[test]
fn semver() {
    let cfg = Config::from_env();
    assert_eq!(Semver::decl(&cfg), "type Semver = { version: string, };")
}
