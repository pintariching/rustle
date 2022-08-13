#[derive(Copy, Clone, Default)]
pub struct Options {
    pub offset_line: Option<usize>,
    pub offset_column: Option<usize>,
    pub start_index: Option<usize>,
}

impl Options {
    pub fn from_start_index(start_index: usize) -> Self {
        Self {
            offset_line: None,
            offset_column: None,
            start_index: Some(start_index),
        }
    }

    pub fn get_offset(&self) -> Offset {
        Offset {
            line: self.offset_line.unwrap_or(0),
            column: self.offset_column.unwrap_or(0),
        }
    }
}

#[derive(Copy, Clone)]
pub struct Range {
    start: usize,
    end: usize,
    line: usize,
}

#[derive(Copy, Clone, PartialEq, Debug)]
pub struct Location {
    pub line: usize,
    pub column: usize,
    pub character: usize,
}

#[derive(Copy, Clone, Default)]
pub struct Offset {
    pub line: usize,
    pub column: usize,
}

pub enum Search<'a> {
    Index(usize),
    Word(&'a str),
}

pub fn get_line_ranges(source: &str) -> Vec<Range> {
    let original_lines = source.split("\n");

    let mut start = 0;
    let mut line_ranges = Vec::new();
    for (i, line) in original_lines.enumerate() {
        let end = start + line.len() + 1;
        let range = Range {
            start,
            end,
            line: i,
        };
        start = end;
        line_ranges.push(range);
    }

    line_ranges
}

fn range_contains(range: &Range, index: usize) -> bool {
    range.start <= index && index < range.end
}

fn get_location(range: &Range, index: usize, offset: Offset) -> Location {
    let line = offset.line + range.line;
    let column = offset.column + index - range.start;
    let character = index;
    Location {
        line,
        column,
        character,
    }
}

fn locate_from_number(
    source: &str,
    search: usize,
    options: Option<Options>,
    current_index: &mut usize,
) -> Option<Location> {
    let offset = options.map(|o| o.get_offset()).unwrap_or(Offset::default());
    let line_ranges = get_line_ranges(&source);
    let mut range = line_ranges.get(*current_index);
    let end = range.map(|r| r.end).unwrap_or(0);

    let d: i8 = if search >= end { 1 } else { -1 };

    while let Some(r) = range {
        if range_contains(r, search) {
            return Some(get_location(&r, search, offset));
        }

        if d == -1 {
            return None;
        }

        *current_index = *current_index + d as usize;
        range = line_ranges.get(*current_index);
    }

    None
}

fn locate_from_string<'a>(
    source: &'a str,
    search: &'a str,
    options: Option<Options>,
    start_index: Option<usize>,
    current_index: &mut usize,
) -> Option<Location> {
    let start_index = start_index.unwrap_or(0);
    let search = source[start_index..].find(&search).map(|i| i + start_index);

    if let Some(search) = search {
        return locate_from_number(source, search, options, current_index);
    }

    None
}

pub fn get_locator<'a>(
    source: &'a str,
    options: Option<Options>,
) -> impl FnMut(Search<'a>, Option<usize>) -> Option<Location> {
    let mut current_index = 0;
    move |search: Search<'a>, start_index: Option<usize>| match search {
        Search::Index(word_index) => {
            locate_from_number(source, word_index, options, &mut current_index)
        }
        Search::Word(word) => {
            locate_from_string(source, word, options, start_index, &mut current_index)
        }
    }
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
    fn locates_by_character_index() {
        let mut locator = get_locator(SAMPLE, None);
        let actual = locator(Search::Index(0), None);
        let expected = Some(Location {
            line: 0,
            column: 0,
            character: 0,
        });

        assert_eq!(actual, expected)
    }

    #[test]
    fn respects_offset_line_x_and_offset_column_x() {
        let mut locator = get_locator(
            SAMPLE,
            Some(Options {
                offset_line: Some(2),
                offset_column: Some(2),
                start_index: None,
            }),
        );
        let actual = locator(Search::Index(0), None);
        let expected = Some(Location {
            line: 2,
            column: 2,
            character: 0,
        });

        assert_eq!(actual, expected)
    }

    #[test]
    fn locator_by_search_string() {
        let mut locator = get_locator(SAMPLE, None);
        let actual = locator(Search::Word("fly"), None);
        let expected = Some(Location {
            line: 0,
            column: 13,
            character: 13,
        });

        assert_eq!(actual, expected)
    }

    #[test]
    fn locator_by_search_string_with_start_index() {
        let mut locator = get_locator(SAMPLE, None);
        let mut location = locator(Search::Index(13), None);
        let expected = Some(Location {
            line: 0,
            column: 13,
            character: 13,
        });
        assert_eq!(location, expected);

        location = locator(Search::Word("fly"), Some(location.unwrap().character + 1));
        let expected = Some(Location {
            line: 2,
            column: 9,
            character: 76,
        });
        assert_eq!(location, expected);

        location = locator(Search::Word("fly"), Some(location.unwrap().character + 1));
        let expected = Some(Location {
            line: 3,
            column: 8,
            character: 104,
        });
        assert_eq!(location, expected);
    }
}
