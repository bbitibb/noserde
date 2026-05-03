use crate::ser::to_json::ToJson;

pub struct JsonObject<'a> {
    out: &'a mut String,
    first: bool,
}

pub fn write_json_string(out: &mut String, value: &str) {
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

pub fn write_json_object<F>(out: &mut String, build: F)
where
    F: FnOnce(&mut JsonObject<'_>),
{
    out.push('{');

    let mut obj = JsonObject { out, first: true };
    build(&mut obj);

    obj.out.push('}');
}

impl JsonObject<'_> {
    pub fn field<T: ToJson + ?Sized>(&mut self, name: &str, value: &T) {
        if !self.first {
            self.out.push(',');
        }

        self.first = false;

        write_json_string(self.out, name);
        self.out.push(':');
        value.write_json(self.out);
    }
}
