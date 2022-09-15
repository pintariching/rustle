use std::collections::HashMap;

use lazy_static::lazy_static;
use regex::{Captures, Regex};

use crate::compiler::parse::utils::entities::{self, ENTITY};

lazy_static! {
    static ref ENTITY_PATTERN: Regex = Regex::new(
        format!(
            "&(#?(?:x[\\w\\d]+|\\d+|{}))(?:;|b)",
            ENTITY::aggregate_to_string()
        )
        .as_str()
    )
    .unwrap();

    static ref DISSALOWED_CONTENTS: HashMap<&'static str, Vec<&'static str>> = HashMap::from([
        ("li", vec!["li"]),
        ("dt", vec!["dt", "dd"]),
        ("dd", vec!["dt", "dt"]),
        ("p", "address article aside blockquote div dl fieldset footer form h1 h2 h3 h4 h5 h6 header hgroup hr main menu nav ol p pre section table ul".split(" ").collect::<Vec<&str>>()),
        ("rt", vec!["rt", "rp"]),
        ("rp", vec!["rt", "rp"]),
        ("optgroup", vec!["optgroup"]),
        ("option", vec!["option", "optgroup"]),
        ("thead", vec!["tbody", "tfoot"]),
        ("tbody", vec!["tbody", "tfoot"]),
        ("tfoot", vec!["tbody"]),
        ("tr", vec!["tr", "tbody"]),
        ("td", vec!["td", "th", "tr"]),
        ("th", vec!["td", "th", "tr"]),
]);
}

static WINDOWS_1252: [u32; 32] = [
    8364, 129, 8218, 402, 8222, 8230, 8224, 8225, 710, 8240, 352, 8249, 338, 141, 381, 143, 144,
    8216, 8217, 8220, 8221, 8226, 8211, 8212, 732, 8482, 353, 8250, 339, 157, 382, 376,
];

/// Takes a character in a hex code, html code or as a html entity,
/// replace illegal code points with alternatives in some cases
/// and returns the character as a symbol in string form.
///
/// # Arguments
///
/// * `html` - The character as a hex code, html code or as a html entity
///
/// # Examples
/// ```
/// use rustle::compiler::parse::utils::html::decode_character_references;
/// assert_eq!(decode_character_references("&#x40;"), "@");
/// assert_eq!(decode_character_references("&#64;"), "@");
/// assert_eq!(decode_character_references("&commat;"), "@");
/// ```
///
pub fn decode_character_references(html: &str) -> String {
    let s = ENTITY_PATTERN.replace_all(&html, |cap: &Captures| {
        let mut code: Option<u32> = None;
        let mat = cap.get(0).unwrap().as_str();
        let entity = cap.get(1).unwrap().as_str();

        if entity.chars().nth(0).unwrap() != '#' {
            code = Some(ENTITY.get(entity).unwrap().clone());
        } else if entity.chars().nth(1).unwrap() == 'x' {
            code = Some(u32::from_str_radix(&entity[2..], 16).unwrap());
        } else {
            code = Some(u32::from_str_radix(&entity[1..], 10).unwrap());
        }

        if let Some(c) = code {
            return char::from_u32(validate_code(c).unwrap())
                .unwrap()
                .to_string();
        } else {
            return mat.to_string();
        }
    });

    s.into_owned()
}

// some code points are verboten. If we were inserting HTML, the browser would replace the illegal
// code points with alternatives in some cases - since we're bypassing that mechanism, we need
// to replace them ourselves
//
// Source: http://en.wikipedia.org/wiki/Character_encodings_in_HTML#Illegal_characters
pub fn validate_code(code: u32) -> Option<u32> {
    // line feed becomes generic whitespace
    if code == 10 {
        return Some(32);
    }

    // ASCII range. (Why someone would use HTML entities for ASCII characters I don't know, but...)
    if code < 128 {
        return Some(code);
    }

    // code points 128-159 are dealt with leniently by browsers, but they're incorrect. We need
    // to correct the mistake or we'll end up with missing € signs and so on
    if code <= 159 {
        return Some(WINDOWS_1252[(code - 128) as usize]);
    }

    // basic multilingual plane
    if code < 55269 {
        return Some(code);
    }

    // UTF-16 surrogate halves
    if code < 57343 {
        return None;
    }

    // rest of the basic multilingual plane
    if code <= 65535 {
        return Some(code);
    }

    // supplementary multilingual plane 0x10000 - 0x1ffff
    if code >= 65536 && code <= 131071 {
        return Some(code);
    }

    // supplementary ideographic plane 0x20000 - 0x2ffff
    if code >= 131072 && code <= 196607 {
        return Some(code);
    }

    None
}

pub fn closing_tag_omitted(current: &str, next: Option<&str>) -> bool {
    if DISSALOWED_CONTENTS.contains_key(current) {
        if let Some(next) = next {
            if DISSALOWED_CONTENTS.get(current).unwrap().contains(&next) {
                return true;
            }
        } else {
            return true;
        }
    }

    false
}

#[cfg(test)]
mod tests {
    use super::closing_tag_omitted;
    use super::decode_character_references;
    use super::ENTITY_PATTERN;

    #[test]
    fn test_closing_tag_omitted() {
        let current = "li";
        let next = "li";
        assert!(closing_tag_omitted(current, Some(next)));

        let current = "li";
        let next = "p";
        assert!(!closing_tag_omitted(current, Some(next)));

        let current = "p";
        let next = "address";
        assert!(closing_tag_omitted(current, Some(next)));

        let current = "tr";
        assert!(closing_tag_omitted(current, None));
    }

    #[test]
    fn test_entity_pattern_regex() {
        let samples = vec!["&CounterClockwiseContourIntegral;", "&eDDot;", "&duhar;"];

        for s in samples {
            assert!(ENTITY_PATTERN.is_match(s));
        }
    }

    #[test]
    fn test_decode_character_references() {
        assert_eq!(decode_character_references("&#10607;"), "⥯");
        assert_eq!(decode_character_references("&bigodot;"), "⨀");
        assert_eq!(decode_character_references("&#xb6;"), "¶");
    }
}
