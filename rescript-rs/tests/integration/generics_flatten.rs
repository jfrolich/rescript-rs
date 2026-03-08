use rescript_rs::Config;
use rescript_rs::TS;

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
    assert_eq!(Item::<()>::decl(&cfg), "type Item<D> = { id: string, } & D;");
    assert_eq!(
        TwoParameters::<(), ()>::decl(&cfg),
        "type TwoParameters<A, B> = { id: string, ab: [A, B], } & A & B;"
    );
    assert_eq!(
        Enum::<(), ()>::decl(&cfg),
        "type Enum<A, B> = { \"A\": A } | { \"B\": B } | { \"AB\": [A, B] };"
    );
}
