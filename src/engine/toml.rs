use crate::engine::Engine;
use crate::Pod;
use crate::{Error, Result};
use std::collections::HashMap;
use toml::Value;

/// [`Engine`](crate::engine::Engine) for the [TOML](https://toml.io/) configuration format.
pub struct TOML;

impl Engine for TOML {
    fn parse(content: &str) -> Result<Pod> {
        match toml::from_str::<Value>(content) {
            Ok(value) => Ok(value.into()),
            Err(e) => Err(Error::deserialize_error(&format!("{}", e))),
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
    use crate::Matter;
    use crate::ParsedEntity;
    use crate::Result;
    use serde::Deserialize;

    #[test]
    fn test_matter() -> Result<()> {
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
        let result: ParsedEntity<FrontMatter> = matter.parse(input)?;
        assert_eq!(result.data, Some(data_expected));
        Ok(())
    }
}
