#[macro_export]
macro_rules! get_model_name {
    () => {{
        let mut module = module_path!().rsplit_once("::").unwrap().1.to_owned();
        module[0..1].make_ascii_uppercase();
        format!("{module}Model")
    }};
}

mod entities {
    mod users {
        use rescript_rs::TS;

        #[derive(TS)]
        #[rescript(export)]
        #[rescript(export_to = "issue_397/")]
        #[rescript(rename = {
            let mut module = module_path!().rsplit_once("::").unwrap().1.to_owned();
            module[0..1].make_ascii_uppercase();
            format!("{module}Model")
        })]
        struct Model;
    }

    mod posts {
        use rescript_rs::TS;

        #[derive(TS)]
        #[rescript(export)]
        #[rescript(export_to = "issue_397/")]
        #[rescript(rename = get_model_name!())]
        struct Model;
    }
}
