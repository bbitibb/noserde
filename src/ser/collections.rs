use crate::ser::to_json::ToJson;

impl<T: ToJson> ToJson for Option<T> {
    fn write_json(&self, out: &mut String) {
        match self {
            Some(value) => value.write_json(out),
            None => out.push_str("null"),
        }
    }
}

impl<T: ToJson> ToJson for Vec<T> {
    fn write_json(&self, out: &mut String) {
        out.push('[');

        for (i, item) in self.iter().enumerate() {
            if i > 0 {
                out.push(',');
            }

            item.write_json(out);
        }

        out.push(']');
    }
}
