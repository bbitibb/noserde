use noserde::{impl_json, json_struct, FromJson, ToJson};

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

#[test]
fn deserializes_basic_struct() {
    let user = User::from_json(r#"{"id":1,"name":"Bob","active":true}"#).unwrap();

    assert_eq!(user.id, 1);
    assert_eq!(user.name, "Bob");
    assert_eq!(user.active, true);
}

#[test]
fn roundtrips_basic_struct() {
    let user = User {
        id: 1,
        name: "Bob".to_string(),
        active: true,
    };

    let json = user.to_json();

    let parsed = User::from_json(&json).unwrap();

    assert_eq!(parsed.id, 1);
    assert_eq!(parsed.name, "Bob");
    assert_eq!(parsed.active, true);
}

json_struct! {
    struct Product {
        id: i32,
        name: String,
        in_stock: bool,
    }
}

#[test]
fn json_struct_deserializes() {
    let product = Product::from_json(
        r#"{"id":10,"name":"Keyboard","in_stock":false}"#
    )
    .unwrap();

    assert_eq!(product.id, 10);
    assert_eq!(product.name, "Keyboard");
    assert_eq!(product.in_stock, false);
}

struct Article {
    title: String,
    tags: Vec<String>,
    views: Option<i32>,
}

impl_json! {
    Article {
        title: String,
        tags: Vec<String>,
        views: Option<i32>,
    }
}

#[test]
fn deserializes_vec_and_option() {
    let article = Article::from_json(
        r#"{"title":"Rust JSON","tags":["rust","json"],"views":null}"#
    )
    .unwrap();

    assert_eq!(article.title, "Rust JSON");
    assert_eq!(article.tags, vec!["rust".to_string(), "json".to_string()]);
    assert_eq!(article.views, None);
}