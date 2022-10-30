use self::{add_lifecycle_call::add_lifecycle_calls, print_js::generate_js_from_expr};

use super::{analyse::AnalysisResult, expr_visitor::Visit, AttributeValue, Fragment, RustleAst};
use swc_ecma_ast::Expr;

mod add_lifecycle_call;
mod print_js;
use print_js::generate_js_from_script;

struct Code {
    counter: usize,
    variables: Vec<String>,
    reactive_declarations: Vec<String>,
    create: Vec<String>,
    update: Vec<String>,
    destroy: Vec<String>,
}

pub fn generate(ast: RustleAst, analysis: AnalysisResult) -> String {
    let mut code = Code {
        counter: 1,
        variables: Vec::new(),
        reactive_declarations: Vec::new(),
        create: Vec::new(),
        update: Vec::new(),
        destroy: Vec::new(),
    };

    for fragment in ast.fragments {
        traverse(&fragment, "target".into(), &analysis, &mut code)
    }

    let updated_script = add_lifecycle_calls(ast.script, &analysis.will_change);
    let script = generate_js_from_script(updated_script);

    for rd in analysis.reactive_declarations {
        let change_json = format!("[\"{}\"]", rd.dependencies.join("\", \""));
        let assignee_json = format!("[\"{}\"]", rd.assignees.join("\", \""));
        code.reactive_declarations.push(
            format!(
                r#"
    if ({}.some(name => collectChanges.includes(name))) {{
        {}
        update({});
    }}
"#,
                change_json,
                generate_js_from_expr(&rd.node).trim_end(),
                assignee_json
            )
            .into(),
        );

        for asignee in rd.assignees {
            code.variables.push(asignee);
        }
    }

    format!(
        r#"
export default function() {{
    {}

    let collectChanges = [];
    let updateCalled = false;
    function update(changed) {{
        changed.forEach(c => collectChanges.push(c));
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
}}
    "#,
        code.variables
            .iter()
            .map(|v| format!("let {};", v))
            .collect::<Vec<String>>()
            .join("\n"),
        script,
        format!(
            "[\"{}\"]",
            analysis
                .will_change
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

fn traverse(node: &Fragment, parent: String, analysis: &AnalysisResult, code: &mut Code) {
    match node {
        Fragment::Script(_) => (),
        Fragment::Element(f) => {
            let variable_name = format!("{}_{}", f.name, code.counter);
            code.counter += 1;

            code.variables.push(variable_name.clone());
            code.create.push(format!(
                "{} = document.createElement('{}');",
                variable_name, f.name
            ));

            for attr in &f.attributes {
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
                        "{}.addEventListener('{}', {});",
                        variable_name, event_name, event_handler
                    ));

                    code.destroy.push(format!(
                        "{}.removeEventListener('{}', {});",
                        variable_name, event_name, event_handler
                    ));
                }

                match &attr.value {
                    AttributeValue::String(str) => code.create.push(format!(
                        "{}.setAttribute('{}', '{}');",
                        variable_name, attr.name, str
                    )),
                    _ => (),
                }
            }

            for fragment in &f.fragments {
                traverse(fragment, variable_name.clone(), analysis, code);
            }

            code.create
                .push(format!("{}.appendChild({});", parent, variable_name));
            code.destroy
                .push(format!("{}.removeChild({});", parent, variable_name));
        }
        Fragment::Expression(f) => {
            let variable_name = format!("txt_{}", code.counter);
            code.counter += 1;

            let expression_name = generate_js_from_expr(f).replace([';', '\n'], "");

            code.variables.push(variable_name.clone());
            code.create.push(format!(
                "{} = document.createTextNode({});",
                variable_name, expression_name
            ));

            code.create
                .push(format!("{}.appendChild({});", parent, variable_name));

            let names = f.extract_names();
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
                                r#"
if ({}.some(name => changed.includes(name))) {{
    {}.data = {};
}}
    "#,
                                names_json, variable_name, expression_name
                            ));
                        }
                    }
                } else {
                    if analysis.will_change.contains(names.first().unwrap()) {
                        code.update.push(format!(
                            r#"
if (changed.includes("{}")) {{
    {}.data = {};
}}
    "#,
                            changes.first().unwrap(),
                            variable_name,
                            expression_name
                        ));
                    }
                }
            }

            if analysis.will_change.contains(&expression_name) {}
        }
        Fragment::Text(f) => {
            let variable_name = format!("txt_{}", code.counter);
            code.counter += 1;

            code.variables.push(variable_name.clone());
            code.create.push(format!(
                "{} = document.createTextNode('{}');",
                variable_name.clone(),
                f.data.to_string().trim()
            ));
            code.create
                .push(format!("{}.appendChild({});", parent, variable_name));
        }
    }
}
