#![allow(dead_code, clippy::disallowed_names)]

use std::{collections::BTreeSet, rc::Rc};

use chrono::NaiveDateTime;
use rescript_rs::TS;
use serde::Serialize;
use uuid::Uuid;

#[derive(Serialize, TS)]
#[rescript(rename_all = "lowercase")]
#[rescript(export, export_to = "UserRole.res")]
enum Role {
    User,
    #[rescript(rename = "administrator")]
    Admin,
}

#[derive(Serialize, TS)]
#[serde(rename_all = "UPPERCASE")]
#[rescript(export)]
enum Gender {
    Male,
    Female,
    Other,
}

#[derive(Serialize, TS)]
#[rescript(export)]
struct User {
    user_id: i32,
    first_name: String,
    last_name: String,
    role: Role,
    family: Vec<User>,
    #[rescript(inline)]
    gender: Gender,
    token: Uuid,
    #[rescript(type = "string")]
    created_at: NaiveDateTime,
}

#[derive(Serialize, TS)]
#[serde(tag = "type", rename_all = "snake_case")]
#[rescript(export)]
enum Vehicle {
    Bicycle { color: String },
    Car { brand: String, color: String },
}

#[derive(Serialize, TS)]
#[rescript(export)]
struct Point<T>
where
    T: TS,
{
    time: u64,
    value: T,
}

#[derive(Serialize, TS)]
#[serde(default)]
#[rescript(export)]
struct Series {
    points: Vec<Point<u64>>,
}

#[derive(Serialize, TS)]
#[serde(tag = "kind", content = "d")]
#[rescript(export)]
enum SimpleEnum {
    A,
    B,
}

#[derive(Serialize, TS)]
#[serde(tag = "kind", content = "data")]
#[rescript(export)]
enum ComplexEnum {
    A,
    B { foo: String, bar: f64 },
    W(SimpleEnum),
    F { nested: SimpleEnum },
    V(Vec<Series>),
    U(Box<User>),
}

#[derive(Serialize, TS)]
#[serde(tag = "kind")]
#[rescript(export)]
enum InlineComplexEnum {
    A,
    B { foo: String, bar: f64 },
    W(SimpleEnum),
    F { nested: SimpleEnum },
    V(Vec<Series>),
    U(Box<User>),
}

#[derive(Serialize, TS)]
#[serde(rename_all = "camelCase")]
#[rescript(export)]
struct ComplexStruct {
    #[serde(default)]
    pub string_tree: Option<Rc<BTreeSet<String>>>,
}
