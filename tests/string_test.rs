use noserde::ToJson;

#[test]
fn string_escapes_quotes() {
    let value = String::from("hello \"world\"");

    assert_eq!(
        value.to_json(),
        r#""hello \"world\"""#
    );
}

#[test]
fn string_escapes_backslash() {
    let value = String::from(r#"C:\Users\Bob"#);

    assert_eq!(
        value.to_json(),
        r#""C:\\Users\\Bob""#
    );
}

#[test]
fn string_escapes_newline() {
    let value = String::from("hello\nworld");

    assert_eq!(
        value.to_json(),
        r#""hello\nworld""#
    );
}