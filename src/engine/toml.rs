use crate::engine::Engine;
use crate::Pod;
use toml::Value as TomlValue;

#[derive(PartialEq, Debug)]
pub struct TOML;

impl Engine for TOML {
    fn parse(content: &str) -> Pod {
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

#[cfg(test)]
mod test {
    use crate::engine::toml::TOML;
    use crate::entity::ParsedEntityStruct;
    use crate::matter::Matter;
    use serde::Deserialize;

    #[test]
    fn test_matter() {
        let matter: Matter<TOML> = Matter::new();
        let input = r#"---
title = "TOML"
description = "Front matter"
categories = "front matter toml"
---

# This file has toml front matter!
"#;
        #[derive(Deserialize, PartialEq, Debug)]
        struct FrontMatter {
            title: String,
            description: String,
            categories: String,
        }
        let data_expected = FrontMatter {
            title: "TOML".to_string(),
            description: "Front matter".to_string(),
            categories: "front matter toml".to_string(),
        };
        let result: ParsedEntityStruct<FrontMatter> = matter.parse_with_struct(input).unwrap();
        assert_eq!(result.data, data_expected);
    }
}
