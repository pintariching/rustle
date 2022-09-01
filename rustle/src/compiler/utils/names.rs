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

pub fn sanitize(name: &str) -> String {
    let letters_and_numbers = Regex::new("/[^a-zA-Z0-9_]+/g").unwrap();
    let starts_with_underscore = Regex::new("/^_/").unwrap();
    let ends_with_underscore = Regex::new("/_$/").unwrap();
    let starts_with_number = Regex::new("/^[0-9]/").unwrap();

    letters_and_numbers.replace(name, "_");
    starts_with_underscore.replace(name, "");
    ends_with_underscore.replace(name, "");
    starts_with_number.replace(name, "_$&").into_owned()
}
