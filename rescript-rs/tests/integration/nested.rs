#![allow(dead_code)]

use std::{cell::Cell, rc::Rc, sync::Arc};

use rescript_rs::{Config, TS};

#[derive(TS)]
#[rescript(export, export_to = "nested/")]
struct A {
    x1: Arc<i32>,
    y1: Cell<i32>,
}

#[derive(TS)]
#[rescript(export, export_to = "nested/")]
struct B {
    a1: Box<A>,
    #[rescript(inline)]
    a2: A,
}

#[derive(TS)]
#[rescript(export, export_to = "nested/")]
struct C {
    b1: Rc<B>,
    #[rescript(inline)]
    b2: B,
}

#[test]
fn test_nested() {
    let cfg = Config::from_env();
    assert_eq!(
        C::inline(&cfg),
        "{ b1: B, b2: { a1: A, a2: { x1: int, y1: int, }, }, }"
    );
}
