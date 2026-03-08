#![allow(dead_code)]

use rescript_rs::{Config, TS};

#[derive(TS)]
#[rescript(export, export_to = "arrays/")]
struct Interface {
    a: [i32; 4],
}

#[test]
fn free() {
    let cfg = Config::from_env();
    assert_eq!(<[String; 4]>::inline(&cfg), "(string, string, string, string)")
}

#[test]
fn interface() {
    let cfg = Config::from_env();
    assert_eq!(
        Interface::inline(&cfg),
        "{ a: (int, int, int, int), }"
    )
}

#[test]
fn newtype() {
    #[derive(TS)]
    struct Newtype(#[allow(dead_code)] [i32; 4]);

    let cfg = Config::from_env();
    assert_eq!(Newtype::inline(&cfg), "(int, int, int, int)")
}
