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
    matter.parse(&content)
}

fn matter_yaml_struct<D: DeserializeOwned>(file_name: &str) -> Option<ParsedEntityStruct<D>> {
    let content = read_content(file_name);
    let matter: Matter<YAML> = Matter::new();
    matter.parse_with_struct(&content)
}

#[test]
fn test_basic() {
    #[derive(serde::Deserialize, PartialEq, Debug)]
    struct FrontMatter {
        title: String,
    }
    let result: ParsedEntityStruct<FrontMatter> = matter_yaml_struct("basic.txt").unwrap();
    let data_expected = FrontMatter {
        title: "Basic".to_string(),
    };
    assert_eq!(
        result.data, data_expected,
        "should get front matter as {data_expected:?}"
    );
    assert_eq!(
        result.content, "this is content.",
        "should get content as \"this is content.\""
    );
}
#[test]
fn test_parse_empty() {
    let result = matter_yaml("empty.md");
    assert!(result.content.is_empty());
    assert!(result.data.is_none());
    assert!(result.excerpt.is_none())
}

#[test]
fn test_parse_complex_yaml_front_matter() {
    #[derive(serde::Deserialize, PartialEq, Debug)]
    struct FrontMatter {
        root: String,
        assets: String,
        analytics: Analytics,
    }
    #[derive(serde::Deserialize, PartialEq, Debug)]
    struct Analytics {
        alexa: String,
    }
    let result: ParsedEntityStruct<FrontMatter> = matter_yaml_struct("complex.md").unwrap();
    let data_expected = FrontMatter {
        root: "_gh_pages".to_string(),
        assets: "<%= site.dest %>/assets".to_string(),
        analytics: Analytics {
            alexa: "lpTeh1awA400OE".to_string(),
        },
    };
    assert_eq!(
        result.data, data_expected,
        "should get front matter as {data_expected:?}"
    );
    assert!(!result.content.is_empty(), "should get content");
    assert!(result.excerpt.is_some(), "should get excerpt")
}

#[test]
fn test_parse_no_matter() {
    let result = matter_yaml("hasnt-matter.md");
    assert!(
        result.data.is_none(),
        "Parsing `hasnt-matter.md` shold give `data` = None."
    );
    assert!(
        !result.content.is_empty(),
        "Parsing `hasnt-matter.md` should give non-empty `content`."
    )
}

#[test]
fn test_all_matter() {
    #[derive(serde::Deserialize, PartialEq, Debug)]
    struct FrontMatter {
        one: String,
        two: String,
        three: String,
    }
    let result = matter_yaml("all.yaml");
    assert!(
        result.data.is_none(),
        "Should not get any front matter from `all.yaml`.",
    );
    assert!(
        !result.content.is_empty(),
        "Parsing `all.yaml` should give non-empty `content`."
    );
    assert!(
        result.excerpt.is_none(),
        "Parsing `all.yaml` should give `excerpt` = None."
    );
}
