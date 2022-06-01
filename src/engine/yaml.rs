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
            Yaml::Hash(val) => {
                val.iter()
                    // FIXME: Here we assume that the key is a string. That is indeed the most
                    // common case, but in YAML, any value can be a key. We've managed well with
                    // only string keys so far, but we should handle this properly in the future.
                    .map(|(key, elem)| (key.to_owned().into_string().unwrap(), elem.into()))
                    .collect::<HashMap<String, Pod>>()
                    .into()
            }
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
    use crate::entity::ParsedEntityStruct;
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
        let result: ParsedEntityStruct<FrontMatter> = matter.parse_with_struct(input).unwrap();
        assert_eq!(result.data, data_expected);
    }
}
