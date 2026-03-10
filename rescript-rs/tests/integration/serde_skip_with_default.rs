#![cfg(feature = "serde-compat")]
#![allow(dead_code)]

use rescript_rs::{Config, TS};
use serde::{Deserialize, Serialize};

fn default_http_version() -> String {
    "2".to_owned()
}

#[derive(Debug, Clone, Deserialize, Serialize, TS)]
#[rescript(export, export_to = "serde_skip_with_default/")]
pub struct Foobar {
    #[serde(skip, default = "default_http_version")]
    pub http_version: String,
    pub something_else: i32,
}

#[test]
fn serde_skip_with_default() {
    let cfg = Config::from_env();
    assert_eq!(
        Foobar::decl(&cfg),
        "type foobar = {\n  something_else: int,\n}"
    );
}
