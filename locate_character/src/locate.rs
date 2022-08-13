use super::*;

pub fn locate<'a>(
    source: &'a str,
    search: Search<'a>,
    options: Option<Options>,
) -> Option<Location> {
    get_locator(source, options)(search, options.and_then(|o| o.start_index))
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &'static str = r#"A flea and a fly in a flue
Were imprisoned, so what could they do?
Said the fly, "let us flee!"
"Let us fly!" said the flea.
So they flew through a flaw in the flue."#;

    #[test]
    fn locates_a_character_by_index() {
        let actual = locate(SAMPLE, Search::Index(13), None);
        let expected = Some(Location {
            line: 0,
            column: 13,
            character: 13,
        });
        assert_eq!(actual, expected)
    }

    #[test]
    fn locates_a_character_by_string() {
        let actual = locate(SAMPLE, Search::Word("fly"), None);
        let expected = Some(Location {
            line: 0,
            column: 13,
            character: 13,
        });
        assert_eq!(actual, expected)
    }

    #[test]
    fn locates_a_character_by_string_with_start_index() {
        let actual = locate(
            SAMPLE,
            Search::Word("fly"),
            Some(Options::from_start_index(14)),
        );
        let expected = Some(Location {
            line: 2,
            column: 9,
            character: 76,
        });
        assert_eq!(actual, expected);

        let actual = locate(
            SAMPLE,
            Search::Word("fly"),
            Some(Options::from_start_index(77)),
        );
        let expected = Some(Location {
            line: 3,
            column: 8,
            character: 104,
        });
        assert_eq!(actual, expected)
    }

    #[test]
    fn respects_offset_line_x_and_offset_column_x() {
        let actual = locate(
            SAMPLE,
            Search::Index(13),
            Some(Options {
                offset_line: Some(2),
                offset_column: Some(2),
                ..Default::default()
            }),
        );
        let expected = Some(Location {
            line: 2,
            column: 15,
            character: 13,
        });
        assert_eq!(actual, expected)
    }
}
