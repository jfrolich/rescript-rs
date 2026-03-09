#![allow(unused)]

use rescript_rs::{Config, TS};

#[test]
fn test_tuple() {
    type Tuple = (String, i32, (i32, i32));
    let cfg = Config::from_env();
    assert_eq!("(string, int, (int, int))", Tuple::name(&cfg));
}

#[test]
#[should_panic]
fn test_decl() {
    type Tuple = (String, i32, (i32, i32));
    let cfg = Config::from_env();
    let _ = Tuple::decl(&cfg);
}

#[test]
fn test_newtype() {
    #[derive(TS)]
    struct NewType(String);

    let cfg = Config::from_env();
    assert_eq!("type newType = string", NewType::decl(&cfg));
}

#[derive(TS)]
#[rescript(export, export_to = "tuple/", rename_all = "camelCase")] // rename_all should compile, but it's a noop
struct TupleNewType(String, i32, (i32, i32));

#[test]
fn test_tuple_newtype() {
    let cfg = Config::from_env();
    assert_eq!(
        "type tupleNewType = (string, int, (int, int))",
        TupleNewType::decl(&cfg)
    )
}

#[derive(TS)]
#[rescript(export, export_to = "tuple/", rename_all = "kebab-case")]
struct Dep1;

#[derive(TS)]
#[rescript(export, export_to = "tuple/")]
struct Dep2;

#[derive(TS)]
#[rescript(export, export_to = "tuple/")]
struct Dep3;

#[derive(TS)]
#[rescript(export, export_to = "tuple/")]
struct Dep4<T> {
    a: (T, T),
    b: (T, T),
}

#[derive(TS)]
#[rescript(export, export_to = "tuple/")]
struct TupleWithDependencies(Dep1, Dep2, Dep4<Dep3>);

#[test]
fn tuple_with_dependencies() {
    let cfg = Config::from_env();
    assert_eq!(
        "type tupleWithDependencies = (dep1, dep2, dep4<dep3>)",
        TupleWithDependencies::decl(&cfg)
    );
}

#[derive(TS)]
#[rescript(export, export_to = "tuple/")]
struct StructWithTuples {
    a: (Dep1, Dep1),
    b: (Dep2, Dep2),
    c: (Dep4<Dep3>, Dep4<Dep3>),
}

#[test]
fn struct_with_tuples() {
    let cfg = Config::from_env();
    assert_eq!(
        "type structWithTuples = {\n  \
            a: (dep1, dep1),\n  \
            b: (dep2, dep2),\n  \
            c: (dep4<dep3>, dep4<dep3>),\n\
        }",
        StructWithTuples::decl(&cfg)
    );
}
