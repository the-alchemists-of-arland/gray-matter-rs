use crate::engine::Engine;
use crate::Pod;
use std::collections::HashMap;
use toml::Value;

/// [`Engine`](crate::engine::Engine) for the [TOML](https://toml.io/) configuration format.
pub struct TOML;

impl Engine for TOML {
    fn parse(content: &str) -> Pod {
        match toml::from_str::<Value>(content) {
            Ok(value) => value.into(),
            Err(_) => Pod::Null,
        }
    }
}

impl From<Value> for Pod {
    fn from(toml_value: Value) -> Self {
        match toml_value {
            Value::String(val) => Pod::String(val),
            Value::Integer(val) => Pod::Integer(val),
            Value::Float(val) => Pod::Float(val),
            Value::Boolean(val) => Pod::Boolean(val),
            Value::Array(val) => val
                .iter()
                .map(|elem| elem.into())
                .collect::<Vec<Pod>>()
                .into(),
            Value::Table(val) => val
                .iter()
                .map(|(key, elem)| (key.to_owned(), elem.into()))
                .collect::<HashMap<String, Pod>>()
                .into(),
            Value::Datetime(val) => Pod::String(val.to_string()),
        }
    }
}

impl From<&Value> for Pod {
    fn from(val: &Value) -> Self {
        val.to_owned().into()
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
