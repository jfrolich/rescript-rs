#![allow(dead_code)]

use rescript_rs::{Config, TS};
#[cfg(feature = "serde-compat")]
use serde::Deserialize;

#[derive(TS)]
#[cfg_attr(feature = "serde-compat", derive(Deserialize))]
#[cfg_attr(feature = "serde-compat", serde(untagged))]
#[cfg_attr(not(feature = "serde-compat"), rescript(untagged))]
#[rescript(export, export_to = "union_unnamed_serde/")]
enum TestUntagged {
    A,   // serde_json -> `null`
    B(), // serde_json -> `[]`
    C(
        #[cfg_attr(feature = "serde-compat", serde(skip))]
        #[cfg_attr(not(feature = "serde-compat"), rescript(skip))]
        i32,
    ), // serde_json -> `null`
}

#[derive(TS)]
#[cfg_attr(feature = "serde-compat", derive(Deserialize))]
#[rescript(export, export_to = "union_unnamed_serde/")]
#[cfg_attr(feature = "serde-compat", serde(tag = "type", content = "value"))]
#[cfg_attr(
    not(feature = "serde-compat"),
    rescript(tag = "type", content = "value")
)]
enum TestExternally {
    A,   // serde_json -> `"A"`
    B(), // serde_json -> `{"B":[]}`
    C(
        #[cfg_attr(feature = "serde-compat", serde(skip))]
        #[cfg_attr(not(feature = "serde-compat"), rescript(skip))]
        i32,
    ), // serde_json -> `"C"`
}

#[derive(TS)]
#[cfg_attr(feature = "serde-compat", derive(Deserialize))]
#[cfg_attr(feature = "serde-compat", serde(tag = "type", content = "content"))]
#[cfg_attr(
    not(feature = "serde-compat"),
    rescript(tag = "type", content = "content")
)]
#[rescript(export, export_to = "union_unnamed_serde/")]
enum TestAdjacently {
    A,   // serde_json -> `{"type":"A"}`
    B(), // serde_json -> `{"type":"B","content":[]}`
    C(
        #[cfg_attr(feature = "serde-compat", serde(skip))]
        #[cfg_attr(not(feature = "serde-compat"), rescript(skip))]
        i32,
    ), // serde_json -> `{"type":"C"}`
}

#[derive(TS)]
#[cfg_attr(feature = "serde-compat", derive(Deserialize))]
#[cfg_attr(feature = "serde-compat", serde(tag = "type"))]
#[cfg_attr(not(feature = "serde-compat"), rescript(tag = "type"))]
#[rescript(export, export_to = "union_unnamed_serde/")]
enum TestInternally {
    A, // serde_json -> `{"type":"A"}`
    B, // serde_json -> `{"type":"B"}`
    C(
        #[cfg_attr(feature = "serde-compat", serde(skip))]
        #[cfg_attr(not(feature = "serde-compat"), rescript(skip))]
        i32,
    ), // serde_json -> `{"type":"C"}`
}

#[test]
fn test() {
    let cfg = Config::from_env();
    assert_eq!(
        TestUntagged::decl(&cfg),
        "type testUntagged = \n  | A \n  | B(array<never>)\n  | C(unit)"
    );

    assert_eq!(
        TestExternally::decl(&cfg),
        "@tag(\"type\")\ntype testExternally = \n  | A\n  | B({ value: array<never> })\n  | C"
    );

    assert_eq!(
        TestAdjacently::decl(&cfg),
        "@tag(\"type\")\ntype testAdjacently = \n  | A\n  | B({ content: array<never> })\n  | C"
    );

    assert_eq!(
        TestInternally::decl(&cfg),
        "@tag(\"type\")\ntype testInternally = \n  | A\n  | B\n  | C"
    );
}
