use crate::compiler::utils::get_code_frame;
use rustle_locate_character::{locate, Options, Search};

#[derive(Default, PartialEq, Debug, Copy, Clone)]
pub struct Location {
    line: usize,
    column: usize,
}

#[derive(PartialEq, Debug)]
pub struct CompileError {
    code: String,
    start: Location,
    end: Location,
    pos: usize,
    filename: String,
    frame: String,
    message: String,
}

impl ToString for CompileError {
    fn to_string(&self) -> String {
        format!(
            "{} ({}:{})\n{}",
            self.message, self.start.line, self.start.column, self.frame
        )
    }
}

#[derive(Copy, Clone)]
pub struct NewErrorProps<'a> {
    pub name: &'a str,
    pub code: &'a str,
    pub source: &'a str,
    pub filename: &'a str,
    pub start: usize,
    pub end: Option<usize>,
}

impl CompileError {
    pub fn new(message: &str, props: NewErrorProps) -> Self {
        let start = locate(
            props.source,
            Search::Index(props.start),
            Some(Options {
                offset_line: Some(1),
                ..Default::default()
            }),
        )
        .map(|start| Location {
            line: start.line,
            column: start.column,
        });
        let end = locate(
            props.source,
            Search::Index(props.end.unwrap_or(props.start)),
            Some(Options {
                offset_line: Some(1),
                ..Default::default()
            }),
        )
        .map(|start| Location {
            line: start.line,
            column: start.column,
        });
        let frame = start
            .map(|start| get_code_frame(props.source.to_string(), start.line - 1, start.column));

        Self {
            code: props.code.to_string(),
            start: start.unwrap_or_default(),
            end: end.unwrap_or_default(),
            pos: props.start,
            filename: props.filename.to_string(),
            frame: frame.unwrap_or_default(),
            message: message.to_string(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // TODO: haven't test yet because compile error.
    #[test]
    fn create_new_error() {
        let actual = CompileError::new(
            "error",
            NewErrorProps {
                name: "name",
                code: "code",
                source: "source",
                filename: "filename",
                start: 0,
                end: Some(1),
            },
        );
        let expected = CompileError {
            code: "code".to_string(),
            start: Location { line: 1, column: 0 },
            end: Location { line: 1, column: 1 },
            pos: 0,
            filename: "filename".to_string(),
            frame: "1: source\n   ^".to_string(),
            message: "error".to_string(),
        };

        assert_eq!(actual, expected)
    }
}
