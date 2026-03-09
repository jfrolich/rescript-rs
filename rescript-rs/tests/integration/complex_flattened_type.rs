use rescript_rs::{Config, TS};

/// Defines the type of input and its intial fields
#[derive(TS)]
#[rescript(tag = "input_type")]
pub enum InputType {
    Text,
    Expression,
    Number {
        min: Option<isize>,
        max: Option<isize>,
    },
    Dropdown {
        options: Vec<(String, String)>,
    },
}

#[derive(TS)]
#[rescript(tag = "type")]
pub enum InputFieldElement {
    Label {
        text: String,
    },
    Input {
        #[rescript(flatten)]
        input: InputType,
        name: Option<String>,
        placeholder: Option<String>,
        default: Option<String>,
    },
}

#[derive(TS)]
#[rescript(export, export_to = "complex_flattened_type/")]
pub struct InputField {
    #[rescript(flatten)]
    r#type: InputFieldElement,
}

#[test]
fn complex_flattened_type() {
    let cfg = Config::from_env();
    assert_eq!(
        InputFieldElement::decl(&cfg),
        "@tag(\"type\")\ntype inputFieldElement = \n  | Label({ text: string, })\n  | Input({ name: option<string>, placeholder: option<string>, default: option<string>, } & (| Text | Expression | Number({ min: option<int>, max: option<int>, }) | Dropdown({ options: array<(string, string)>, })))"
    );
    assert_eq!(
        InputField::decl(&cfg),
        "type inputField = | Label({ text: string, }) | Input({ name: option<string>, placeholder: option<string>, default: option<string>, } & (| Text | Expression | Number({ min: option<int>, max: option<int>, }) | Dropdown({ options: array<(string, string)>, })))"
    )
}
