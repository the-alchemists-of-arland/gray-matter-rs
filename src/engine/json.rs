use crate::engine::Engine;
use crate::Pod;
use serde_json::Value;
use std::collections::HashMap;

/// [`Engine`](crate::engine::Engine) for the [JSON](https://www.json.org/) configuration format.
pub struct JSON;

impl Engine for JSON {
    fn parse(content: &str) -> Pod {
        match content.parse::<Value>() {
            Ok(data) => data.into(),
            Err(_) => Pod::Null,
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
            Value::Array(val) => {
                val.iter()
                    .map(|elem| elem.into())
                    .collect::<Vec<Pod>>()
                    .into()
            }
            Value::Object(val) => {
                val.iter()
                    .map(|(key, elem)| (key.to_owned(), elem.into()))
                    .collect::<HashMap<String, Pod>>()
                    .into()
            }
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
    use crate::engine::json::JSON;
    use crate::entity::ParsedEntityStruct;
    use crate::matter::Matter;
    use serde::Deserialize;

    #[test]
    fn test_matter() {
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
        let result: ParsedEntityStruct<FrontMatter> = matter.parse_with_struct(input).unwrap();
        assert_eq!(result.data, data_expected);
    }
}
