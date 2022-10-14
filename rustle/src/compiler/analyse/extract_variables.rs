use swc_ecma_ast::{BindingIdent, Decl, Script, VarDecl, VarDeclarator};

pub fn extract_root_variables(script: &Script) -> Vec<String> {
    let declarations = script
        .body
        .clone()
        .into_iter()
        .filter_map(|stmt| stmt.decl())
        .collect::<Vec<Decl>>();

    let var_declarations = declarations
        .into_iter()
        .filter_map(|decl| decl.var())
        .collect::<Vec<Box<VarDecl>>>();

    let var_declarators = var_declarations
        .into_iter()
        .map(|var_decls| var_decls.decls)
        .flatten()
        .collect::<Vec<VarDeclarator>>();

    let binding_idents = var_declarators
        .into_iter()
        .filter_map(|var_declrs| var_declrs.name.ident())
        .collect::<Vec<BindingIdent>>();

    let names = binding_idents
        .into_iter()
        .map(|ident| ident.id.sym.to_string())
        .collect::<Vec<String>>();

    names
}
