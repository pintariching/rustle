use std::collections::HashMap;

// The `foreign` namespace covers all DOM implementations that aren"t HTML5.
// It opts out of HTML5-specific a11y checks and case-insensitive attribute names.
pub const FOREIGN: &str = "https://svelte.dev/docs#template-syntax-svelte-options";
pub const HTML: &str = "http://www.w3.org/1999/xhtml";
pub const MATHML: &str = "http://www.w3.org/1998/Math/MathML";
pub const SVG: &str = "http://www.w3.org/2000/svg";
pub const XLINK: &str = "http://www.w3.org/1999/xlink";
pub const XML: &str = "http://www.w3.org/XML/1998/namespace";
pub const XMLNS: &str = "http://www.w3.org/2000/xmlns";

pub const VALID_NAMESPACES: [&str; 14] = [
    "foreign", "html", "mathml", "svg", "xlink", "xml", "xmlns", FOREIGN, HTML, MATHML, SVG, XLINK,
    XML, XMLNS,
];

pub struct Namespaces {
    foreign: String,
    html: String,
    mathml: String,
    svg: String,
    xlink: String,
    xml: String,
    xmlns: String,
}
