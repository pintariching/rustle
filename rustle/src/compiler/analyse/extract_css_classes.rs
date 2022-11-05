use swc_css_ast::{
    ComplexSelectorChildren, QualifiedRulePrelude, Rule, Stylesheet, SubclassSelector,
};

pub fn extract_css_classes(stylesheet: &Stylesheet) -> Vec<String> {
    let mut buf = Vec::new();

    for rule in &stylesheet.rules {
        match rule {
            Rule::QualifiedRule(ql) => match &ql.prelude {
                QualifiedRulePrelude::SelectorList(sl) => {
                    for cs in &sl.children {
                        for cs in &cs.children {
                            match cs {
                                ComplexSelectorChildren::CompoundSelector(cs) => {
                                    for ss in &cs.subclass_selectors {
                                        match ss {
                                            SubclassSelector::Class(c) => {
                                                buf.push(c.text.value.to_string())
                                            }
                                            _ => panic!("{:#?}", ss),
                                        }
                                    }
                                }
                                ComplexSelectorChildren::Combinator(_) => panic!("{:#?}", ql),
                            }
                        }
                    }
                }
                QualifiedRulePrelude::ListOfComponentValues(_) => panic!("{:#?}", ql),
                QualifiedRulePrelude::RelativeSelectorList(_) => panic!("{:#?}", ql),
            },
            _ => panic!("CSS rule not supported: {:#?}", rule),
        }
    }

    buf
}
