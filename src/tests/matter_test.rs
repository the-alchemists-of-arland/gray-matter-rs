use crate::engine::yaml::YAML;
use crate::entity::ParsedEntityStruct;
use crate::matter::Matter;

#[test]
fn test_front_matter() {
    #[derive(serde::Deserialize, PartialEq, Debug)]
    struct FrontMatter {
        abc: String,
    }
    let front_matter = FrontMatter {
        abc: "xyz".to_string(),
    };
    let mut matter: Matter<YAML> = Matter::new();
    let result: ParsedEntityStruct<FrontMatter> =
        matter.matter_struct("---\nabc: xyz\n---".to_string());
    assert_eq!(
        true,
        result.data == front_matter,
        "should get front matter as {:?}",
        front_matter
    );
    matter.delimiter = "~~~";
    let result = matter.matter("---\nabc: xyz\n---".to_string());
    assert_eq!(true, result.data.len() == 0, "should get no front matter");
    let result: ParsedEntityStruct<FrontMatter> =
        matter.matter_struct("~~~\nabc: xyz\n~~~".to_string());
    assert_eq!(
        true,
        result.data == front_matter,
        "{}",
        "should get front matter by custom delimiter"
    );
    let result = matter.matter("\nabc: xyz\n~~~".to_string());
    assert_eq!(true, result.data.len() == 0, "should get no front matter");
}

#[test]
pub fn test_empty_matter() {
    let matter: Matter<YAML> = Matter::new();
    let table = vec![
        "---\n---\nThis is content",
        "---\n\n---\nThis is content",
        "---\n\n\n\n\n\n---\nThis is content",
        "---\n # this is a comment\n# another one\n---\nThis is content",
    ];
    for input in table.into_iter() {
        let result = matter.matter(input.to_string());
        assert_eq!(true, result.data.len() == 0, "should get no front matter");
        assert_eq!(
            true,
            result.content == "This is content",
            "should get content as \"This is content\""
        );
    }
}

#[test]
pub fn test_matter_excerpt() {
    #[derive(serde::Deserialize, PartialEq)]
    struct FrontMatter {
        abc: String,
    }
    let mut matter: Matter<YAML> = Matter::new();
    let result: ParsedEntityStruct<FrontMatter> =
        matter.matter_struct("---\nabc: xyz\n---\nfoo\nbar\nbaz\n---\ncontent".to_string());
    assert_eq!(
        true,
        result.data.abc == "xyz".to_string(),
        "should get front matter xyz as value of abc"
    );
    assert_eq!(
        true,
        result.content == "foo\nbar\nbaz\n---\ncontent".to_string(),
        "should get content as \"foo\nbar\nbaz\n---\ncontent\"",
    );
    assert_eq!(
        true,
        result.excerpt == "foo\nbar\nbaz",
        "should get an excerpt after front matter"
    );
    matter.excerpt_separator = "<!-- endexcerpt -->";
    let result: ParsedEntityStruct<FrontMatter> = matter.matter_struct(
        "---\nabc: xyz\n---\nfoo\nbar\nbaz\n<!-- endexcerpt -->\ncontent".to_string(),
    );
    assert_eq!(
        true,
        result.data.abc == "xyz".to_string(),
        "should get front matter xyz as value of abc"
    );
    assert_eq!(
        true,
        result.content == "foo\nbar\nbaz\n<!-- endexcerpt -->\ncontent".to_string(),
        "should use a custom separator"
    );
    assert_eq!(
        true,
        result.excerpt == "foo\nbar\nbaz",
        "should get excerpt as \"foo\nbar\nbaz\""
    );
    let result = matter.matter("foo\nbar\nbaz\n<!-- endexcerpt -->\ncontent".to_string());
    assert_eq!(true, result.data.len() == 0, "should get no front matter");
    assert_eq!(
        true,
        result.content == "foo\nbar\nbaz\n<!-- endexcerpt -->\ncontent".to_string(),
        "should get content as \"foo\nbar\nbaz\n<!-- endexcerpt -->\ncontent\"",
    );
    assert_eq!(
        true,
        result.excerpt == "foo\nbar\nbaz",
        "should use a custom separator when no front-matter exists"
    );
}

#[test]
fn test_parser() {
    let matter: Matter<YAML> = Matter::new();
    let result = matter.matter("---whatever\nabc: xyz\n---".to_string());
    assert_eq!(
        true,
        result.data.len() == 0,
        "exstra characters should get no front matter"
    );
    assert_eq!(
        true,
        result.content.is_empty(),
        "exstra characters should get no content"
    );
    let result = matter.matter("--- true\n---".to_string());
    assert_eq!(
        true,
        result.data.len() == 0,
        "boolean yaml types should get no front matter"
    );
    let result = matter.matter("--- 233\n---".to_string());
    assert_eq!(
        true,
        result.data.len() == 0,
        "number yaml types should get no front matter"
    );
    let result = matter.matter(String::new());
    assert_eq!(
        true,
        result.data.len() == 0,
        "empty input should get no front matter"
    );
    #[derive(serde::Deserialize, PartialEq, Debug)]
    struct FrontMatter {
        abc: String,
        version: i64,
    }
    let result: ParsedEntityStruct<FrontMatter> = matter.matter_struct("---\nabc: xyz\nversion: 2\n---\n\n<span class=\"alert alert-info\">This is an alert</span>\n".to_string());
    let data_expected = FrontMatter {
        abc: "xyz".to_string(),
        version: 2,
    };
    assert_eq!(
        true,
        data_expected == result.data,
        "should get front matter as {:?}",
        data_expected
    );
    let content_expected =
        "\n<span class=\"alert alert-info\">This is an alert</span>\n".to_string();
    assert_eq!(
        true,
        result.content == content_expected,
        "should get content as {:?}",
        content_expected
    );
    #[derive(serde::Deserialize, PartialEq, Debug)]
    struct FrontMatterName {
        name: String,
    }
    let result: ParsedEntityStruct<FrontMatterName> = matter.matter_struct(
        r#"---
name: "troublesome --- value"
---
here is some content
"#
        .to_string(),
    );
    let data_expected = FrontMatterName {
        name: "troublesome --- value".to_string(),
    };
    assert_eq!(
        true,
        result.data == data_expected,
        "should correctly identify delimiters and ignore strings that look like delimiters and get front matter as {:?}", data_expected
    );
    let result: ParsedEntityStruct<FrontMatterName> =
        matter.matter_struct("---\nname: \"troublesome --- value\"\n".to_string());
    assert_eq!(
        true,
        result.data == data_expected,
        "should correctly parse a string that only has an opening delimiter and get front matter as {:?}", data_expected
    );
    let result = matter.matter("-----------name--------------value\nfoo".to_string());
    assert_eq!(
        true,
        result.data.len() == 0,
        "should not try to parse a string has content that looks like front-matter"
    );
}
