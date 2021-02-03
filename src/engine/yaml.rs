use crate::engine::Engine;
use yaml_rust::yaml::Hash;
use yaml_rust::{Yaml, YamlLoader};

#[derive(PartialEq, Debug)]
pub struct YAML {}

impl Engine for YAML {
    type Output = Yaml;

    fn new() -> Self {
        return YAML {};
    }

    fn parse(&self, content: &str) -> Self::Output {
        match YamlLoader::load_from_str(content) {
            Ok(docs) => docs[0].clone(),
            Err(..) => self.init_data(),
        }
    }

    fn init_data(&self) -> Self::Output {
        return Yaml::Hash(Hash::new());
    }
}

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

    let mut hash_map = Hash::new();
    hash_map.insert(
        Yaml::String("title".to_string()),
        Yaml::String("Home".to_string()),
    );
    let data = Yaml::Hash(hash_map);
    let parsed_entity = ParsedEntity {
        data,
        content: "Some excerpt\n---\nOther stuff",
        excerpt: "Some excerpt",
        orig: input,
    };

    assert_eq!(matter.matter(input), parsed_entity);
}
