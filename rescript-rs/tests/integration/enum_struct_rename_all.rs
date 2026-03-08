#[cfg(feature = "serde-compat")]
use serde::Serialize;
use rescript_rs::{Config, TS};

#[derive(TS)]
#[rescript(export, export_to = "enum_struct_rename_all/")]
#[cfg_attr(feature = "serde-compat", derive(Serialize))]
#[cfg_attr(feature = "serde-compat", serde(tag = "type", rename_all = "camelCase"))]
#[cfg_attr(not(feature = "serde-compat"), ts(tag = "type", rename_all = "camelCase"))]
pub enum TaskStatus {
    #[cfg_attr(feature = "serde-compat", serde(rename_all = "camelCase"))]
    #[cfg_attr(not(feature = "serde-compat"), ts(rename_all = "camelCase"))]
    Running { started_time: String },

    #[cfg_attr(feature = "serde-compat", serde(rename_all = "camelCase"))]
    #[cfg_attr(not(feature = "serde-compat"), ts(rename_all = "camelCase"))]
    Terminated {
        status: i32,
        stdout: String,
        stderr: String,
    },
}

#[test]
pub fn enum_struct_rename_all() {
    let cfg = Config::from_env();
    assert_eq!(
        TaskStatus::inline(&cfg),
        r#"| @as("running") Running({ startedTime: string, }) | @as("terminated") Terminated({ status: int, stdout: string, stderr: string, })"#
    )
}

#[derive(TS, Clone)]
#[rescript(export, export_to = "enum_struct_rename_all/")]
#[cfg_attr(feature = "serde-compat", derive(Serialize))]
#[cfg_attr(feature = "serde-compat", serde(tag = "type", content = "value", rename_all_fields = "kebab-case"))]
#[cfg_attr(not(feature = "serde-compat"), ts(tag = "type", content = "value", rename_all_fields = "kebab-case"))]
pub enum TaskStatus2 {
    Running {
        started_time: String,
    },

    Terminated {
        status: i32,
        stdout: String,
        stderr: String,
    },

    A(i32),
    B(i32, i32),
    C,
}

#[test]
pub fn enum_struct_rename_all_fields() {
    let cfg = Config::from_env();
    assert_eq!(
        TaskStatus2::inline(&cfg),
        r#"| Running({ value: { "started-time": string, } }) | Terminated({ value: { status: int, stdout: string, stderr: string, } }) | A({ value: int }) | B({ value: (int, int) }) | C"#
    )
}
