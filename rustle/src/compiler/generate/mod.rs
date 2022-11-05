use self::{
    add_lifecycle_call::add_lifecycle_calls,
    print_js::{generate_js_from_expr, generate_js_from_module_item},
};

use super::{analyse::AnalysisResult, expr_visitor::Visit, AttributeValue, Fragment, RustleAst};
use swc_ecma_ast::{Expr, ModuleDecl, ModuleItem};

mod add_lifecycle_call;
mod generate_css;
mod print_css;
mod print_js;

pub use generate_css::generate_css;
use print_js::generate_js_from_script;

struct Code {
    counter: usize,
    variables: Vec<String>,
    nested_components: Vec<String>,
    reactive_declarations: Vec<String>,
    imports: Vec<String>,
    create: Vec<String>,
    update: Vec<String>,
    destroy: Vec<String>,
}

/// Generates the javascript code from the AST and the analysis
pub fn generate_js(ast: &mut RustleAst, analysis: &AnalysisResult) -> String {
    let mut code = Code {
        counter: 1,
        variables: Vec::new(),
        nested_components: Vec::new(),
        reactive_declarations: Vec::new(),
        imports: Vec::new(),
        create: Vec::new(),
        update: Vec::new(),
        destroy: Vec::new(),
    };

    // checks what code to add into the final template
    for fragment in &ast.fragments {
        traverse(&fragment, "target".into(), &analysis, &mut code)
    }

    // add lifecycle calls to variables that will be updated
    let script = if let Some(script) = &mut ast.script {
        let updated_script = add_lifecycle_calls(script, &analysis.will_change);
        generate_js_from_script(updated_script)
    } else {
        String::new()
    };

    // turn the AST script back into String, to be inserted into the final template
    // TODO: the formatting of the generated js is not good, fix this somehow
    for rd in &analysis.reactive_declarations {
        // transforms a vec into a js array ["name1", "name2"]
        let change_json = format!("[\"{}\"]", rd.dependencies.join("\", \""));
        let assignee_json = format!("[\"{}\"]", rd.assignees.join("\", \""));
        code.reactive_declarations.push(
            format!(
                r#"        if ({}.some(name => collectChanges.includes(name))) {{
            {}
            update({});
        }}"#,
                change_json,
                generate_js_from_expr(&rd.node).trim_end(),
                assignee_json
            )
            .into(),
        );

        for asignee in &rd.assignees {
            code.variables.push(asignee.clone());
        }
    }

    // change import from "svelte" to "js" and add it to imports
    if let Some(ie) = &mut ast.imports_exports {
        for mi in ie {
            match mi {
                ModuleItem::ModuleDecl(md) => match md {
                    ModuleDecl::Import(i) => {
                        let mut name = i.src.value.to_string();
                        name = name.replace("svelte", "js");

                        i.src.value = name.into();
                        i.src.raw = None;

                        code.imports.push(generate_js_from_module_item(mi));
                    }
                    _ => (),
                },
                _ => (),
            }
        }
    }

    // the final javascript template
    format!(
        r#"{}
export default function() {{
{}{}
    let collectChanges = [];
    let updateCalled = false;
    function update(changed) {{
        changed.forEach((c) => collectChanges.push(c));
        if (updateCalled) return;
        updateCalled = true;

        updateReactiveDeclarations(collectChanges);
        if (typeof lifecycle !== "undefined") lifecycle.update(collectChanges);

        collectChanges = [];
        updateCalled = false;
    }}

{}

    update({});

    function updateReactiveDeclarations() {{
{}
    }}

    var lifecycle = {{
        create(target) {{
{}
        }},
        update(changed) {{
{}
        }},
        destroy() {{
{}
        }},
    }};
    return lifecycle;
}}"#,
        code.imports.join(""),
        code.variables
            .iter()
            .map(|v| format!("    let {};", v))
            .collect::<Vec<String>>()
            .join("\n"),
        code.nested_components.join("\n"),
        script,
        format!(
            "[\"{}\"]",
            analysis
                .will_change
                .clone()
                .into_iter()
                .collect::<Vec<String>>()
                .join("\", \"")
        ),
        code.reactive_declarations.join("\n"),
        code.create.join("\n"),
        code.update.join("\n"),
        code.destroy.join("\n")
    )
}

/// traverses a node and checks what sort of element to create or function to add
fn traverse(node: &Fragment, parent: String, analysis: &AnalysisResult, code: &mut Code) {
    match node {
        Fragment::Program(_) => (),
        // adds HTML elements like <h1>, <div>, <button>
        Fragment::Element(f) => {
            let variable_name = format!("{}_{}", f.name, code.counter);
            code.counter += 1;

            code.variables.push(variable_name.clone());
            code.create.push(format!(
                "            {} = document.createElement('{}');",
                variable_name, f.name
            ));

            // adds attributes on:click, class, style
            for attr in &f.attributes {
                // handles events
                if attr.name.starts_with("on:") {
                    let event_name = attr.name.chars().skip(3).collect::<String>();
                    let event_handler = match &attr.value {
                        AttributeValue::Expr(expr) => match expr {
                            Expr::Ident(ident) => ident.sym.to_string(),
                            _ => panic!("Unhandled event handler name"),
                        },
                        _ => panic!("Unhandled event handler name"),
                    };

                    code.create.push(format!(
                        "            {}.addEventListener('{}', {});",
                        variable_name, event_name, event_handler
                    ));

                    code.destroy.push(format!(
                        "            {}.removeEventListener('{}', {});",
                        variable_name, event_name, event_handler
                    ));
                }

                // handles attributes class, stye, disabled
                match &attr.value {
                    AttributeValue::String(value) => {
                        // add unique scope to attributes if it's a class
                        if attr.name == "class" {
                            code.create.push(format!(
                                "            {}.setAttribute('{}', '{} {}');",
                                variable_name, attr.name, value, analysis.css_unique_scope
                            ));
                        } else {
                            code.create.push(format!(
                                "            {}.setAttribute('{}', '{}');",
                                variable_name, attr.name, value
                            ));
                        }
                    }
                    _ => (),
                }
            }

            for fragment in &f.fragments {
                traverse(fragment, variable_name.clone(), analysis, code);
            }

            code.create.push(format!(
                "            {}.appendChild({});",
                parent, variable_name
            ));
            code.destroy.push(format!(
                "            {}.removeChild({});",
                parent, variable_name
            ));
        }

        // adds expressions inside curly braces as text nodes
        Fragment::Expression(f) => {
            let variable_name = format!("txt_{}", code.counter);
            code.counter += 1;

            let expression_name = generate_js_from_expr(f).replace([';', '\n'], "");

            code.variables.push(variable_name.clone());
            code.create.push(format!(
                "            {} = document.createTextNode({});",
                variable_name, expression_name
            ));

            code.create.push(format!(
                "            {}.appendChild({});",
                parent, variable_name
            ));

            let names = f.extract_names();

            // this is a mess
            if names.len() > 0 {
                let mut changes = Vec::new();
                for name in &names {
                    if analysis.will_change.contains(name) {
                        changes.push(name.as_str());
                    }
                }

                if changes.len() > 1 {
                    let names_json = format!("[\"{}\"]", changes.join("\", \""));

                    for name in names {
                        if analysis.will_change.contains(&name) {
                            code.update.push(format!(
                                r#"            if ({}.some(name => changed.includes(name))) {{
                {}.data = {};
            }}"#,
                                names_json, variable_name, expression_name
                            ));
                        }
                    }
                } else {
                    if analysis.will_change.contains(names.first().unwrap()) {
                        code.update.push(format!(
                            r#"            if (changed.includes("{}")) {{
                {}.data = {};
            }}"#,
                            changes.first().unwrap(),
                            variable_name,
                            expression_name
                        ));
                    }
                }
            }

            if analysis.will_change.contains(&expression_name) {}
        }

        // creates plain text nodes
        Fragment::Text(f) => {
            let variable_name = format!("txt_{}", code.counter);
            code.counter += 1;

            code.variables.push(variable_name.clone());
            code.create.push(format!(
                "            {} = document.createTextNode('{}');",
                variable_name.clone(),
                f.data.to_string().trim()
            ));
            code.create.push(format!(
                "            {}.appendChild({});",
                parent, variable_name
            ));
        }

        Fragment::Style(_) => (),
        Fragment::NestedComponent(nc) => {
            let variable_name = format!("{}_{}", nc.name.to_lowercase(), code.counter);
            code.counter += 1;

            code.nested_components
                .push(format!("    let {} = {}();", variable_name, nc.name));

            code.create
                .push(format!("            {}.create(target);", variable_name));
        }
    }
}
