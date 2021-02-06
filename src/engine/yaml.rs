use crate::engine::Engine;
use crate::value::pod::Pod;
use yaml_rust::{Yaml, YamlLoader};

#[derive(PartialEq, Debug)]
pub struct YAML {}

impl Engine for YAML {
    fn new() -> Self {
        return YAML {};
    }

    fn parse(&self, content: &str) -> Pod {
        match YamlLoader::load_from_str(content) {
            Ok(docs) => docs[0].clone().into(),
            Err(..) => Pod::Null,
        }
    }
}

impl Into<Pod> for Yaml {
    fn into(self) -> Pod {
        match self {
            Yaml::Real(val) => Pod::Float(val.parse().unwrap_or(0 as f64)),
            Yaml::Integer(val) => Pod::Integer(val),
            Yaml::String(val) => Pod::String(val),
            Yaml::Boolean(val) => Pod::Boolean(val),
            Yaml::Array(val) => {
                let mut pod = Pod::new_array();
                for (index, item) in val.into_iter().enumerate() {
                    pod[index] = item.into();
                }
                pod
            }
            Yaml::Hash(val) => {
                let mut pod = Pod::new_hash();
                for (key, val) in val.into_iter() {
                    pod[key.as_str().unwrap()] = val.into();
                }
                pod
            }
            Yaml::Null => Pod::Null,
            _ => Pod::Null,
        }
    }
}

// todo: add more tests
#[test]
fn test_matter() {
    use crate::entity::ParsedEntity;
    use crate::matter::Matter;
    let matter: Matter<YAML> = Matter::new();
    let input = r#"---
title: Home
---
Some excerpt
---
Other stuff"#;

    let mut data = Pod::new_hash();
    data["title"] = Pod::String("Home".to_string());
    let parsed_entity = ParsedEntity {
        data,
        content: "Some excerpt\n---\nOther stuff".to_string(),
        excerpt: "Some excerpt".to_string(),
        orig: input.to_string(),
    };

    assert_eq!(matter.matter(input.to_string()), parsed_entity);
}
