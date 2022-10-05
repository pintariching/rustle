use super::{analyse::AnalysisResult, Fragment, RustleAst};
use swc::{config::SourceMapsConfig, Compiler};
use swc_common::collections::AHashMap;
use swc_ecma_ast::{EsVersion, Expr};

struct Code {
    counter: usize,
    variables: Vec<String>,
    create: Vec<String>,
    update: Vec<String>,
    destroy: Vec<String>,
}

pub fn generate(ast: RustleAst, analysis: AnalysisResult) -> String {
    let mut code = Code {
        counter: 1,
        variables: Vec::new(),
        create: Vec::new(),
        update: Vec::new(),
        destroy: Vec::new(),
    };

    for fragment in ast.fragments {
        traverse(&fragment, "target".into(), &analysis, &mut code)
    }

    let compiler = Compiler::new(Default::default());
    let script = compiler
        .print(
            &ast.script,
            None,
            None,
            false,
            EsVersion::Es2022,
            SourceMapsConfig::Bool(false),
            &AHashMap::default(),
            None,
            false,
            None,
            false,
            false,
        )
        .unwrap()
        .code;

    format!(
        r#"
	export default function() {{
		{}
		{}
		const lifecycle = {{
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
        script,
        code.variables
            .iter()
            .map(|v| format!("let {};", v))
            .collect::<Vec<String>>()
            .join("\n"),
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
                        Expr::Ident(ident) => ident.sym.to_string(),
                        _ => panic!(),
                    };

                    code.create.push(format!(
                        "{}.addEventListener('{}', {});",
                        parent, event_name, event_handler
                    ));

                    code.destroy.push(format!(
                        "{}.removeEventListener('{}', {});",
                        parent, event_name, event_handler
                    ));
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

            let expression_name = match f {
                Expr::Ident(ident) => ident.sym.to_string(),
                _ => panic!(),
            };

            code.variables.push(variable_name.clone());
            code.create.push(format!(
                "{} = document.createTextNode({});",
                variable_name, expression_name
            ));

            code.create
                .push(format!("{}.appendChild({});", parent, variable_name));

            if analysis.will_change.contains(&expression_name) {
                code.update.push(format!(
                    r#"
					if (changed.includes('{}')) {{
						{}.data = {};
					}}
				"#,
                    expression_name, variable_name, expression_name
                ));
            }
        }
        Fragment::Text(f) => {
            let variable_name = format!("txt_{}", code.counter);
            code.counter += 1;

            code.variables.push(variable_name.clone());
            code.create.push(format!(
                "{} = document.createTextNode('{}');",
                variable_name.clone(),
                f.data.to_string()
            ));
            code.create
                .push(format!("{}.appendChild({});", parent, variable_name));
        }
    }
}
