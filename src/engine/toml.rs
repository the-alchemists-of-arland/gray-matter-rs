use crate::engine::Engine;
use crate::value::pod::Pod;
use toml::Value as TomlValue;

#[derive(PartialEq, Debug)]
pub struct TOML {}

impl Engine for TOML {
    fn new() -> Self {
        return TOML {};
    }

    fn parse(&self, content: &str) -> Pod {
        match toml::from_str::<TomlValue>(content) {
            Ok(value) => value.into(),
            Err(..) => Pod::Null,
        }
    }
}

impl Into<Pod> for TomlValue {
    fn into(self) -> Pod {
        match self {
            TomlValue::String(val) => Pod::String(val),
            TomlValue::Integer(val) => Pod::Integer(val),
            TomlValue::Float(val) => Pod::Float(val),
            TomlValue::Boolean(val) => Pod::Boolean(val),
            TomlValue::Array(val) => {
                let mut pod = Pod::new_array();
                for (index, item) in val.into_iter().enumerate() {
                    pod[index] = item.into();
                }
                pod
            }
            TomlValue::Table(val) => {
                let mut pod = Pod::new_hash();
                for (key, val) in val.into_iter() {
                    pod[key] = val.into();
                }
                pod
            }
            TomlValue::Datetime(val) => Pod::String(val.to_string()),
        }
    }
}

// todo: add more tests
#[test]
fn test_matter() {
    use crate::entity::ParsedEntity;
    use crate::matter::Matter;
    let matter: Matter<TOML> = Matter::new();
    let input = r#"---
title = "Home"
---
Some excerpt
---
Other stuff"#;

    let mut data = Pod::new_hash();
    data["title"] = Pod::String("Home".to_string());
    let parsed_entity = ParsedEntity {
        data,
        content: "Some excerpt\n---\nOther stuff",
        excerpt: "Some excerpt",
        orig: input,
    };

    assert_eq!(matter.matter(input), parsed_entity);
}
