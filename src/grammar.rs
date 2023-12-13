pub struct Decorate;
impl Decorate {
    pub fn output(name: String, _type: Option<String>, contents: Option<String>) -> String {
        format!(
            "{}{} ({})",
            name,
            match _type {
                Some(_type) => format!("::{}", _type),
                None => "".to_string(),
            },
            match contents {
                Some(contents) => format!("\n  {}\n", contents),
                None => "".to_string(),
            }
        )
    }
}
