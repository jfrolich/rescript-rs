#![allow(dead_code)]

use rescript_rs::{Config, TS};

trait Bar {
    type Baz;
}

impl Bar for String {
    type Baz = i32;
}

#[derive(TS)]
#[rescript(export)]
struct Foo {
    #[rescript(optional, as = "Option<_>")]
    my_optional_bool: bool,

    #[rescript(as = "<_ as Bar>::Baz")]
    q_self: String,
}

#[test]
fn test() {
    let cfg = Config::from_env();
    assert_eq!(
        Foo::inline(&cfg),
        "{ my_optional_bool?: bool, q_self: int, }"
    );
}
