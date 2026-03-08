use rescript_rs::{Config, TS};

#[derive(TS)]
#[rescript(export, export_to = "repr_enum/", repr(enum))]
enum Foo {
    A = 1,
    B = 2,
}

#[derive(TS)]
#[rescript(export, export_to = "repr_enum/", repr(enum))]
enum Bar {
    A = 1,
    B,
}

#[derive(TS)]
#[rescript(export, export_to = "repr_enum/", repr(enum))]
enum Baz {
    A,
    B,
}

#[derive(TS)]
#[rescript(export, export_to = "repr_enum/", repr(enum = name))]
enum Biz {
    A = 0,
    B,
}

#[derive(TS)]
#[rescript(export, export_to = "repr_enum/", rename_all = "snake_case", repr(enum = name))]
enum SnakeCase {
    EnumVariantFoo,
    EnumVariantBar,
}

#[derive(TS)]
#[rescript(export, export_to = "repr_enum/", rename_all = "camelCase", repr(enum = name))]
enum CamelCase {
    EnumVariantFoo,
    EnumVariantBar,
}

#[derive(TS)]
#[rescript(export, export_to = "repr_enum/", rename_all = "kebab-case", repr(enum = name))]
enum KebabCase {
    EnumVariantFoo,
    EnumVariantBar,
}

#[test]
fn repr_enum_decl() {
    let cfg = Config::from_env();
    assert_eq!(
        Foo::decl(&cfg),
        "type foo = | @as(1) A | @as(2) B"
    );
    assert_eq!(
        Bar::decl(&cfg),
        "type bar = | @as(1) A | B"
    );
    assert_eq!(
        Baz::decl(&cfg),
        "type baz = | A | B"
    );
    assert_eq!(
        Biz::decl(&cfg),
        "type biz = | A | B"
    );
    assert_eq!(
        SnakeCase::decl(&cfg),
        r#"type snakecase = | @as("enum_variant_foo") EnumVariantFoo | @as("enum_variant_bar") EnumVariantBar"#
    );
    assert_eq!(
        CamelCase::decl(&cfg),
        r#"type camelcase = | @as("enumVariantFoo") EnumVariantFoo | @as("enumVariantBar") EnumVariantBar"#
    );
    assert_eq!(
        KebabCase::decl(&cfg),
        r#"type kebabcase = | @as("enum-variant-foo") EnumVariantFoo | @as("enum-variant-bar") EnumVariantBar"#
    );
}

#[test]
fn repr_enum_inline() {
    let cfg = Config::from_env();
    assert_eq!(Foo::inline(&cfg), "| @as(1) A | @as(2) B");
    assert_eq!(Bar::inline(&cfg), "| @as(1) A | B");
    assert_eq!(Baz::inline(&cfg), "| A | B");
    assert_eq!(Biz::inline(&cfg), "| A | B");
    assert_eq!(
        SnakeCase::inline(&cfg),
        r#"| @as("enum_variant_foo") EnumVariantFoo | @as("enum_variant_bar") EnumVariantBar"#
    );
}
