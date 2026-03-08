#![allow(dead_code)]

use rescript_rs::{Config, TS};

trait Driver {
    type Info;
}

struct TsDriver;

#[derive(TS)]
struct TsInfo;

impl Driver for TsDriver {
    type Info = TsInfo;
}

#[derive(TS)]
#[rescript(export, export_to = "bound/")]
#[rescript(concrete(D = TsDriver))]
struct Inner<D: Driver> {
    info: D::Info,
}

#[derive(TS)]
#[rescript(export, export_to = "bound/")]
#[rescript(concrete(D = TsDriver), bound = "D::Info: TS")]
struct Outer<D: Driver> {
    inner: Inner<D>,
}

#[test]
fn test_bound() {
    let cfg = Config::from_env();
    assert_eq!(
        Outer::<TsDriver>::decl(&cfg),
        "type outer = { inner: Inner, }"
    );
    assert_eq!(
        Inner::<TsDriver>::decl(&cfg),
        "type inner = { info: TsInfo, }"
    );
}
