use crate::list;

#[derive(Debug)]
pub struct Error<'a> {
    pub code: &'a str,
    pub message: &'a str,
}

impl<'a> Error<'a> {
    fn new(code: &'a str, message: &'a str) -> Error {
        Error { code, message }
    }

    pub fn css_syntax_error(message: &'a str) -> Error {
        Error::new("css-syntax-error", message)
    }

    pub fn duplicate_attribute() -> Error {
        Error::new("duplicate-attribute", "Attributes need to be unique")
    }

    pub fn duplicate_element(slug: &'a str, name: &'a str) -> Error {
        Error::new(
            &format!("duplicate-{}", slug),
            &format!("A component can only have one <{}> tag", name),
        )
    }

    pub fn duplicate_style() -> Error {
        Error::new(
            "duplicate-style",
            "You can only have one top-level <style> tag per component",
        )
    }

    pub fn empty_attribute_shorthand() {
        Error::new(
            "empty-attribute-shorthand",
            "Attribute shorthand cannot be empty",
        )
    }

    pub fn empty_directive_name(directive_type: &'a str) -> Error {
        Error::new(
            "empty-directive-name",
            &format!("{} name cannot be empty", directive_type),
        )
    }

    pub fn empty_global_selector() -> Error {
        Error::new("css-syntax-error", ":global() must contain a selector")
    }

    pub fn expected_block_type() -> Error {
        Error::new("expected-block-type", "Expected if, each or await")
    }

    pub fn expected_name() -> Error {
        Error::new("expected-name", "Expected name")
    }

    // TODO: block is of type any - &str is a placeholder
    pub fn invalid_catch_placement_unclosed_block(block: &'a str) -> Error {
        Error::new(
            "invalid-catch-placement",
            &format!("Expected to close {} before seeing {{:catch}} block", block),
        )
    }

    pub fn invalid_catch_placement_without_await() -> Error {
        Error::new(
            "invalid-catch-placement",
            "Cannot have an {{:catch}} block outside an {{#await ...}} block",
        )
    }

    pub fn invalid_component_definition() -> Error {
        Error::new(
            "invalid-component-definition",
            "invalid component definition",
        )
    }

    pub fn invalid_closing_tag_unopened(name: &'a str) -> Error {
        Error::new(
            "invalid-closing-tag",
            &format!(
                "</{}> attempted to close an element that was not open",
                name
            ),
        )
    }

    pub fn invalid_closing_tag_autoclosed(name: &'a str, reason: &'a str) -> Error {
        Error::new(
            "invalid-closing-tag",
            &format!(
                "</{}> attempted to close <{}> that was already automatically closed by <{}>",
                name, name, reason
            ),
        )
    }

    pub fn invalid_debug_args() -> Error {
        Error::new(
            "invalid-debug-args",
            "{@debug ...} arguments must be identifiers, not arbitrary expressions",
        )
    }

    pub fn invalid_declaration() -> Error {
        Error::new("invalid-declaration", "Declaration cannot be empty")
    }

    pub fn invalid_directive_value() -> Error {
        Error::new(
            "invalid-directive-value",
            "Directive value must be a JavaScript expression enclosed in curly braces",
        )
    }

    pub fn invalid_elseif() -> Error {
        Error::new("invalid-elseif", "'elseif' should be 'else if'")
    }

    pub fn invalid_elseif_placement_outside_if() -> Error {
        Error::new(
            "invalid-elseif-placement",
            "Cannot have an {:else if ...} block outside an {#if ...} block",
        )
    }

    pub fn invalid_elseif_placement_unclosed_block(block: &'a str) -> Error {
        Error::new(
            "invalid-elseif-placement",
            &format!(
                "Expected to close {} before seeing {{:else if ...}} block",
                block
            ),
        )
    }

    pub fn invalid_else_placement_outside_if() -> Error {
        Error::new(
            "invalid-else-placement",
            "Cannot have an {:else} block outside an {#if ...} or {#each ...} block",
        )
    }

    pub fn invalid_else_placement_unclosed_block(block: &'a str) -> Error {
        Error::new(
            "invalid-else-placement",
            &format!("Expected to close {} before seeing {{:else}} block", block),
        )
    }

    pub fn invalid_element_content(slug: &'a str, name: &'a str) -> Error {
        Error::new(
            &format!("invalid-{}-content", slug),
            &format!("<{}> cannot have children", name),
        )
    }

    pub fn invalid_element_definition() -> Error {
        Error::new("invalid-element-definition", "Invalid element definition")
    }

    pub fn invalid_element_placement(slug: &'a str, name: &'a str) -> Error {
        Error::new(
            &format!("invalid-{}-placement", slug),
            &format!("<{}> tags cannot be inside elements or blocks", name),
        )
    }

    pub fn invalid_ref_directive(name: &'a str) -> Error {
        Error::new(
            "invalid-ref-directive",
            &format!(
                "The ref directive is no longer supported — use bind:this={{{}}} instead",
                name
            ),
        )
    }

    pub fn invalid_ref_selector() -> Error {
        Error::new(
            "invalid-ref-selector",
            "ref selectors are no longer supported",
        )
    }

    pub fn invalid_self_placement() -> Error {
        Error::new(
            "invalid-self-placement",
            "<svelte:self> components can only exist inside {#if} blocks, {#each} blocks, or slots passed to components",
        )
    }

    pub fn invalid_script_instance() -> Error {
        Error::new(
            "invalid-script",
            "A component can only have one instance-level <script> element",
        )
    }

    pub fn invalid_script_module() -> Error {
        Error::new(
            "invalid-script",
            "A component can only have one <script context=\"module\"> element",
        )
    }

    pub fn invalid_script_context_attribute() -> Error {
        Error::new("invalid-script", "context attribute must be static")
    }

    pub fn invalid_script_context_value() -> Error {
        Error::new(
            "invalid-script",
            "If the context attribute is supplied, its value must be \"module\"",
        )
    }

    pub fn invalid_tag_name() -> Error {
        Error::new("invalid-tag-name", "Expected valid tag name")
    }

    pub fn invalid_tag_name_svelte_element(tags: &'a [str], match_str: &'a str) -> Error {
        if match_str.len() > 0 {
            Error::new(
                "invalid-tag-name",
                &format!(
                    "Valid <svelte:...> tag names are {} (did you mean {}?",
                    list!(tags),
                    match_str
                ),
            )
        } else {
            Error::new(
                "invalid-tag-name",
                &format!("Valid <svelte:...> tag names are {}", list!(tags)),
            )
        }
    }

    pub fn invalid_then_placement_unclosed_block(block: &'a str) -> Error {
        Error::new(
            "invalid-then-placement",
            &format!("Expected to close {} before seeing {{:then}} block", block),
        )
    }

    pub fn invalid_then_placement_without_await() -> Error {
        Error::new(
            "invalid-then-placement",
            "Cannot have an {:then} block outside an {#await ...} block",
        )
    }

    pub fn invalid_void_content(name: &'a str) -> Error {
        Error::new(
            "invalid-void-content",
            &format!(
                "<{}> is a void element and cannot have children, or a closing tag",
                name
            ),
        )
    }

    pub fn missing_component_definition() -> Error {
        Error::new(
            "missing-component-definition",
            "<svelte:component> must have a 'this' attribute",
        )
    }

    pub fn missing_attribute_value() -> Error {
        Error::new(
            "missing-attribute-value",
            "Expected value for the attribute",
        )
    }

    pub fn missing_element_definition() -> Error {
        Error::new(
            "missing-element-definition",
            "<svelte:element> must have a 'this' attribute",
        )
    }

    pub fn unclosed_script() -> Error {
        Error::new("unclosed-script", "<script> must have a closing tag")
    }

    pub fn unclosed_style() -> Error {
        Error::new("unclosed-style", "<style> must have a closing tag")
    }

    pub fn unclosed_comment() -> Error {
        Error::new("unclosed-comment", "comment was left open, expected -->")
    }

    pub fn unclosed_attribute_value(token: &'a str) -> Error {
        Error::new(
            "unclosed-attribute-value",
            &format!("Expected to close the attribute value with {}", token),
        )
    }

    pub fn unexpected_block_close() -> Error {
        Error::new("unexpected-block-close", "Unexpected block closing tag")
    }

    pub fn unexpected_eof() -> Error {
        Error::new("unexpected-eof", "Unexpected end of input")
    }

    pub fn unexpected_eof_token(token: &'a str) -> Error {
        Error::new("unexpected-eof", &format!("Unexpected {}", token))
    }

    pub fn unexpected_token(token: &'a str) -> Error {
        Error::new("unexpected-token", &format!("Expected {}", token))
    }

    pub fn unexpected_token_destructure() -> Error {
        Error::new(
            "unexpected-token",
            "Expected identifier or destructure pattern",
        )
    }
}
