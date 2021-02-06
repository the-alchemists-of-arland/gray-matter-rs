use crate::engine::yaml::YAML;
use crate::entity::{ParsedEntity, ParsedEntityStruct};
use crate::matter::Matter;
use crate::value::pod::Pod;
use serde::de::DeserializeOwned;
use std::fs;
use std::path::Path;

fn get_fixtures(file_name: &str) -> impl AsRef<Path> {
    let mut root_dir = std::env::current_dir()
        .unwrap()
        .as_os_str()
        .to_str()
        .unwrap()
        .to_string();
    root_dir.push_str("/src/tests/fixtures/");
    root_dir.push_str(file_name);
    root_dir
}

fn read_content(file_name: &str) -> String {
    let path = get_fixtures(file_name);
    fs::read_to_string(path).expect("Cannot read")
}

fn matter_yaml(file_name: &str) -> ParsedEntity {
    let content = read_content(file_name);
    let matter: Matter<YAML> = Matter::new();
    matter.matter(content)
}

fn matter_yaml_struct<D: DeserializeOwned>(file_name: &str) -> ParsedEntityStruct<D> {
    let content = read_content(file_name);
    let matter: Matter<YAML> = Matter::new();
    matter.matter_struct(content)
}

#[test]
fn test_basic() {
    #[derive(serde::Deserialize, PartialEq)]
    struct FrontMatter {
        title: String,
    }
    let result: ParsedEntityStruct<FrontMatter> = matter_yaml_struct("basic.txt");
    assert_eq!(
        true,
        result.data
            == FrontMatter {
                title: "Basic".to_string()
            }
    );
    assert_eq!(true, result.content == "this is content.")
}
#[test]
fn test_parse_empty() {
    let result = matter_yaml("empty.md");
    assert_eq!(true, result.content.is_empty());
    assert_eq!(true, result.excerpt.is_empty());
    assert_eq!(true, result.orig.is_empty());
    assert_eq!(true, result.data == Pod::new_hash());
}

#[test]
fn test_parse_complex_yaml_front_matter() {
    #[derive(serde::Deserialize, PartialEq)]
    struct FrontMatter {
        root: String,
        assets: String,
        google: bool,
    }
    let result: ParsedEntityStruct<FrontMatter> = matter_yaml_struct("complex.md");
    assert_eq!(
        true,
        result.data
            == FrontMatter {
                root: "_gh_pages".to_string(),
                assets: "<%= site.dest %>/assets".to_string(),
                google: false,
            },
    );
    assert_eq!(false, result.content.is_empty());
    assert_eq!(false, result.excerpt.is_empty())
}

#[test]
fn test_parse_no_matter() {
    let result = matter_yaml("hasnt-matter.md");
    assert_eq!(true, result.data == Pod::new_hash());
}

#[test]
fn test_all_matter() {
    #[derive(serde::Deserialize, PartialEq)]
    struct FrontMatter {
        one: String,
        two: String,
        three: String,
    }
    let result: ParsedEntityStruct<FrontMatter> = matter_yaml_struct("all.yaml");
    assert_eq!(
        true,
        result.data
            == FrontMatter {
                one: "foo".to_string(),
                two: "bar".to_string(),
                three: "baz".to_string()
            }
    );
    assert_eq!(true, result.content.is_empty());
    assert_eq!(true, result.excerpt.is_empty());
}
