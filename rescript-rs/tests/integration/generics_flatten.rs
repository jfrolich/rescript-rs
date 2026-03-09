use rescript_rs::Config;
use rescript_rs::TS;
#[cfg(feature = "serde-compat")]
use serde::Serialize;

// https://github.com/Aleph-Alpha/ts-rs/issues/335
#[derive(TS)]
#[rescript(export, export_to = "generics/flatten/")]
struct Item<D> {
    id: String,
    #[rescript(flatten)]
    inner: D,
}

#[derive(TS)]
#[rescript(export, export_to = "generics/flatten/")]
struct TwoParameters<A, B> {
    id: String,
    #[rescript(flatten)]
    a: A,
    #[rescript(flatten)]
    b: B,
    ab: (A, B),
}

#[derive(TS)]
#[cfg_attr(feature = "serde-compat", derive(Serialize))]
#[cfg_attr(feature = "serde-compat", serde(tag = "type", content = "value"))]
#[cfg_attr(
    not(feature = "serde-compat"),
    rescript(tag = "type", content = "value")
)]
#[rescript(export, export_to = "generics/flatten/")]
enum Enum<A, B> {
    A {
        #[rescript(flatten)]
        a: A,
    },
    B {
        #[rescript(flatten)]
        b: B,
    },
    AB(A, B),
}

#[test]
fn flattened_generic_parameters() {
    use rescript_rs::TS;

    #[derive(TS)]
    struct Inner {
        x: i32,
    }

    let cfg = Config::from_env();
    assert_eq!(Item::<()>::decl(&cfg), "type item<D> = { id: string, } & D");
    assert_eq!(
        TwoParameters::<(), ()>::decl(&cfg),
        "type twoParameters<A, B> = { id: string, ab: (A, B), } & A & B"
    );
    assert_eq!(
        Enum::<(), ()>::decl(&cfg),
        "@tag(\"type\")\ntype enum<A, B> = \n  | A({ value: A })\n  | B({ value: B })\n  | AB({ value: (A, B) })"
    );
}
