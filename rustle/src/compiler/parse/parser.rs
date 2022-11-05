use lazy_static::lazy_static;
use regex::Regex;
use swc_common::Span;
use swc_css_ast::Stylesheet;
use swc_ecma_ast::{ModuleItem, Program, Script};

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
    /// use rustle::compiler::Parser;
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

        let mut script: Option<Script> = None;
        let mut imports_exports: Option<Vec<ModuleItem>> = None;
        let program_index = fragments.iter().position(|f| match f {
            Fragment::Program(_) => true,
            _ => false,
        });

        if let Some(i) = program_index {
            if let Fragment::Program(p) = fragments.remove(i) {
                // split import and exports from the other statements
                match p {
                    Program::Module(m) => {
                        let mut stmts = Vec::new();
                        let mut imps_exps = Vec::new();

                        for mi in m.body {
                            match mi {
                                ModuleItem::ModuleDecl(_) => imps_exps.push(mi),
                                ModuleItem::Stmt(s) => stmts.push(s),
                            }
                        }

                        script = Some(Script {
                            span: Span::default(),
                            body: stmts,
                            shebang: None,
                        });

                        imports_exports = Some(imps_exps);
                    }
                    Program::Script(s) => script = Some(s),
                }
            };
        }

        let mut style: Option<Stylesheet> = None;
        let style_index = fragments.iter().position(|f| match f {
            Fragment::Style(_) => true,
            _ => false,
        });

        if let Some(i) = style_index {
            if let Fragment::Style(s) = fragments.remove(i) {
                style = Some(s)
            } else {
                style = None
            };
        }

        RustleAst {
            script,
            imports_exports,
            style,
            fragments,
            is_component: false,
        }
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
    /// use rustle::compiler::Parser;
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

    /// Checks if the character at the current index
    /// matches the provided char
    ///
    /// # Arguments
    ///
    /// * `char_to_match` - The char to match
    ///
    /// # Examples
    ///
    /// ```
    /// use rustle::compiler::Parser;
    ///
    /// let mut parser = Parser::new("rustle is awesome");
    /// assert!(parser.match_next_char('r'));
    ///
    /// parser.index = 10;
    /// assert!(parser.match_next_char('a'));
    /// ```
    pub fn match_next_char(&mut self, char_to_match: char) -> bool {
        let char = self.content.chars().nth(self.index);

        if let Some(c) = char {
            return c == char_to_match;
        } else {
            return false;
        }
    }

    /// Checks if the character at the current index
    /// matches the provided array of chars
    ///
    /// # Arguments
    ///
    /// * `chars_to_match` - The array of chars to match
    ///
    /// # Examples
    ///
    /// ```
    /// use rustle::compiler::Parser;
    ///
    /// let mut parser = Parser::new("rustle is awesome");
    /// assert!(parser.match_next_chars(&['a', 'b', 'r']));
    ///
    /// parser.index = 10;
    /// assert!(parser.match_next_chars(&['a']));
    /// ```
    pub fn match_next_chars(&mut self, chars_to_match: &[char]) -> bool {
        let char = self.content.chars().nth(self.index);

        if let Some(c) = char {
            for char_to_match in chars_to_match {
                if c == *char_to_match {
                    return true;
                }
            }

            return false;
        } else {
            return false;
        }
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
    /// use rustle::compiler::Parser;
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
    /// use rustle::compiler::Parser;
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
                .unwrap_or_else(|| panic!("{self:#?}"))
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
    use crate::compiler::Parser;
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
