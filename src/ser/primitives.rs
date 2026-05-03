use crate::ser::to_json::ToJson;

impl ToJson for bool {
    fn write_json(&self, out: &mut String) {
        out.push_str(if *self { "true" } else { "false" });
    }
}

impl ToJson for String {
    fn write_json(&self, out: &mut String) {
        write_json_string(self, out);
    }
}

impl ToJson for &str {
    fn write_json(&self, out: &mut String) {
        write_json_string(self, out);
    }
}

fn write_json_string(value: &str, out: &mut String) {
    out.push('"');

    for ch in value.chars() {
        match ch {
            '"' => out.push_str("\\\""),
            '\\' => out.push_str("\\\\"),
            '\n' => out.push_str("\\n"),
            '\r' => out.push_str("\\r"),
            '\t' => out.push_str("\\t"),
            '\u{08}' => out.push_str("\\b"),
            '\u{0C}' => out.push_str("\\f"),
            ch if ch.is_control() => {
                out.push_str(&format!("\\u{:04x}", ch as u32));
            }
            ch => out.push(ch),
        }
    }

    out.push('"');
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
    i8, i16, i32, i64, i128, isize,
    u8, u16, u32, u64, u128, usize,
    f32, f64,
);
