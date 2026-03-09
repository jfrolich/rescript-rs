#![allow(dead_code)]

use std::{borrow::Cow, rc::Rc, sync::Arc};

use rescript_rs::{Config, TS};

#[derive(TS)]
#[rescript(export, export_to = "unsized/")]
struct S<'a> {
    b: Box<str>,
    c: Cow<'a, str>,
    r: Rc<str>,
    a: Arc<str>,
}

#[test]
fn contains_str() {
    let cfg = Config::from_env();
    assert_eq!(
        S::decl(&cfg),
        "type s = {\n  b: string,\n  c: string,\n  r: string,\n  a: string,\n}"
    )
}
