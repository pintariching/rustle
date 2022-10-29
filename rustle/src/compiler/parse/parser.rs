use lazy_static::lazy_static;
use regex::Regex;

use crate::compiler::{Fragment, RustleAst};

use super::fragments::parse_fragments;

lazy_static! {
    static ref WHITESPACE: Regex = Regex::new("[\\s\n]").unwrap();
}

#[derive(Debug, Clone)]
pub struct Parser {
    pub index: usize,
    pub content: String,
}

impl Parser {
    /// Creates a new Parser struct and sets the index to 0
    /// To parse the file to an AST use the `.parse()` function
    ///  
    /// # Arguments
    ///
    /// * `content` - the file content as a string
    ///
    /// # Examples
    ///
    /// ```
    /// use std::fs;
    /// use rustle::compiler::parse::Parser;
    ///
    /// let source = fs::read_to_string("tests/app.svelte").unwrap();
    /// let parser = Parser::new(&source);
    /// ```
    pub fn new(content: &str) -> Self {
        Self {
            index: 0,
            content: content.into(),
        }
    }

    /// Parses the content to an AST and returns it
    pub fn parse(&mut self) -> RustleAst {
        let mut fragments = parse_fragments(self, |parser| parser.index < parser.content.len());

        let script_index = fragments
            .iter()
            .position(|f| match f {
                Fragment::Script(_) => true,
                _ => false,
            })
            .unwrap();

        let script = if let Fragment::Script(s) = fragments.remove(script_index) {
            Some(s)
        } else {
            None
        }
        .unwrap();

        RustleAst { script, fragments }
    }

    /// Checks if the string at the current index
    /// matches the provided string
    ///
    /// # Arguments
    ///
    /// * `str` - The string to match
    ///
    /// # Examples
    ///
    /// ```
    /// use rustle::compiler::parse::Parser;
    ///
    /// let mut parser = Parser::new("rustle is awesome");
    /// assert!(parser.match_str("rustle"));
    ///
    /// parser.index = 10;
    /// assert!(parser.match_str("awesome"));
    /// ```
    pub fn match_str(&mut self, str: &str) -> bool {
        self.content
            .chars()
            .skip(self.index)
            .take(str.len())
            .collect::<String>()
            .as_str()
            == str
    }

    /// Eats the provided string at the index
    /// if it matches and advances the index.
    /// If not, then it panics.
    ///
    /// # Arguments
    ///
    /// * `str` - The string to eat
    ///
    /// # Examples
    ///
    /// ```
    /// use rustle::compiler::parse::Parser;
    ///
    /// let mut parser = Parser::new("rustle is awesome");
    ///
    /// parser.eat("rustle");
    /// assert_eq!(parser.index, 6);
    ///
    /// parser.eat(" ");
    /// assert_eq!(parser.index, 7);
    ///
    /// parser.eat("is");
    /// assert_eq!(parser.index, 9);
    /// ```
    pub fn eat(&mut self, str: &str) {
        if self.match_str(str) {
            self.index += str.len();
        } else {
            panic!("Parse error: expecting {}", str);
        }
    }

    /// Reads the content at the index untill
    /// the content matches the `Regex`
    ///
    /// # Arguments
    ///
    /// * `regex` - The regex to match aginst
    ///
    /// # Examples
    ///
    /// ```
    /// use rustle::compiler::parse::Parser;
    ///	use regex::Regex;
    ///
    /// let mut parser = Parser::new("rustle is awesome");
    /// // Regex that matches the letters a-z
    /// let regex = Regex::new("[a-z]").unwrap();
    ///
    /// assert_eq!(parser.read_while_matching(&regex), "rustle".to_string());
    ///
    /// parser.index = 10;
    /// assert_eq!(parser.read_while_matching(&regex), "awesome".to_string());
    /// ```
    pub fn read_while_matching(&mut self, regex: &Regex) -> String {
        let start_index = self.index;

        while regex.is_match(
            self.content
                .chars()
                .nth(self.index)
                .unwrap()
                .to_string()
                .as_str(),
        ) {
            self.index += 1;

            if self.index >= self.content.len() {
                break;
            }
        }

        self.content
            .get(start_index..self.index)
            .unwrap()
            .to_string()
    }

    /// Advances the index untill the next non-whitespace character.
    ///
    /// The same as running `parser.read_while_matching(Regex::new("[\\s\n]").unwrap());`
    pub fn skip_whitespace(&mut self) {
        self.read_while_matching(&WHITESPACE);
    }
}

#[cfg(test)]
mod tests {
    use crate::compiler::parse::Parser;
    use regex::Regex;

    #[test]
    fn test_match_str() {
        let mut parser = Parser::new("rustle is awesome");
        assert!(parser.match_str("rustle"));
    }

    #[test]
    fn test_eat() {
        let mut parser = Parser::new("rustle is awesome");

        parser.eat("rustle");
        assert_eq!(parser.index, 6);

        parser.eat(" ");
        assert_eq!(parser.index, 7);

        parser.eat("is");
        assert_eq!(parser.index, 9);

        parser.eat(" ");
        assert_eq!(parser.index, 10);

        parser.eat("awesome");
        assert_eq!(parser.index, 17);
    }

    #[test]
    fn test_read_while_matching() {
        let mut parser = Parser::new("rustle is awesome");
        let regex = Regex::new("[a-z]").unwrap();

        let mut matched = parser.read_while_matching(&regex);
        assert_eq!(parser.index, 6);
        assert_eq!(matched, "rustle".to_string());

        parser.eat(" ");
        assert_eq!(parser.index, 7);

        matched = parser.read_while_matching(&regex);
        assert_eq!(parser.index, 9);
        assert_eq!(matched, "is".to_string());

        parser.eat(" ");
        assert_eq!(parser.index, 10);

        matched = parser.read_while_matching(&regex);
        assert_eq!(parser.index, 17);
        assert_eq!(matched, "awesome".to_string());
    }

    #[test]
    fn test_skip_whitespace() {
        let mut parser = Parser::new("      rustle");
        parser.skip_whitespace();
        assert_eq!(parser.index, 6);
    }
}
