use self::{
    add_lifecycle_call::add_lifecycle_calls,
    print_js::{generate_js_from_expr, generate_js_from_module_item},
};
use std::{
    collections::{hash_set::HashSet, HashMap},
    ffi::OsStr,
};

use super::{analyse::AnalysisResult, expr_visitor::Visit, AttributeValue, Fragment, RustleAst};
use swc_ecma_ast::{Decl, Expr, Lit, ModuleDecl, ModuleItem, Pat, Stmt};

mod add_lifecycle_call;
mod generate_css;
mod print_css;
mod print_js;

pub use generate_css::generate_css;
use indoc::indoc;
use print_js::generate_js_from_script;

struct Code {
    counter: usize,
    ctx_counter: usize,
    ctx_tracker: HashMap<String, usize>,
    variables: Vec<String>,
    c_variables: Vec<String>,
    variables_tracker: HashMap<String, String>,
    nested_components: Vec<String>,
    reactive_declarations: Vec<String>,
    imports: Vec<String>,
    internals_imports: HashSet<String>,
    create: Vec<String>,
    mount: Vec<String>,
    update: Vec<String>,
    destroy: Vec<String>,
}

/// Generates the javascript code from the AST and the analysis
pub fn generate_js(ast: &mut RustleAst, analysis: &AnalysisResult, input_name: &OsStr) -> String {
    let mut code = Code {
        counter: 1,
        ctx_counter: 0,
        ctx_tracker: HashMap::new(),
        variables: Vec::new(),
        c_variables: Vec::new(),
        variables_tracker: HashMap::new(),
        nested_components: Vec::new(),
        reactive_declarations: Vec::new(),
        imports: Vec::new(),
        internals_imports: HashSet::new(),
        create: Vec::new(),
        mount: Vec::new(),
        update: Vec::new(),
        destroy: Vec::new(),
    };

    //Variable declarations
    if let Some(script) = &mut ast.script {
        for stmt in &mut script.body {
            match stmt {
                Stmt::Decl(decl) => match decl {
                    Decl::Var(vd) => {
                        for v in &mut vd.decls {
                            let mut key = String::new();
                            let mut val = String::new();

                            if let Some(expr) = &mut v.init {
                                match &**expr {
                                    Expr::Lit(l) => match l {
                                        Lit::Str(s) => val = s.value.to_string(),
                                        Lit::Num(n) => val = n.raw.clone().unwrap().to_string(),
                                        Lit::Bool(b) => val = b.value.to_string(),
                                        _ => (),
                                    },
                                    _ => (),
                                }
                            }

                            match &v.name {
                                Pat::Ident(i) => key = i.id.sym.to_string(),
                                _ => panic!("{:#?}", decl),
                            }

                            if key.len() > 0 && val.len() > 0 {
                                code.variables_tracker.insert(key, val);
                            }
                        }
                    }
                    _ => (),
                },
                _ => (),
            }
        }
    }

    // change import from "svelte" to "js" and add it to imports
    if let Some(i) = &mut ast.imports {
        for mi in i {
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

    // generate exports
    if let Some(e) = &ast.exports {
        for mi in e {
            match mi {
                ModuleItem::ModuleDecl(md) => match md {
                    ModuleDecl::ExportDecl(ed) => match &ed.decl {
                        Decl::Var(vd) => {
                            for decl in &vd.decls {
                                match &decl.name {
                                    Pat::Ident(i) => {
                                        let export_name = i.sym.to_string();

                                        code.variables.push(export_name.clone());

                                        code.create.push(format!(
                                            "            {} = props.{};",
                                            export_name, export_name
                                        ));
                                    }
                                    _ => (),
                                }
                            }
                        }
                        _ => (),
                    },
                    _ => (),
                },
                _ => (),
            }
        }
    }

    // checks what code to add into the final template
    for fragment in &ast.fragments {
        traverse(&fragment, "target".into(), &analysis, &mut code, None)
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

    //Check for internal imports
    let mut import_statment = String::from("");

    if code.internals_imports.len() > 0 {
        import_statment = format!(
            indoc! {r#"
        import {{
            noop,
            SvelteComponent,
            {imports}
        }} from "svelte/internal"
        "#},
            imports = code
                .internals_imports
                .iter()
                .map(|v| format!("{},", v))
                .collect::<Vec<String>>()
                .join("\n    ")
        )
    }

    // New TODO - Finish hopefully being one-to-one with the svelte compiler
    format!(
        indoc! {r#"
        /* {source_name}.svelte generated by Rustle 0.1.5 */
        {imports_internal}

        function create_fragment(ctx) {{
            {variables}

            return {{
                c() {{
                    {create}
                }},
                m(target, anchor) {{
                    {mounts}
                }},
                p: noop,
                i: noop,
                o: noop,
                d(detaching) {{
                    {destroy}
                }}
            }};
        }}

        {constants}

        class {source_name} extends SvelteComponent {{
            constructor(options) {{
                super();
                init(this, options, null, create_fragment, safe_not_equal, {{}});
            }}
        }}

        export default {source_name}
    "#},
        source_name = input_name.to_str().unwrap(),
        imports_internal = import_statment,
        variables = code
            .variables
            .iter()
            .map(|v| format!("let {};", v))
            .collect::<Vec<String>>()
            .join("\n    "),
        create = code.create.join("\n            "),
        mounts = code.mount.join("\n            "),
        destroy = code.destroy.join("\n            "),
        constants = code.c_variables.join("\n")
    )
}

/// traverses a node and checks what sort of element to create or function to add
fn traverse(
    node: &Fragment,
    parent: String,
    analysis: &AnalysisResult,
    code: &mut Code,
    op: Option<&Fragment>,
) {
    match node {
        Fragment::Program(_) => (),
        // adds HTML elements like <h1>, <div>, <button>
        Fragment::Element(f) => {
            let variable_name = format!("{}_{}", f.name, code.counter);
            code.counter += 1;

            if parent == "target" {
                code.destroy
                    .push(format!("if (detaching) detach({});", variable_name));
                code.internals_imports.insert("detach".to_string());
            }

            code.variables.push(variable_name.clone());
            code.create
                .push(format!("{} = element('{}');", variable_name, f.name));

            code.mount
                .push(format!("insert(target, {}, anchor);", variable_name));

            code.internals_imports.insert("element".to_string());
            code.internals_imports.insert("insert".to_string());

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
                        "{}.addEventListener('{}', {});",
                        variable_name, event_name, event_handler
                    ));

                    code.destroy.push(format!(
                        "{}.removeEventListener('{}', {});",
                        variable_name, event_name, event_handler
                    ));
                }

                // handles attributes class, stye, disabled
                match &attr.value {
                    AttributeValue::String(value) => {
                        // add unique scope to attributes if it's a class
                        if attr.name == "class" {
                            code.create.push(format!(
                                r#"attr({},"{}","{}");"#,
                                variable_name, attr.name, value,
                            ));

                            code.internals_imports.insert("attr".to_string());
                        } else {
                            code.create.push(format!(
                                "{}.setAttribute('{}', '{}');",
                                variable_name, attr.name, value
                            ));
                        }
                    }
                    _ => (),
                }
            }

            let optimize = f.fragments.iter().find(|&x| match x {
                Fragment::Program(_) => false,
                Fragment::Element(_) => false,
                Fragment::Expression(f) => {
                    let expression_name = generate_js_from_expr(f).replace([';', '\n'], "");
                    analysis.will_change.contains(&expression_name)
                }
                Fragment::Style(_) => false,
                Fragment::NestedComponent(_) => false,
                Fragment::Text(_) => false,
            });

            for fragment in &f.fragments {
                traverse(fragment, variable_name.clone(), analysis, code, optimize);
            }

            // code.create.push(format!(
            //     "{}.appendChild({});",
            //     parent, variable_name
            // ));
            // code.destroy.push(format!(
            //     "{}.removeChild({});",
            //     parent, variable_name
            // ));
        }

        // adds expressions inside curly braces as text nodes
        Fragment::Expression(f) => {
            let variable_name = format!("txt_{}", code.counter);
            code.counter += 1;

            if parent == "target" {
                code.destroy
                    .push(format!("if (detaching) detach({});", variable_name));
                code.internals_imports.insert("detach".to_string());
            }

            let expression_name = generate_js_from_expr(f).replace([';', '\n'], "");

            if let None = op {
                code.create.push(format!(
                    r#"{}.textContent += `${{{}}}`;"#,
                    parent, expression_name
                ));
            } else {
                code.variables.push(variable_name.clone());

                if analysis.will_change.contains(&expression_name) {
                    let current_ctx = code.ctx_tracker.get(&expression_name);

                    match current_ctx {
                        Some(i) => {
                            code.create.push(format!(
                                "{} = text(/* {} */ ctx[{}]);",
                                variable_name, expression_name, i
                            ));
                        }
                        None => {
                            code.ctx_tracker
                                .insert(expression_name.clone(), code.ctx_counter);

                            code.create.push(format!(
                                "{} = text(/* {} */ ctx[{}]);",
                                variable_name, expression_name, code.ctx_counter
                            ));

                            code.ctx_counter += 1;
                        }
                    }
                } else {
                    code.create
                        .push(format!("{} = text({});", variable_name, expression_name,));

                    let value = code.variables_tracker.get(&expression_name).unwrap();

                    code.c_variables
                        .push(format!("let {} = {};", expression_name, value));
                }

                code.internals_imports.insert("text".to_string());

                code.mount
                    .push(format!("append({}, {});", parent, variable_name));

                code.internals_imports.insert("append".to_string());
            }

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
                {}.data = props ? props.{} : {};
            }}"#,
                            changes.first().unwrap(),
                            variable_name,
                            expression_name,
                            expression_name
                        ));
                    }
                }
            }
        }

        // creates plain text nodes
        Fragment::Text(f) => {
            let variable_name = format!("txt_{}", code.counter);
            code.counter += 1;

            if parent == "target" {
                code.destroy
                    .push(format!("if (detaching) detach({});", variable_name));
                code.internals_imports.insert("detach".to_string());
            }

            if let None = op {
                if parent == "target" {
                    code.variables.push(variable_name.clone());

                    code.create.push(format!(
                        r#"{} = text("{}");"#,
                        variable_name.clone(),
                        f.data.to_string().trim()
                    ));

                    code.internals_imports.insert("text".to_string());

                    code.mount
                        .push(format!("insert(target, {}, anchor);", variable_name));

                    code.internals_imports.insert("insert".to_string());
                } else {
                    code.create.push(format!(
                        r#"{}.textContent += "{}";"#,
                        parent,
                        f.data.to_string()
                    ));
                }
            } else {
                code.variables.push(variable_name.clone());

                code.create.push(format!(
                    r#"{} = text("{}");"#,
                    variable_name.clone(),
                    f.data.to_string()
                ));

                code.internals_imports.insert("text".to_string());

                if parent.len() > 0 {
                    code.mount
                        .push(format!("append({}, {});", parent, variable_name));

                    code.internals_imports.insert("append".to_string());
                } else {
                    code.mount
                        .push(format!("insert(target, {}, anchor);", variable_name));

                    code.internals_imports.insert("insert".to_string());
                }
            }
        }

        Fragment::Style(_) => (),
        Fragment::NestedComponent(nc) => {
            let variable_name = format!("{}_{}", nc.name.to_lowercase(), code.counter);
            code.counter += 1;

            if parent == "target" {
                code.destroy
                    .push(format!("if (detaching) detach({});", variable_name));
                code.internals_imports.insert("detach".to_string());
            }

            let mut attrs = Vec::new();
            for attr in &nc.attributes {
                let attr_value = match &attr.value {
                    AttributeValue::Expr(e) => generate_js_from_expr(e).replace([';', '\n'], ""),
                    AttributeValue::String(s) => s.clone(),
                };

                let current_attr = format!("{}: {}", attr.name, attr_value);
                attrs.push(current_attr.clone());

                code.update.push(format!(
                    r#"            if (changed.includes("{}")) {{
                {}.update("{}", {{ {} }});
}}"#,
                    attr_value, variable_name, attr.name, current_attr
                ));
            }

            code.nested_components
                .push(format!("    let {} = {}();", variable_name, nc.name));

            code.create.push(format!(
                "            {}.create(target, {{ {} }});",
                variable_name,
                attrs.join(", ")
            ));
        }
    }
}
