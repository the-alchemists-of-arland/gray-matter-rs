use crate::engine::Engine;
use crate::{ParsedEntity, ParsedEntityStruct};
use regex::Regex;

enum Part {
    Matter,
    MaybeExcerpt,
    Content,
}

pub struct Matter<T: Engine> {
    pub delimiter: String,
    pub excerpt_delimiter: Option<String>,
    engine: T,
}

impl<T: Engine> Matter<T> {
    pub fn new() -> Self {
        Self {
            delimiter: "---".to_string(),
            excerpt_delimiter: None,
            engine: T::new(),
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
    /// let parsed_entity =  matter.parse(input);
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
        if input.is_empty()
        || input.len() <= self.delimiter.len()
        {
            return parsed_entity;
        }

        // If excerpt delimiter is given, use it. Otherwise, use normal delimiter
        let excerpt_delimiter = self.excerpt_delimiter.clone()
            .unwrap_or_else(|| self.delimiter.clone());

        let mut lines = input.lines();

        // If first line starts with a delimiter followed by newline, we are looking at front
        // matter. Else, we might be looking at an excerpt.
        // FIXME: We are only trimming the start of the line. We might have a delimiter with
        // whitespace after: `---  \n`. gray_matter should handle this.
        let mut looking_at = if input.trim_start().starts_with(&(self.delimiter.clone() + "\n")) {
            lines.next();
            Part::Matter
        } else { Part::MaybeExcerpt };

        let mut acc = String::new();
        for line in lines {
            &line.to_string().push('\n');
            acc += &format!("\n{}", line);
            match looking_at {
                Part::Matter => {
                    if line.trim() == self.delimiter {
                        let comment_re = Regex::new(r"(?m)^\s*#[^\n]+").unwrap();
                        let matter = comment_re.replace_all(&acc, "")
                            .trim()
                            .strip_suffix(&self.delimiter).unwrap()
                            .trim_matches('\n')
                            .to_string();

                        if !matter.is_empty() {
                            parsed_entity.data = Some(self.engine.parse(&matter));
                            parsed_entity.matter = matter;
                        }

                        acc = String::new();
                        looking_at = Part::MaybeExcerpt;
                    }
                },

                Part::MaybeExcerpt => {
                    if line.trim() == excerpt_delimiter {
                        parsed_entity.excerpt = Some(
                            acc.trim()
                            .strip_prefix(&self.delimiter).unwrap_or(&acc)
                            .strip_suffix(&excerpt_delimiter).unwrap()
                            .trim_matches('\n')
                            .to_string()
                        );

                        looking_at = Part::Content;
                    }
                },

                Part::Content => {}
            }
        }

        parsed_entity.content = acc.trim().to_string();

        parsed_entity
    }

    /// Wrapper around [`parse`](Matter::parse), that deserializes any front matter into a custom struct.
    ///
    /// Supplied as an ease-of-use function to prevent having to deserialize manually.
    ///
    /// Returns `None` if no front matter is found, or if the front matter is not deserializable
    /// into the custom struct.
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
    /// let matter: Matter<YAML> = Matter::new();
    /// let input = "---\ntitle: Home\n---\nOther stuff";
    /// let parsed_entity =  matter.parse_with_struct::<Config>(input).unwrap();
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
    use crate::engine::yaml::YAML;
    use crate::entity::ParsedEntityStruct;
    use super::Matter;

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
        assert_eq!(
            true,
            result.data == front_matter,
            "should get front matter as {:?}",
            front_matter
        );
        matter.delimiter = "~~~".to_string();
        let result = matter.parse("---\nabc: xyz\n---");
        assert!(result.data.is_none(), "should get no front matter");
        let result: ParsedEntityStruct<FrontMatter> =
            matter.parse_with_struct("~~~\nabc: xyz\n~~~").unwrap();
        assert_eq!(
            result.data,
            front_matter,
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
            "---\n # this is a comment\n# another one\n# yet another\n---\nThis is content",
        ];
        for input in table.into_iter() {
            let result = matter.parse(input);
            assert!(result.data.is_none(), "should get no front matter");
            assert_eq!(
                result.content,
                "This is content",
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
            matter.parse_with_struct("---\nabc: xyz\n---\nfoo\nbar\nbaz\n---\ncontent").unwrap();
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
        let result: ParsedEntityStruct<FrontMatter> = matter.parse_with_struct(
            "---\nabc: xyz\n---\nfoo\nbar\nbaz\n<!-- endexcerpt -->\ncontent",
        ).unwrap();
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
            result.excerpt.unwrap(),
            "foo\nbar\nbaz",
            "should get excerpt as \"foo\nbar\nbaz\""
        );
        let result = matter.parse("foo\nbar\nbaz\n<!-- endexcerpt -->\ncontent");
        assert!(result.data.is_none(), "should get no front matter");
        assert_eq!(
            true,
            result.content == "foo\nbar\nbaz\n<!-- endexcerpt -->\ncontent".to_string(),
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
            "Looks similar to front matter:\n{}\nIs really just content.",
            raw
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
        assert_eq!(
            true,
            data_expected == result.data,
            "should get front matter as {:?}",
            data_expected
        );
        let content_expected =
            "<span class=\"alert alert-info\">This is an alert</span>".to_string();
        assert_eq!(
            result.content,
            content_expected,
            "should get content as {:?}",
            content_expected
        );
        #[derive(serde::Deserialize, PartialEq, Debug)]
        struct FrontMatterName {
            name: String,
        }
        let result: ParsedEntityStruct<FrontMatterName> = matter.parse_with_struct(
            r#"---
    name: "troublesome --- value"
    ---
    here is some content
    "#,).unwrap();
        let data_expected = FrontMatterName {
            name: "troublesome --- value".to_string(),
        };
        assert_eq!(
            true,
            result.data == data_expected,
            "should correctly identify delimiters and ignore strings that look like delimiters and get front matter as {:?}", data_expected
        );
        let result: ParsedEntityStruct<FrontMatterName> =
            matter.parse_with_struct("---\nname: \"troublesome --- value\"\n---").unwrap();
        assert_eq!(
            true,
            result.data == data_expected,
            "should correctly parse a string that only has an opening delimiter and get front matter as {:?}", data_expected
        );
        let result = matter.parse("-----------name--------------value\nfoo");
        assert!(
            result.data.is_none(),
            "should not try to parse a string has content that looks like front-matter"
        );
    }
}
