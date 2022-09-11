use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
    static ref PATTERN: Regex = Regex::new(r"?m^\s*svelte-ignore\s+([\s\S]+)\s*").unwrap();
}

pub fn extract_svelte_ignore(text: &str) -> Vec<String> {
    let captures = PATTERN.captures(text);
    if let Some(c) = captures {
        return c
            .iter()
            .nth(1)
            .unwrap()
            .unwrap()
            .as_str()
            .split(" ")
            .map(|x| x.trim().to_string())
            .collect();
    }

    Vec::new()
}
