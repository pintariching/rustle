mod brackets;
mod entities;
pub mod html;
pub mod node;

//re-exports
pub use brackets::{get_bracket_close, is_bracket_close, is_bracket_open, is_bracket_pair};
pub use html::{closing_tag_omitted, decode_character_references, validate_code};
