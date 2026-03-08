#![allow(unused)]

mod issue_261 {
    use rescript_rs::{Config, TS};

    trait Driver {
        type Info;
    }

    struct TsDriver;
    impl Driver for TsDriver {
        type Info = String;
    }

    #[derive(TS)]
    #[rescript(export, export_to = "concrete_generic/issue_261/")]
    struct OtherInfo {
        x: i32,
    }

    #[derive(TS)]
    #[rescript(export, export_to = "concrete_generic/issue_261/")]
    struct OtherDriver;
    impl Driver for OtherDriver {
        type Info = OtherInfo;
    }

    #[derive(TS)]
    #[rescript(export, export_to = "concrete_generic/issue_261/", concrete(T = TsDriver))]
    struct Consumer1<T: Driver> {
        info: T::Info,
    }

    #[derive(TS)]
    #[rescript(export, export_to = "concrete_generic/issue_261/", concrete(T = OtherDriver))]
    struct Consumer2<T: Driver> {
        info: T::Info,
        driver: T,
    }

    #[test]
    fn concrete_generic_param() {
        let cfg = Config::from_env();
        assert_eq!(
            Consumer1::<TsDriver>::decl(&cfg),
            "type consumer1 = { info: string, }"
        );
        assert_eq!(
            Consumer1::<TsDriver>::decl(&cfg),
            Consumer1::<OtherDriver>::decl(&cfg)
        );

        assert_eq!(
            Consumer2::<OtherDriver>::decl_concrete(&cfg),
            "type consumer2 = { info: OtherInfo, driver: OtherDriver, }"
        );
    }
}

mod simple {
    use rescript_rs::{Config, TS};

    #[derive(TS)]
    #[rescript(export, export_to = "concrete_generic/simple/")]
    #[rescript(concrete(T = i32))]
    struct Simple<T> {
        t: T,
    }

    #[derive(TS)]
    #[rescript(export, export_to = "concrete_generic/simple/")]
    struct Tuple<T> {
        f: Option<T>,
    }

    #[derive(TS)]
    #[rescript(export, export_to = "concrete_generic/simple/")]
    #[rescript(concrete(T = i32))]
    struct WithOption<T> {
        opt: Option<T>,
    }

    #[test]
    fn simple() {
        let cfg = Config::from_env();
        assert_eq!(Simple::<String>::decl(&cfg), "type simple = { t: int, }");
        assert_eq!(
            WithOption::<String>::decl(&cfg),
            "type withoption = { opt: option<int>, }"
        );
    }
}
