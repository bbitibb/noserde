use noserde::ToJson;

#[test]
fn serializes_numbers() {
    assert_eq!(42_i32.to_json(), "42");
    assert_eq!((-5_i64).to_json(), "-5");
    assert_eq!(10_usize.to_json(), "10");
    assert_eq!(3.5_f64.to_json(), "3.5");
}