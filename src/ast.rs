#[derive(Debug, PartialEq)]
pub struct Program {
    pub statements: Vec<Statement>,
}

#[derive(Debug, PartialEq)]
pub enum Statement {
    Let(LetStatement),
}

#[derive(Debug, PartialEq)]
pub enum Expression {
    Empty,
}

#[derive(Debug, PartialEq)]
pub struct LetStatement {
    pub name: Ident,
    pub value: Expression,
}

#[derive(Debug, PartialEq)]
pub struct Ident(pub String);
