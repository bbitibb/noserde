use noserde::{ToJson, write_json_object, write_json_string};

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

#[test]
fn write_json_string_escapes_special_chars() {
    let mut out = String::new();

    write_json_string(&mut out, "quote \" slash \\ line\n\u{01}");

    assert_eq!(out, r#""quote \" slash \\ line\n\u0001""#);
}

#[test]
fn write_json_object_writes_empty_object() {
    let mut out = String::new();

    write_json_object(&mut out, |_| {});

    assert_eq!(out, "{}");
}

#[test]
fn write_json_object_writes_fields() {
    let mut out = String::new();

    write_json_object(&mut out, |obj| {
        obj.field("name", &"Bob");
        obj.field("active", &true);
    });

    assert_eq!(out, r#"{"name":"Bob","active":true}"#);
}

#[test]
fn manual_to_json_can_use_object_writer() {
    let address = Address {
        city: "Budapest".to_string(),
        zip: 1111,
    };

    assert_eq!(address.to_json(), r#"{"city":"Budapest","zip":1111}"#);
}
