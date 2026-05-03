use noserde::{FromJson, JsonError, impl_json};

#[derive(Debug)]
struct User {
    id: i32,
    name: String,
    active: bool,
}

impl_json! {
    User {
        id: i32,
        name: String,
        active: bool,
    }
}

#[derive(Debug)]
struct Address {
    city: String,
    zip: i32,
}

impl_json! {
    Address {
        city: String,
        zip: i32,
    }
}

#[derive(Debug)]
struct Person {
    name: String,
    address: Address,
}

impl_json! {
    Person {
        name: String,
        address: Address,
    }
}

#[test]
fn deserializes_struct_with_whitespace_and_any_field_order() {
    let user = User::from_json(
        r#"{
            "active" : false,
            "name" : "Ada",
            "id" : 42
        }"#,
    )
    .unwrap();

    assert_eq!(user.id, 42);
    assert_eq!(user.name, "Ada");
    assert_eq!(user.active, false);
}

#[test]
fn missing_required_field_returns_error() {
    let err = User::from_json(r#"{"id":1,"name":"Bob"}"#).unwrap_err();

    assert_eq!(err, JsonError::MissingField("active"));
}

#[test]
fn unknown_field_returns_error() {
    let err = User::from_json(r#"{"id":1,"name":"Bob","active":true,"role":"admin"}"#).unwrap_err();

    assert_eq!(err, JsonError::UnknownField("role".to_string()));
}

#[test]
fn trailing_characters_return_error() {
    let err = User::from_json(r#"{"id":1,"name":"Bob","active":true} null"#).unwrap_err();

    assert_eq!(err, JsonError::TrailingCharacters);
}

#[test]
fn deserializes_nested_custom_struct() {
    let person =
        Person::from_json(r#"{"name":"Bob","address":{"city":"Budapest","zip":1111}}"#).unwrap();

    assert_eq!(person.name, "Bob");
    assert_eq!(person.address.city, "Budapest");
    assert_eq!(person.address.zip, 1111);
}

#[test]
fn deserializes_string_escapes_and_basic_unicode_escape() {
    let value =
        String::from_json(r#""line\n tab\t quote\" slash\\ solidus\/ heart\u2764""#).unwrap();

    assert_eq!(value, "line\n tab\t quote\" slash\\ solidus/ heart\u{2764}");
}

#[test]
fn invalid_string_escape_returns_error() {
    let err = String::from_json(r#""bad\x""#).unwrap_err();

    assert_eq!(err, JsonError::InvalidEscape('x'));
}

#[test]
fn invalid_unicode_escape_returns_error() {
    let err = String::from_json(r#""bad\u12xz""#).unwrap_err();

    assert_eq!(err, JsonError::InvalidUnicodeEscape);
}

#[test]
fn deserializes_empty_and_populated_vecs() {
    let empty = Vec::<String>::from_json("[]").unwrap();
    let values = Vec::<i32>::from_json("[1, 2, -3]").unwrap();

    assert_eq!(empty, Vec::<String>::new());
    assert_eq!(values, vec![1, 2, -3]);
}

#[test]
fn deserializes_option_null_and_some_value() {
    let none = Option::<i32>::from_json(" null ").unwrap();
    let some = Option::<i32>::from_json("5").unwrap();

    assert_eq!(none, None);
    assert_eq!(some, Some(5));
}

#[test]
fn invalid_numbers_return_error() {
    let float_for_int = i32::from_json("1.2").unwrap_err();
    let overflowing_u8 = u8::from_json("999").unwrap_err();

    assert_eq!(float_for_int, JsonError::InvalidNumber);
    assert_eq!(overflowing_u8, JsonError::InvalidNumber);
}
