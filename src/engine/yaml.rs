use crate::engine::Engine;
use crate::Pod;
use std::collections::HashMap;
use yaml::{Yaml, YamlLoader};

/// [`Engine`](crate::engine::Engine) for the [YAML](https://yaml.org) configuration format.
pub struct YAML;

impl Engine for YAML {
    fn parse(content: &str) -> Pod {
        match YamlLoader::load_from_str(content) {
            Ok(docs) => {
                let mut doc = Pod::Null;
                if !docs.is_empty() {
                    doc = docs[0].clone().into();
                }
                doc
            }
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
            Yaml::Array(val) => val
                .iter()
                .map(|elem| elem.into())
                .collect::<Vec<Pod>>()
                .into(),
            Yaml::Hash(val) => val
                .iter()
                .filter_map(|(key, elem)| {
                    let key = match key {
                        Yaml::String(s) | Yaml::Real(s) => s.to_string(),
                        Yaml::Boolean(b) => b.to_string(),
                        Yaml::Integer(i) => i.to_string(),
                        Yaml::Null => "null".to_string(),
                        // Other types should not be expressible as keys.
                        _ => return None,
                    };

                    Some((key, elem.into()))
                })
                .collect::<HashMap<String, Pod>>()
                .into(),
            Yaml::Null => Pod::Null,
            _ => Pod::Null,
        }
    }
}

impl From<&Yaml> for Pod {
    fn from(val: &Yaml) -> Self {
        val.to_owned().into()
    }
}

#[cfg(test)]
mod test {
    use crate::engine::yaml::YAML;
    use crate::entity::ParsedEntity;
    use crate::matter::Matter;
    use serde::Deserialize;

    #[test]
    fn test_matter() {
        let matter: Matter<YAML> = Matter::new();
        let input = r#"---
one: foo
two: bar
three: baz
---"#;
        #[derive(Deserialize, PartialEq, Debug)]
        struct FrontMatter {
            one: String,
            two: String,
            three: String,
        }
        let data_expected = FrontMatter {
            one: "foo".to_string(),
            two: "bar".to_string(),
            three: "baz".to_string(),
        };
        let result: ParsedEntity<FrontMatter> = matter.parse(input);
        assert_eq!(result.data, Some(data_expected));
    }

    #[test]
    fn non_string_keys() {
        let matter: Matter<YAML> = Matter::new();
        let input = r#"---
1: foo
true: bar
three: baz
null: boo
---"#;
        #[derive(Deserialize, PartialEq, Debug)]
        struct FrontMatter {
            #[serde(rename = "1")]
            one: String,
            #[serde(rename = "true")]
            two: String,
            three: String,
            null: String,
        }
        let data_expected = FrontMatter {
            one: "foo".to_string(),
            two: "bar".to_string(),
            three: "baz".to_string(),
            null: "boo".to_string(),
        };
        let result: ParsedEntity<FrontMatter> = matter.parse(input);
        assert_eq!(result.data, Some(data_expected));
    }
}
