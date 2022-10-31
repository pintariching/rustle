use std::collections::HashSet;

use swc_common::Span;
use swc_css_ast::{
    ClassSelector, ComplexSelectorChildren, Ident, QualifiedRulePrelude, Rule, Stylesheet,
    SubclassSelector,
};
use swc_ecma_ast::Class;

use crate::compiler::{analyse::AnalysisResult, RustleAst};

use super::print_css::generate_css_from_stylesheet;

pub fn generate_css(ast: &mut RustleAst, analysis: &AnalysisResult) -> String {
    if let Some(stylesheet) = &mut ast.style {
        remove_unused_css(stylesheet, &analysis.css_classes_in_template);
        add_unique_scope(stylesheet, &analysis.css_unique_scope);
        generate_css_from_stylesheet(stylesheet)
    } else {
        String::new()
    }
}

fn remove_unused_css(stylesheet: &mut Stylesheet, used_classes: &HashSet<String>) {
    stylesheet.rules.retain(|r| match r {
        Rule::QualifiedRule(ql) => match &ql.prelude {
            QualifiedRulePrelude::SelectorList(sl) => sl.children.iter().any(|cs| {
                cs.children.iter().any(|csc| match csc {
                    ComplexSelectorChildren::CompoundSelector(cs) => {
                        cs.subclass_selectors.iter().any(|ss| match ss {
                            SubclassSelector::Class(c) => {
                                let class = c.text.value.to_string();

                                if used_classes.contains(&class) {
                                    true
                                } else {
                                    false
                                }
                            }
                            _ => panic!("{:#?}", ss),
                        })
                    }
                    ComplexSelectorChildren::Combinator(c) => panic!("{:#?}", c),
                })
            }),
            QualifiedRulePrelude::ListOfComponentValues(_) => panic!("{:#?}", ql),
        },
        _ => panic!("CSS rule not supported: {:#?}", r),
    })
}

fn add_unique_scope(stylesheet: &mut Stylesheet, scope: &str) {
    for rule in &mut stylesheet.rules {
        match rule {
            Rule::QualifiedRule(ql) => match &mut ql.prelude {
                QualifiedRulePrelude::SelectorList(sl) => {
                    for cs in &mut sl.children {
                        for cs in &mut cs.children {
                            match cs {
                                ComplexSelectorChildren::CompoundSelector(cs) => {
                                    let scope_selector = SubclassSelector::Class(ClassSelector {
                                        span: Span::default(),
                                        text: Ident {
                                            span: Span::default(),
                                            value: scope.into(),
                                            raw: Some(scope.into()),
                                        },
                                    });
                                    cs.subclass_selectors.push(scope_selector);
                                }
                                ComplexSelectorChildren::Combinator(_) => panic!("{:#?}", ql),
                            }
                        }
                    }
                }
                QualifiedRulePrelude::ListOfComponentValues(_) => panic!("{:#?}", ql),
            },
            _ => panic!("CSS rule not supported: {:#?}", rule),
        }
    }
}
