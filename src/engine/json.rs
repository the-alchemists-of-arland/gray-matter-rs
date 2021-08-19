use crate::engine::Engine;
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
            JsonValue::Number(val) => {
                let val_string = val.to_string();
                if val_string.contains(".") {
                    Pod::Float(val_string.parse().unwrap_or(0 as f64))
                } else {
                    Pod::Integer(val_string.parse().unwrap_or(0))
                }
            }
            JsonValue::Boolean(val) => Pod::Boolean(val),
            JsonValue::Array(val) => {
                let mut pod = Pod::new_array();
                for (index, item) in val.into_iter().enumerate() {
                    pod[index] = item.into();
                }
                pod
            }
            JsonValue::Object(val) => {
                let mut pod = Pod::new_hash();
                for (key, val) in val.iter() {
                    pod[key] = (*val).clone().into();
                }
                pod
            }
        }
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
