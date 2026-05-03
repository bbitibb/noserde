use noserde::{impl_to_json, json_struct, ToJson};

struct User {
    id: i32,
    name: String,
    active: bool,
}

impl_to_json!(User { id, name, active });

#[test]
fn impl_to_json_serializes_struct() {
    let user = User {
        id: 1,
        name: "Bob".to_string(),
        active: true,
    };

    assert_eq!(
        user.to_json(),
        r#"{"id":1,"name":"Bob","active":true}"#
    );
}

json_struct! {
    struct Product {
        id: i32,
        name: String,
        in_stock: bool,
    }
}

#[test]
fn json_struct_serializes_struct() {
    let product = Product {
        id: 10,
        name: "Keyboard".to_string(),
        in_stock: false,
    };

    assert_eq!(
        product.to_json(),
        r#"{"id":10,"name":"Keyboard","in_stock":false}"#
    );
}

struct Book {
    id: i32,
    title: String,
    author: String,
    n_pages: i32,
}

impl_to_json!( Book { id, title, author, n_pages } );

#[test]
fn impl_to_json_book_struct(){
    let book = Book {
        id: 69,
        title: String::from("Csipike"),
        author: String::from("Fodor Sandor"),
        n_pages: 42,
    };
    
    assert_eq!(book.to_json(), r#"{"id":69,"title":"Csipike","author":"Fodor Sandor","n_pages":42}"#);
}