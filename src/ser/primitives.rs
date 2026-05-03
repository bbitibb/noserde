use crate::ser::to_json::ToJson;
use crate::ser::writer::write_json_string;

impl ToJson for bool {
    fn write_json(&self, out: &mut String) {
        out.push_str(if *self { "true" } else { "false" });
    }
}

impl ToJson for String {
    fn write_json(&self, out: &mut String) {
        write_json_string(out, self);
    }
}

impl ToJson for &str {
    fn write_json(&self, out: &mut String) {
        write_json_string(out, self);
    }
}

macro_rules! impl_number_to_json {
    ($($ty:ty),* $(,)?) => {
        $(
            impl ToJson for $ty {
                fn write_json(&self, out: &mut String) {
                    out.push_str(&self.to_string());
                }
            }
        )*
    };
}

impl_number_to_json!(
    i8, i16, i32, i64, i128, isize, u8, u16, u32, u64, u128, usize, f32, f64,
);
