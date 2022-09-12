use lazy_static::lazy_static;
use regex::Regex;

pub const GLOBALS: [&str; 62] = [
    "alert",
    "Array",
    "BigInt",
    "Boolean",
    "clearInterval",
    "clearTimeout",
    "confirm",
    "console",
    "Date",
    "decodeURI",
    "decodeURIComponent",
    "document",
    "Element",
    "encodeURI",
    "encodeURIComponent",
    "Error",
    "EvalError",
    "Event",
    "EventSource",
    "fetch",
    "FormData",
    "global",
    "globalThis",
    "history",
    "HTMLElement",
    "Infinity",
    "InternalError",
    "Intl",
    "isFinite",
    "isNaN",
    "JSON",
    "localStorage",
    "location",
    "Map",
    "Math",
    "NaN",
    "navigator",
    "Node",
    "Number",
    "Object",
    "parseFloat",
    "parseInt",
    "process",
    "Promise",
    "prompt",
    "RangeError",
    "ReferenceError",
    "RegExp",
    "sessionStorage",
    "Set",
    "setInterval",
    "setTimeout",
    "String",
    "SVGElement",
    "Symbol",
    "SyntaxError",
    "TypeError",
    "undefined",
    "URIError",
    "URL",
    "URLSearchParams",
    "window",
];

pub const RESERVED: [&str; 48] = [
    "arguments",
    "await",
    "break",
    "case",
    "catch",
    "class",
    "const",
    "continue",
    "debugger",
    "default",
    "delete",
    "do",
    "else",
    "enum",
    "eval",
    "export",
    "extends",
    "false",
    "finally",
    "for",
    "function",
    "if",
    "implements",
    "import",
    "in",
    "instanceof",
    "interface",
    "let",
    "new",
    "null",
    "package",
    "private",
    "protected",
    "public",
    "return",
    "static",
    "super",
    "switch",
    "this",
    "throw",
    "true",
    "try",
    "typeof",
    "var",
    "void",
    "while",
    "with",
    "yield",
];

// TODO: find a replacement for acorn => swc_ecma_parser?
pub fn is_valid(str: &str) -> bool {
    let i = 0;

    while i < str.len() {
        //let code = full_char_code_at(str, i);
    }

    todo!()
}

lazy_static! {
    static ref LETTERS_AND_NUMBERS: Regex = Regex::new("[^a-zA-Z0-9_]").unwrap();
    static ref STARTS_WITH_UNDERSCORE: Regex = Regex::new("^_").unwrap();
    static ref ENDS_WITH_UNDERSCORE: Regex = Regex::new("_$").unwrap();
    static ref STARTS_WITH_NUMBER: Regex = Regex::new("^[0-9]").unwrap();
}

pub fn sanitize(name: &str) -> String {
    LETTERS_AND_NUMBERS.replace(name, "_");
    STARTS_WITH_UNDERSCORE.replace(name, "");
    ENDS_WITH_UNDERSCORE.replace(name, "");
    STARTS_WITH_NUMBER.replace(name, "_$&").into_owned()
}

#[cfg(test)]
mod tests {
    use super::{
        ENDS_WITH_UNDERSCORE, LETTERS_AND_NUMBERS, STARTS_WITH_NUMBER, STARTS_WITH_UNDERSCORE,
    };

    #[test]
    fn test_leters_and_numbers_regex() {
        let samples = vec!["%", "#", "ž", " "];

        for s in samples {
            assert!(LETTERS_AND_NUMBERS.is_match(s));
        }
    }
}
