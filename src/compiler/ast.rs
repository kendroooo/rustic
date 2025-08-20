use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Program {
    pub items: Vec<Item>,
    pub imports: Vec<Import>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Import {
    pub module_path: String,
    pub span: crate::diagnostics::Span,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Item {
    Function(Function),
    Struct(Struct),
    Variable(Variable),
    Constant(Constant),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Function {
    pub name: String,
    pub parameters: Vec<Parameter>,
    pub return_type: Type,
    pub body: Block,
    pub span: crate::diagnostics::Span,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Struct {
    pub name: String,
    pub fields: Vec<Field>,
    pub span: crate::diagnostics::Span,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Field {
    pub name: String,
    pub field_type: Type,
    pub span: crate::diagnostics::Span,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Parameter {
    pub name: String,
    pub param_type: Type,
    pub default_value: Option<Expression>,
    pub span: crate::diagnostics::Span,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Variable {
    pub name: String,
    pub var_type: Type,
    pub initializer: Expression,
    pub mutable: bool,
    pub span: crate::diagnostics::Span,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Constant {
    pub name: String,
    pub const_type: Type,
    pub value: Expression,
    pub span: crate::diagnostics::Span,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Type {
    Int,
    Float,
    Str,
    Bool,
    List(Box<Type>),
    Struct(String),
    Void,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Block {
    pub statements: Vec<Statement>,
    pub span: crate::diagnostics::Span,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Statement {
    Expression(Expression),
    Variable(Variable),
    Assignment(Assignment),
    If(IfStatement),
    For(ForLoop),
    Try(TryStatement),
    Return(ReturnStatement),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Assignment {
    pub target: Expression,
    pub value: Expression,
    pub span: crate::diagnostics::Span,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct IfStatement {
    pub condition: Expression,
    pub then_block: Block,
    pub else_ifs: Vec<(Expression, Block)>,
    pub else_block: Option<Block>,
    pub span: crate::diagnostics::Span,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ForLoop {
    pub variable: String,
    pub iterable: Expression,
    pub body: Block,
    pub span: crate::diagnostics::Span,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TryStatement {
    pub try_block: Block,
    pub catch_clauses: Vec<CatchClause>,
    pub span: crate::diagnostics::Span,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CatchClause {
    pub exception_type: String,
    pub handler_block: Block,
    pub span: crate::diagnostics::Span,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ReturnStatement {
    pub value: Option<Expression>,
    pub span: crate::diagnostics::Span,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Expression {
    Literal(Literal),
    Identifier(Identifier),
    Binary(BinaryOp),
    Unary(UnaryOp),
    Call(FunctionCall),
    MemberAccess(MemberAccess),
    List(ListLiteral),
    StructInit(StructInitializer),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Literal {
    Integer(i64),
    Float(f64),
    String(String),
    Boolean(bool),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Identifier {
    pub name: String,
    pub span: crate::diagnostics::Span,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BinaryOp {
    pub left: Box<Expression>,
    pub operator: BinaryOperator,
    pub right: Box<Expression>,
    pub span: crate::diagnostics::Span,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum BinaryOperator {
    Add, Sub, Mul, Div, Mod,
    Eq, Ne, Lt, Le, Gt, Ge,
    And, Or,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct UnaryOp {
    pub operator: UnaryOperator,
    pub operand: Box<Expression>,
    pub span: crate::diagnostics::Span,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum UnaryOperator {
    Neg, Not
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FunctionCall {
    pub function: Box<Expression>,
    pub arguments: Vec<Expression>,
    pub span: crate::diagnostics::Span,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MemberAccess {
    pub object: Box<Expression>,
    pub member: String,
    pub span: crate::diagnostics::Span,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ListLiteral {
    pub elements: Vec<Expression>,
    pub span: crate::diagnostics::Span,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct StructInitializer {
    pub struct_name: String,
    pub fields: HashMap<String, Expression>,
    pub span: crate::diagnostics::Span,
}