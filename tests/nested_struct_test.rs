use noserde::{impl_to_json, ToJson};

struct Address {
    city: String,
    zip: i32,
}

impl_to_json!(Address { city, zip });

struct Person {
    name: String,
    address: Address,
}

impl_to_json!(Person { name, address });

#[test]
fn serializes_nested_struct() {
    let person = Person {
        name: "Bob".to_string(),
        address: Address {
            city: "The Moon".to_string(),
            zip: 1111,
        },
    };

    assert_eq!(
        person.to_json(),
        r#"{"name":"Bob","address":{"city":"The Moon","zip":1111}}"#
    );
}