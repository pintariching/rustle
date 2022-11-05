static HTML_TAGS: [&str; 6] = ["h1", "h2", "h3", "h4", "div", "button"];

pub fn is_html_tag(tag: &str) -> bool {
    HTML_TAGS.contains(&tag)
}
