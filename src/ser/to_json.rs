pub trait ToJson {
    fn write_json(&self, out: &mut String);

    fn to_json(&self) -> String {
        let mut out = String::new();
        self.write_json(&mut out);
        out
    }

    fn save_json<P: AsRef<std::path::Path>>(&self, path: P) -> std::io::Result<()> {
        std::fs::write(path, self.to_json())
    }
}