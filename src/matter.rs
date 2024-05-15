use crate::engine::Engine;
use crate::{ParsedEntity, ParsedEntityStruct};
use std::fmt::Write;
use std::marker::PhantomData;

enum Part {
    Matter,
    MaybeExcerpt,
    Content,
}

/// Coupled with an [`Engine`](crate::engine::Engine) of choice, `Matter` stores delimiter(s) and
/// handles parsing.
pub struct Matter<T: Engine> {
    pub delimiter: String,
    pub close_delimiter: Option<String>,
    pub excerpt_delimiter: Option<String>,
    engine: PhantomData<T>,
}

impl<T: Engine> Default for Matter<T> {
    fn default() -> Self {
        Matter::new()
    }
}

impl<T: Engine> Matter<T> {
    pub fn new() -> Self {
        Self {
            delimiter: "---".to_string(),
            close_delimiter: None,
            excerpt_delimiter: None,
            engine: PhantomData,
        }
    }

    /// Runs parsing on the input. Uses the [engine](crate::engine) contained in `self` to parse any front matter
    /// detected.
    ///
    /// ## Examples
    ///
    /// Basic usage:
    ///
    /// ```rust
    /// # use gray_matter::Matter;
    /// # use gray_matter::engine::YAML;
    /// let matter: Matter<YAML> = Matter::new();
    /// let input = "---\ntitle: Home\n---\nOther stuff";
    /// let parsed_entity = matter.parse(input);
    ///
    /// assert_eq!(parsed_entity.content, "Other stuff");
    /// ```
    pub fn parse(&self, input: &str) -> ParsedEntity {
        // Initialize ParsedEntity
        let mut parsed_entity = ParsedEntity {
            data: None,
            excerpt: None,
            content: String::new(),
            orig: input.to_owned(),
            matter: String::new(),
        };

        // Check if input is empty or shorter than the delimiter
        if input.is_empty() || input.len() <= self.delimiter.len() {
            return parsed_entity;
        }

        // If excerpt delimiter is given, use it. Otherwise, use normal delimiter
        let excerpt_delimiter = self
            .excerpt_delimiter
            .clone()
            .unwrap_or_else(|| self.delimiter.clone());

        let close_delimiter = self
            .close_delimiter
            .clone()
            .unwrap_or_else(|| self.delimiter.clone());
        // If first line starts with a delimiter followed by newline, we are looking at front
        // matter. Else, we might be looking at an excerpt.
        let (mut looking_at, lines) = match input.split_once('\n') {
            Some((first_line, rest)) if first_line.trim_end() == self.delimiter => {
                (Part::Matter, rest.lines())
            }
            _ => (Part::MaybeExcerpt, input.lines()),
        };

        let mut acc = String::new();
        for line in lines {
            let line = line.trim_end();
            match looking_at {
                Part::Matter => {
                    if line == self.delimiter || line == close_delimiter {
                        let matter = acc.trim().to_string();

                        if !matter.is_empty() {
                            parsed_entity.data = Some(T::parse(&matter));
                            parsed_entity.matter = matter;
                        }

                        acc = String::new();
                        looking_at = Part::MaybeExcerpt;
                        continue;
                    }
                }

                Part::MaybeExcerpt => {
                    if line.ends_with(&excerpt_delimiter) {
                        parsed_entity.excerpt = Some(
                            format!(
                                "{}\n{}",
                                acc.trim_start_matches('\n'),
                                line.strip_suffix(&excerpt_delimiter).unwrap(),
                            )
                            .trim_end()
                            .to_string(),
                        );

                        looking_at = Part::Content;
                    }
                }

                Part::Content => {}
            }

            write!(&mut acc, "\n{line}").unwrap();
        }

        parsed_entity.content = acc.trim_start_matches('\n').to_string();

        parsed_entity
    }

    /// Wrapper around [`parse`](Matter::parse), that deserializes any front matter into a custom
    /// struct. Supplied as an ease-of-use function to prevent having to deserialize manually.
    ///
    /// Returns `None` if no front matter is found, or if the front matter is not deserializable
    /// into the custom struct.
    ///
    /// ## Examples
    ///
    /// Basic usage:
    ///
    /// ```rust
    /// # use gray_matter::Matter;
    /// # use gray_matter::engine::YAML;
    /// # use gray_matter::ParsedEntityStruct;
    /// #[derive(serde::Deserialize)]
    /// struct Config {
    ///     title: String,
    /// }
    ///
    /// let matter: Matter<YAML> = Matter::new();
    /// let input = "---\ntitle: Home\n---\nOther stuff";
    /// let parsed_entity =  matter.parse_with_struct::<Config>(input).unwrap();
    ///
    /// assert_eq!(parsed_entity.data.title, "Home");
    /// ```
    pub fn parse_with_struct<D: serde::de::DeserializeOwned>(
        &self,
        input: &str,
    ) -> Option<ParsedEntityStruct<D>> {
        let parsed_entity = self.parse(input);
        let data: D = parsed_entity.data?.deserialize().ok()?;

        Some(ParsedEntityStruct {
            data,
            content: parsed_entity.content,
            excerpt: parsed_entity.excerpt,
            orig: parsed_entity.orig,
            matter: parsed_entity.matter,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::Matter;
    use crate::engine::{TOML, YAML};
    use crate::ParsedEntityStruct;

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
            matter.parse_with_struct("---\nabc: xyz\n---").unwrap();
        assert!(
            result.data == front_matter,
            "{}",
            "should get front matter as {front_matter:?}",
        );
        matter.delimiter = "~~~".to_string();
        let result = matter.parse("---\nabc: xyz\n---");
        assert!(result.data.is_none(), "should get no front matter");
        let result: ParsedEntityStruct<FrontMatter> =
            matter.parse_with_struct("~~~\nabc: xyz\n~~~").unwrap();
        assert_eq!(
            result.data, front_matter,
            "{}",
            "should get front matter by custom delimiter"
        );
        let result = matter.parse("\nabc: xyz\n~~~");
        assert!(result.data.is_none(), "should get no front matter");
    }

    #[test]
    fn test_front_matter_with_different_delimiters() {
        #[derive(serde::Deserialize, PartialEq, Debug)]
        struct FrontMatter {
            abc: String,
        }
        let front_matter = FrontMatter {
            abc: "xyz".to_string(),
        };
        let mut matter: Matter<YAML> = Matter::new();
        let result: ParsedEntityStruct<FrontMatter> =
            matter.parse_with_struct("---\nabc: xyz\n---").unwrap();
        assert!(
            result.data == front_matter,
            "{}",
            "should get front matter as {front_matter:?}"
        );
        matter.delimiter = "<!--".to_string();
        matter.close_delimiter = Some("-->".to_string());
        let result = matter.parse("---\nabc: xyz\n---");
        assert!(result.data.is_none(), "should get no front matter");
        let result: ParsedEntityStruct<FrontMatter> =
            matter.parse_with_struct("<!--\nabc: xyz\n-->").unwrap();
        assert_eq!(
            result.data, front_matter,
            "{}",
            "should get front matter by custom delimiter"
        );
        let result = matter.parse("\nabc: xyz\n~~~");
        assert!(result.data.is_none(), "should get no front matter");
    }

    #[test]
    pub fn test_empty_matter() {
        let matter: Matter<YAML> = Matter::new();
        let table = vec![
            "---\n---\nThis is content",
            "---\n\n---\nThis is content",
            "---\n\n\n\n\n\n---\nThis is content",
        ];
        for input in table.into_iter() {
            let result = matter.parse(input);
            assert!(result.data.is_none(), "should get no front matter");
            assert_eq!(result.content, "This is content");
        }
    }

    #[test]
    pub fn test_matter_excerpt() {
        #[derive(serde::Deserialize, PartialEq)]
        struct FrontMatter {
            abc: String,
        }
        let mut matter: Matter<YAML> = Matter::new();
        let result: ParsedEntityStruct<FrontMatter> = matter
            .parse_with_struct("---\nabc: xyz\n---\nfoo\nbar\nbaz\n---\ncontent")
            .unwrap();
        assert_eq!(
            result.data.abc,
            "xyz".to_string(),
            "should get front matter xyz as value of abc"
        );
        assert_eq!(
            result.content,
            "foo\nbar\nbaz\n---\ncontent".to_string(),
            "should get content as \"foo\nbar\nbaz\n---\ncontent\"",
        );
        assert_eq!(
            result.excerpt.unwrap(),
            "foo\nbar\nbaz",
            "should get an excerpt after front matter"
        );
        matter.excerpt_delimiter = Some("<!-- endexcerpt -->".to_string());
        let result: ParsedEntityStruct<FrontMatter> = matter
            .parse_with_struct("---\nabc: xyz\n---\nfoo\nbar\nbaz\n<!-- endexcerpt -->\ncontent")
            .unwrap();
        assert!(
            result.data.abc == *"xyz",
            "should get front matter xyz as value of abc"
        );
        assert!(
            result.content == *"foo\nbar\nbaz\n<!-- endexcerpt -->\ncontent",
            "should use a custom separator"
        );
        assert_eq!(
            result.excerpt.unwrap(),
            "foo\nbar\nbaz",
            "should get excerpt as \"foo\nbar\nbaz\""
        );

        // Check that the endexcerpt delimiter can be on the same line
        let result: ParsedEntityStruct<FrontMatter> = matter
            .parse_with_struct("---\nabc: xyz\n---\nfoo\nbar\nbaz<!-- endexcerpt -->\ncontent")
            .unwrap();
        assert!(
            result.data.abc == *"xyz",
            "should get front matter xyz as value of abc"
        );
        assert!(
            result.content == *"foo\nbar\nbaz<!-- endexcerpt -->\ncontent",
            "should use a custom separator"
        );
        assert_eq!(
            result.excerpt.unwrap(),
            "foo\nbar\nbaz",
            "should get excerpt as \"foo\nbar\nbaz\""
        );
        let result = matter.parse("foo\nbar\nbaz\n<!-- endexcerpt -->\ncontent");
        assert!(result.data.is_none(), "should get no front matter");
        assert!(
            result.content == *"foo\nbar\nbaz\n<!-- endexcerpt -->\ncontent",
            "should get content as \"foo\nbar\nbaz\n<!-- endexcerpt -->\ncontent\"",
        );
        assert_eq!(
            result.excerpt.unwrap(),
            "foo\nbar\nbaz",
            "should use a custom separator when no front-matter exists"
        );
    }

    #[test]
    fn test_parser() {
        let matter: Matter<YAML> = Matter::new();
        let raw = "---whatever\nabc: xyz\n---".to_string();
        let result = matter.parse(&raw);
        assert!(
            result.data.is_none(),
            "extra characters should get no front matter"
        );
        assert!(
            !result.content.is_empty(),
            "{}",
            "Looks similar to front matter:\n{raw}\nIs really just content."
        );
        let result = matter.parse("--- true\n---");
        assert!(
            result.data.is_none(),
            "boolean yaml types should get no front matter"
        );
        let result = matter.parse("--- 233\n---");
        assert!(
            result.data.is_none(),
            "number yaml types should get no front matter"
        );
        assert!(
            matter.parse("").data.is_none(),
            "Empty string should give `data` = None."
        );
        #[derive(serde::Deserialize, PartialEq, Debug)]
        struct FrontMatter {
            abc: String,
            version: i64,
        }
        let result: ParsedEntityStruct<FrontMatter> = matter.parse_with_struct("---\nabc: xyz\nversion: 2\n---\n\n<span class=\"alert alert-info\">This is an alert</span>\n").unwrap();
        let data_expected = FrontMatter {
            abc: "xyz".to_string(),
            version: 2,
        };
        assert!(
            data_expected == result.data,
            "{}",
            "should get front matter as {data_expected:?} "
        );
        let content_expected =
            "<span class=\"alert alert-info\">This is an alert</span>".to_string();
        assert_eq!(
            result.content, content_expected,
            "should get content as {content_expected:?}"
        );
        #[derive(serde::Deserialize, PartialEq, Debug)]
        struct FrontMatterName {
            name: String,
        }
        let result: ParsedEntityStruct<FrontMatterName> = matter
            .parse_with_struct(
                r#"---
name: "troublesome --- value"
---
here is some content
"#,
            )
            .unwrap();
        let data_expected = FrontMatterName {
            name: "troublesome --- value".to_string(),
        };
        assert!(
            result.data == data_expected, "{}",
            "should correctly identify delimiters and ignore strings that look like delimiters and get front matter as {data_expected:?}"
        );
        let result: ParsedEntityStruct<FrontMatterName> = matter
            .parse_with_struct("---\nname: \"troublesome --- value\"\n---")
            .unwrap();
        assert!(
            result.data == data_expected, "{}",
            "should correctly parse a string that only has an opening delimiter and get front matter as {data_expected:?}"
        );
        let result = matter.parse("-----------name--------------value\nfoo");
        assert!(
            result.data.is_none(),
            "should not try to parse a string has content that looks like front-matter"
        );
        let result = matter.parse("---\nname: ---\n---\n---\n");
        assert_eq!(
            result.content, "---",
            "should correctly handle rogue delimiter"
        );
        let result = matter.parse("---\nname: bar\n---\n---\n---");
        assert_eq!(
            result.content, "---\n---",
            "should correctly handle two rogue delimiter"
        );
    }

    #[test]
    #[allow(clippy::approx_constant)]
    fn test_int_vs_float() {
        #[derive(serde::Deserialize, PartialEq)]
        struct FrontMatter {
            int: i64,
            float: f64,
        }
        let raw = r#"---
int = 42
float = 3.14159265
---"#;
        let matter: Matter<TOML> = Matter::new();
        let result = matter.parse_with_struct::<FrontMatter>(raw).unwrap();

        assert_eq!(result.data.int, 42_i64);
        assert_eq!(result.data.float, 3.14159265_f64);
    }

    #[test]
    fn test_whitespace_content() {
        let raw = r#"---
field1 = "Value"
field2 = [3.14, 42]
---

    this is code block

# This is header"#;
        let matter: Matter<TOML> = Matter::new();
        let result = matter.parse(raw);

        assert_eq!(result.content, "    this is code block\n\n# This is header")
    }

    #[test]
    fn test_whitespace_without_frontmatter() {
        let matter: Matter<YAML> = Matter::new();
        let raw = r#"    An excerpt
---
    This is my content"#;
        let result = matter.parse(raw);

        assert_eq!(
            result.content,
            "    An excerpt\n---\n    This is my content"
        );

        assert_eq!(result.excerpt.unwrap(), "    An excerpt".to_string());
    }
}
