use crate::engine::Engine;
use crate::entity::ParsedEntity;
use crate::matter::Matter;

#[derive(PartialEq, Debug)]
pub struct JSON {}

impl Engine for JSON {
    type Output = json::JsonValue;

    fn new() -> Self {
        return JSON {};
    }

    fn parse(&self, content: &str) -> Self::Output {
        match json::parse(content) {
            Ok(data) => data,
            Err(_) => self.init_data(),
        }
    }

    fn init_data(&self) -> Self::Output {
        json::parse("{}").unwrap()
    }
}

#[test]
fn test_matter() {
    let matter: Matter<JSON> = Matter::new();
    let input = r#"---
{"title": "Home"}
---
Some excerpt
---
Other stuff"#;

    let data = object! {
        title: "Home"
    };
    let parsed_entity = ParsedEntity {
        data,
        content: "Some excerpt\n---\nOther stuff",
        excerpt: "Some excerpt",
        orig: input,
    };

    assert_eq!(matter.matter(input), parsed_entity);
}
