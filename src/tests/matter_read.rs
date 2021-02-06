use crate::engine::yaml::YAML;
use crate::entity::{ParsedEntity, ParsedEntityStruct};
use crate::matter::Matter;
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
    #[derive(serde::Deserialize, PartialEq, Debug)]
    struct FrontMatter {
        title: String,
    }
    let result: ParsedEntityStruct<FrontMatter> = matter_yaml_struct("basic.txt");
    let data_expected = FrontMatter {
        title: "Basic".to_string(),
    };
    assert_eq!(
        true,
        result.data == data_expected,
        "should get front matter as {:?}",
        data_expected
    );
    assert_eq!(
        true,
        result.content == "this is content.",
        "should get content as \"this is content.\""
    );
}
#[test]
fn test_parse_empty() {
    let result = matter_yaml("empty.md");
    assert_eq!(true, result.content.is_empty(), "should get no content");
    assert_eq!(true, result.excerpt.is_empty(), "should get no excerpt");
    assert_eq!(true, result.orig.is_empty(), "should get no orig");
    assert_eq!(true, result.data.len() == 0, "should get no front matter");
}

#[test]
fn test_parse_complex_yaml_front_matter() {
    #[derive(serde::Deserialize, PartialEq, Debug)]
    struct FrontMatter {
        root: String,
        assets: String,
        google: bool,
    }
    let result: ParsedEntityStruct<FrontMatter> = matter_yaml_struct("complex.md");
    let data_expected = FrontMatter {
        root: "_gh_pages".to_string(),
        assets: "<%= site.dest %>/assets".to_string(),
        google: false,
    };
    assert_eq!(
        true,
        result.data == data_expected,
        "should get front matter as {:?}",
        data_expected
    );
    assert_eq!(false, result.content.is_empty(), "should get content");
    assert_eq!(false, result.excerpt.is_empty(), "should get excerpt")
}

#[test]
fn test_parse_no_matter() {
    let result = matter_yaml("hasnt-matter.md");
    assert_eq!(true, result.data.len() == 0, "should get no front matter");
}

#[test]
fn test_all_matter() {
    #[derive(serde::Deserialize, PartialEq, Debug)]
    struct FrontMatter {
        one: String,
        two: String,
        three: String,
    }
    let result: ParsedEntityStruct<FrontMatter> = matter_yaml_struct("all.yaml");
    let data_expected = FrontMatter {
        one: "foo".to_string(),
        two: "bar".to_string(),
        three: "baz".to_string(),
    };
    assert_eq!(
        true,
        result.data == data_expected,
        "should get front matter as {:?}",
        data_expected
    );
    assert_eq!(true, result.content.is_empty(), "should get no content");
    assert_eq!(true, result.excerpt.is_empty(), "should get no excerpt");
}
