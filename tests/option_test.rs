use noserde::{impl_to_json, ToJson};

struct UserWithAge {
    name: String,
    age: Option<i32>,
}

impl_to_json!(UserWithAge { name, age });

#[test]
fn serializes_some_option() {
    let user = UserWithAge {
        name: "Bob".to_string(),
        age: Some(20),
    };

    assert_eq!(
        user.to_json(),
        r#"{"name":"Bob","age":20}"#
    );
}

#[test]
fn serializes_none_option() {
    let user = UserWithAge {
        name: "Bob".to_string(),
        age: None,
    };

    assert_eq!(
        user.to_json(),
        r#"{"name":"Bob","age":null}"#
    );
}