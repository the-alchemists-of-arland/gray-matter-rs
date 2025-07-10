use crate::engine::Engine;
use crate::Pod;
use crate::{Error, Result};
use json::Value;
use std::collections::HashMap;

/// [`Engine`](crate::engine::Engine) for the [JSON](https://www.json.org/) configuration format.
pub struct JSON;

impl Engine for JSON {
    fn parse(content: &str) -> Result<Pod> {
        match content.parse::<Value>() {
            Ok(data) => Ok(data.into()),
            Err(e) => Err(Error::deserialize_error(&format!("{}", e))),
        }
    }
}

impl From<Value> for Pod {
    fn from(json_val: Value) -> Self {
        match json_val {
            Value::Null => Pod::Null,
            Value::String(val) => Pod::String(val),
            Value::Number(val) => {
                if let Some(int) = val.as_i64() {
                    Pod::Integer(int)
                } else {
                    // NOTE: Looking at the source of serde_json, it looks like `as_f64` will
                    // always be Some. https://docs.rs/serde_json/latest/src/serde_json/number.rs.html#240-249
                    Pod::Float(val.as_f64().unwrap())
                }
            }
            Value::Bool(val) => Pod::Boolean(val),
            Value::Array(val) => val
                .iter()
                .map(|elem| elem.into())
                .collect::<Vec<Pod>>()
                .into(),
            Value::Object(val) => val
                .iter()
                .map(|(key, elem)| (key.to_owned(), elem.into()))
                .collect::<HashMap<String, Pod>>()
                .into(),
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
    use crate::engine::JSON;
    use crate::Matter;
    use crate::ParsedEntity;
    use crate::Result;
    use serde::Deserialize;

    #[test]
    fn test_matter() -> Result<()> {
        let matter: Matter<JSON> = Matter::new();
        let input = r#"---
{
    "title": "JSON",
     "description": "Front Matter"
}
---
Some excerpt
---
Other stuff"#;
        #[derive(PartialEq, Deserialize, Debug)]
        struct FrontMatter {
            title: String,
            description: String,
        }
        let data_expected = FrontMatter {
            title: "JSON".to_string(),
            description: "Front Matter".to_string(),
        };
        let result: ParsedEntity<FrontMatter> = matter.parse(input)?;
        assert_eq!(result.data, Some(data_expected));
        Ok(())
    }
}
