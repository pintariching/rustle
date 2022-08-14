use std::collections::HashMap;

use lazy_static::lazy_static;
use regex::{Captures, Regex};

use crate::compiler::parse::utils::entities::Entity;

lazy_static! {
    static ref ENTITY_PATTERN: Regex = Regex::new(
        format!(
            "&(#?(?:x[\\w\\d]+|\\d+|{}))(?:;|\\b)",
            Entity::aggregate_to_string()
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

static windows_1252: [i32; 32] = [
    8364, 129, 8218, 402, 8222, 8230, 8224, 8225, 710, 8240, 352, 8249, 338, 141, 381, 143, 144,
    8216, 8217, 8220, 8221, 8226, 8211, 8212, 732, 8482, 353, 8250, 339, 157, 382, 376,
];

// TODO: fix this
pub fn decode_character_references(html: &str) -> String {
    // ENTITY_PATTERN
    //     .replace(html, |captures: &Captures| {
    //         let code: i32;

    //         if &captures[0] != "#" {
    //             code = Entity::from_code(code)
    //         }
    //     })
    //     .into_owned()

    todo!()
}

// some code points are verboten. If we were inserting HTML, the browser would replace the illegal
// code points with alternatives in some cases - since we're bypassing that mechanism, we need
// to replace them ourselves
//
// Source: http://en.wikipedia.org/wiki/Character_encodings_in_HTML#Illegal_characters
pub fn validate_code(entity: Entity) -> Option<Entity> {
    let code = entity.to_code();

    // line feed becomes generic whitespace
    if code == 10 {
        return Some(Entity::from_code(32));
    }

    // ASCII range. (Why someone would use HTML entities for ASCII characters I don't know, but...)
    if code < 128 {
        return Some(entity);
    }

    // code points 128-159 are dealt with leniently by browsers, but they're incorrect. We need
    // to correct the mistake or we'll end up with missing € signs and so on
    if code <= 159 {
        return Some(Entity::from_code(windows_1252[(code - 128) as usize]));
    }

    // basic multilingual plane
    if code < 55269 {
        return Some(entity);
    }

    // UTF-16 surrogate halves
    if code < 57343 {
        return None;
    }

    // rest of the basic multilingual plane
    if code <= 65535 {
        return Some(entity);
    }

    // supplementary multilingual plane 0x10000 - 0x1ffff
    if code >= 65536 && code <= 131071 {
        return Some(entity);
    }

    // supplementary ideographic plane 0x20000 - 0x2ffff
    if code >= 131072 && code <= 196607 {
        return Some(entity);
    }

    None
}

// can this be a child of the parent element, or does it implicitly
// close it, like `<li>one<li>two`?
pub fn closing_tag_ommited(current: &str, next: Option<&str>) -> bool {
    if DISSALOWED_CONTENTS.contains_key(current) {
        if let Some(next) = next {
            if let Some(contents) = DISSALOWED_CONTENTS.get(current) {
                if contents.contains(&next) {
                    return true;
                }
            }
        }
    }

    false
}
