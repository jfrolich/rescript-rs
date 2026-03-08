#![allow(dead_code)]

#[cfg(feature = "serde-compat")]
use serde::Deserialize;
use rescript_rs::{Config, TS};

#[derive(TS)]
#[cfg_attr(feature = "serde-compat", derive(Deserialize))]
#[cfg_attr(feature = "serde-compat", serde(untagged))]
#[cfg_attr(not(feature = "serde-compat"), rescript(untagged))]
#[rescript(export, export_to = "union_named_serde/")]
enum TestUntagged {
    A,   // serde_json -> `null`
    B(), // serde_json -> `[]`
    C {
        #[cfg_attr(feature = "serde-compat", serde(skip))]
        #[cfg_attr(not(feature = "serde-compat"), rescript(skip))]
        val: i32,
    }, // serde_json -> `{}`
}

#[derive(TS)]
#[cfg_attr(feature = "serde-compat", derive(Deserialize))]
#[rescript(export, export_to = "union_named_serde/")]
#[cfg_attr(feature = "serde-compat", serde(tag = "type", content = "value"))]
#[cfg_attr(not(feature = "serde-compat"), rescript(tag = "type", content = "value"))]
enum TestExternally {
    A,   // serde_json -> `"A"`
    B(), // serde_json -> `{"B":[]}`
    C {
        #[cfg_attr(feature = "serde-compat", serde(skip))]
        #[cfg_attr(not(feature = "serde-compat"), rescript(skip))]
        val: i32,
    }, // serde_json -> `{"C":{}}`
}

#[derive(TS)]
#[cfg_attr(feature = "serde-compat", derive(Deserialize))]
#[cfg_attr(feature = "serde-compat", serde(tag = "type", content = "content"))]
#[cfg_attr(not(feature = "serde-compat"), rescript(tag = "type", content = "content"))]
#[rescript(export, export_to = "union_named_serde/")]
enum TestAdjacently {
    A,   // serde_json -> `{"type":"A"}`
    B(), // serde_json -> `{"type":"B","content":[]}`
    C {
        #[cfg_attr(feature = "serde-compat", serde(skip))]
        #[cfg_attr(not(feature = "serde-compat"), rescript(skip))]
        val: i32,
    }, // serde_json -> `{"type":"C","content":{}}`
}

#[derive(TS)]
#[cfg_attr(feature = "serde-compat", derive(Deserialize))]
#[cfg_attr(feature = "serde-compat", serde(tag = "type"))]
#[cfg_attr(not(feature = "serde-compat"), rescript(tag = "type"))]
#[rescript(export, export_to = "union_named_serde/")]
enum TestInternally {
    A, // serde_json -> `{"type":"A"}`
    B, // serde_json -> `{"type":"B"}`
    C {
        #[cfg_attr(feature = "serde-compat", serde(skip))]
        #[cfg_attr(not(feature = "serde-compat"), rescript(skip))]
        val: i32,
    }, // serde_json -> `{"type":"C"}`
}

#[test]
fn test() {
    let cfg = Config::from_env();
    assert_eq!(
        TestUntagged::decl(&cfg),
        "type testuntagged = | A  | B(array<never>) | C({  })"
    );

    assert_eq!(
        TestExternally::decl(&cfg),
        "@tag(\"type\")\ntype testexternally = | A | B({ value: array<never> }) | C({ value: {  } })"
    );

    assert_eq!(
        TestAdjacently::decl(&cfg),
        "@tag(\"type\")\ntype testadjacently = | A | B({ content: array<never> }) | C({ content: {  } })"
    );

    assert_eq!(
        TestInternally::decl(&cfg),
        "@tag(\"type\")\ntype testinternally = | A | B | C({  })"
    );
}
