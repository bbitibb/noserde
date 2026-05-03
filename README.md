# noserde

Small JSON serialization/deserialization for simple Rust structs.

No dependencies. No proc macros. Not Serde.

## Existing Struct

```rust
use noserde::{FromJson, ToJson, impl_json};

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

let user = User {
    id: 1,
    name: "Bob".to_string(),
    active: true,
};

let json = user.to_json();
assert_eq!(json, r#"{"id":1,"name":"Bob","active":true}"#);

let parsed = User::from_json(&json).unwrap();
assert_eq!(parsed.name, "Bob");
```

## Define Struct With JSON Support

```rust
use noserde::{FromJson, ToJson, json_struct};

json_struct! {
    #[derive(Debug, PartialEq)]
    pub struct Product {
        pub id: i32,
        pub name: String,
        pub in_stock: bool,
    }
}

let product = Product::from_json(
    r#"{"id":10,"name":"Keyboard","in_stock":false}"#
)
.unwrap();

assert_eq!(product.to_json(), r#"{"id":10,"name":"Keyboard","in_stock":false}"#);
```

## Vec and Option

```rust
use noserde::{FromJson, impl_json};

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

let article = Article::from_json(
    r#"{"title":"Rust JSON","tags":["rust","json"],"views":null}"#
)
.unwrap();

assert_eq!(article.tags, vec!["rust".to_string(), "json".to_string()]);
assert_eq!(article.views, None);
```

## Manual Serialization

```rust
use noserde::{ToJson, write_json_object};

struct Address {
    city: String,
    zip: i32,
}

impl ToJson for Address {
    fn write_json(&self, out: &mut String) {
        write_json_object(out, |obj| {
            obj.field("city", &self.city);
            obj.field("zip", &self.zip);
        });
    }
}

let address = Address {
    city: "Budapest".to_string(),
    zip: 1111,
};

assert_eq!(address.to_json(), r#"{"city":"Budapest","zip":1111}"#);
```

## Supported Basics

`bool`, integers, floats, `String`, `&str`, `Option<T>`, `Vec<T>`, and nested structs that implement the traits.
