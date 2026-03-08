use rescript_rs::{Config, TS};

#[derive(TS)]
#[rescript(export, export_to = "unit/")]
struct Unit;

#[derive(TS)]
#[rescript(export, export_to = "unit/")]
struct Unit2 {}

#[derive(TS)]
#[rescript(export, export_to = "unit/")]
struct Unit3();

#[derive(TS)]
#[rescript(export, export_to = "unit/")]
struct Unit4(());

#[test]
fn test() {
    let cfg = Config::from_env();
    assert_eq!("type unit = unit", Unit::decl(&cfg));
    assert_eq!("type unit2 = {  }", Unit2::decl(&cfg));
    assert_eq!("type unit3 = array<never>", Unit3::decl(&cfg));
    assert_eq!("type unit4 = unit", Unit4::decl(&cfg));
}
