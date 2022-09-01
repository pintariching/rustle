use swc_estree_ast::{
    CatchClause, Class, ClassBody, Expression, Function, Identifier, Literal, ModuleDeclaration,
    ModuleSpecifier, Pattern, Program, Property, SpreadElement, Statement, Super, SwitchCase,
    TemplateElement, VariableDeclarator,
};

// The commented out enums are missing from swc_estree_ast
// I'm not sure if they're used and where to find them
#[derive(Clone, Debug)]
pub enum Node {
    //AssignmentProperty(AssignmentProperty),
    CatchClause(CatchClause),
    Class(Class),
    ClassBody(ClassBody),
    Expression(Expression),
    Function(Function),
    Identifier(Identifier),
    Literal(Literal),
    //MethodDefinition(MethodDefinition),
    ModuleDeclaration(ModuleDeclaration),
    ModuleSpecifier(ModuleSpecifier),
    Pattern(Pattern),
    //PrivateIdentifier(PrivateIdentifier),
    Program(Program),
    Property(Property),
    //PropertyDefinition(PropertyDefinition),
    SpreadElement(SpreadElement),
    Statement(Statement),
    Super(Super),
    SwitchCase(SwitchCase),
    TemplateElement(TemplateElement),
    VariableDeclarator(VariableDeclarator),
    //Testing purpose
    Empty,
}
