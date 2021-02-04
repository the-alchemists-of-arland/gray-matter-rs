use crate::engine::Engine;
use crate::entity::ParsedEntity;
use crate::value::pod::Pod;
use regex::Regex;

pub struct Matter<T: Engine> {
    pub delimiter: &'static str,
    pub excerpt_separator: &'static str,
    engine: T,
}

impl<T: Engine> Matter<T> {
    pub fn new() -> Self {
        return Matter {
            delimiter: "---",
            excerpt_separator: "---",
            engine: T::new(),
        };
    }

    /// **matter** takes a &str, extracts and parses front-matter from it,
    /// then returns a ParsedEntity result.
    /// ```rust
    /// let matter: gray_matter::matter::Matter<gray_matter::engine::yaml::YAML> = gray_matter::matter::Matter::new();
    /// let input = "---\ntitle: Home\n---\nOther stuff";
    /// let parsed_entity =  matter.matter(input);
    /// ```
    pub fn matter(&self, input: &'static str) -> ParsedEntity {
        let parsed_entity = ParsedEntity {
            data: Pod::new_hash(),
            content: input,
            excerpt: "",
            orig: input,
        };
        if input.is_empty() {
            return parsed_entity;
        }
        self.parse_matter(parsed_entity)
    }

    fn parse_matter(&self, mut entity: ParsedEntity) -> ParsedEntity {
        // the orig length should greater than the given delimiter
        if entity.orig.len() <= self.delimiter.len() {
            return entity;
        }
        if !entity.orig.starts_with(self.delimiter) {
            self.excerpt(&mut entity);
            return entity;
        }
        if entity.orig[3..4] == self.delimiter[0..1] {
            return entity;
        }
        // strip the opening delimiter
        let stripped = &entity.orig[self.delimiter.len()..];
        // check if close delimiter exists
        // if not, the full stripped content is the front matter
        let close_index = match stripped.find(self.delimiter) {
            Some(index) => index,
            None => stripped.len(),
        };
        let (raw_matter, rest) = stripped.split_at(close_index);
        let re = Regex::new(r"^\s*#[^\n]+").unwrap();
        let block = re.replace_all(raw_matter, "").into_owned();
        if !block.is_empty() {
            entity.data = self.engine.parse(block.trim())
        }
        if !rest.is_empty() && rest.len() > self.delimiter.len() {
            let (_, content) = rest.split_at(self.delimiter.len());
            let trim_content = if content.starts_with("\r") || content.starts_with("\n") {
                &content[1..]
            } else {
                content
            };
            entity.content = trim_content;
            self.excerpt(&mut entity);
        }
        return entity;
    }

    fn excerpt(&self, entity: &mut ParsedEntity) {
        let delimiter = if self.excerpt_separator.is_empty() {
            self.delimiter
        } else {
            self.excerpt_separator
        };
        match entity.content.find(delimiter) {
            Some(index) => {
                if index > 0 {
                    entity.excerpt = &entity.content[..index].trim();
                }
            }
            None => {}
        }
    }
}
