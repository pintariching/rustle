mod brackets;
mod entities;
mod html;
mod node;

//re-exports
pub use brackets::{get_bracket_close, is_bracket_close, is_bracket_open, is_bracket_pair};
pub use html::{closing_tag_ommited, decode_character_references, validate_code};
