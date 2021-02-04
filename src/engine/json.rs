use crate::engine::Engine;
use crate::value::number::Number;
use crate::value::pod::Pod;
use json::JsonValue;

#[derive(PartialEq, Debug)]
pub struct JSON {}

impl Engine for JSON {
    fn new() -> Self {
        return JSON {};
    }

    fn parse(&self, content: &str) -> Pod {
        match json::parse(content) {
            Ok(data) => data.into(),
            Err(_) => Pod::Null,
        }
    }
}

impl Into<Pod> for JsonValue {
    fn into(self) -> Pod {
        match self {
            JsonValue::Null => Pod::Null,
            JsonValue::Short(val) => Pod::String(val.as_str().to_string()),
            JsonValue::String(val) => Pod::String(val),
            JsonValue::Number(_) => match self.as_f64() {
                Some(number) => Pod::Number(Number::from(number)),
                None => match self.as_isize() {
                    Some(number) => Pod::Number(Number::from(number)),
                    None => Pod::Number(Number::from(0)),
                },
            },
            JsonValue::Boolean(val) => Pod::Boolean(val),
            JsonValue::Array(val) => {
                let mut pod = Pod::new_array();
                for item in val.into_iter() {
                    match pod.push(item) {
                        Ok(_) => {}
                        Err(_) => {}
                    };
                }
                pod
            }
            JsonValue::Object(val) => {
                let mut pod = Pod::new_hash();
                for (key, val) in val.iter() {
                    match pod.insert(key.to_string(), (*val).clone()) {
                        Ok(_) => {}
                        Err(_) => {}
                    }
                }
                pod
            }
        }
    }
}

// todo: add more tests
#[test]
fn test_matter() {
    use crate::entity::ParsedEntity;
    use crate::matter::Matter;
    let matter: Matter<JSON> = Matter::new();
    let input = r#"---
{"title": "Home"}
---
Some excerpt
---
Other stuff"#;

    let mut data = Pod::new_hash();
    match data.insert("title".to_string(), Pod::String("Home".to_string())) {
        Ok(_) => {}
        Err(err) => panic!(err),
    };
    let parsed_entity = ParsedEntity {
        data,
        content: "Some excerpt\n---\nOther stuff",
        excerpt: "Some excerpt",
        orig: input,
    };

    assert_eq!(matter.matter(input), parsed_entity);
}
