# Changelog

## 0.2.4

### Bug fixes

- Fixed overeager whitespace trimming.

## 0.2.3

### Enhancements

- Smaller footprint. A few dependencies have been removed, and the in-house YAML and TOML engines are now optional.

## 0.2.2

### Enhancements

- Excerpt delimiter is now allowed to be on the same line with excerpt content.

## 0.2.1

### Enhancements

- gray-matter is now more strict with delimiters and whitespace on the same line. Previously, whitespace was allowed both before and after the delimiter. Now, you can only have whitespace after the delimiter.
- gray-matter is also less strict with the first delimiter. It does not allow whitespace at the start of the line, but does so at the end (which it did not previously).

### Bug fixes

- Fixed a panic that was thrown when two delimiters directly followed eachother.

## 0.2.0

### Major changes

- Inputs that correctly start with a front matter delimiter, but which are not closed, are not parsed as all front matter anymore. Consider:

    ```
    ---
    field: Value

    Some text
    ```

    `---` in Markdown can also be used as a horisontal rule, which made starting a document with one - like the example above - impossible using this approach.

    gray_matter is now strict with regards to the front matter section actually being closed, like:


    ```
    ---
    field: Value
    ---

    Some text
    ```

### API changes

- Flatter API structure. You can now get access to most core structs and enums at the crate base. These are `Matter`, `ParsedEntity`, `ParsedEntityStruct`, `Error` and `Pod`. Engines are also made more accessible, being located directly in the `engine` module, like `gray_matter::engine::TOML`.
- Changes to `Matter` function names, to avoid repeated names and give a friendlier interface:

    - `Matter::matter` -> `Matter::parse`
    - `Matter::matter_struct` -> `Matter::parse_with_struct`

    It's also worth noting that these functions now take the `input` parameter as a string slice (`&str`) instead of a `String`, for flexibility.

- `Matter` field `excerpt_separator` is now called `excerpt_delimiter` for consistency.
- `ParsedEntity` and `ParsedEntityStruct` have the added field `matter`, that stores the raw front matter content. The `excerpt` field is now an `Option<String>`, that is `None` if no excerpt is found.

    In `ParsedEntity`, the field `data` is an `Option<Pod>`, that is `None` if no front matter is found.

### Enhancements

- More documentation. Most public-facing interface should now be documented.
- More idiomatic parse logic. Should be easier to maintain.
