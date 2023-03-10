use crate::parser::tokens::TokenSymbol;

#[derive(Clone, Debug, PartialEq)]
pub enum AbstractSyntaxTree {
    Empty(u32),
    And(u32, u32, Box<AbstractSyntaxTree>, Box<TokenSymbol>, Box<AbstractSyntaxTree>),
    Not(u32, u32, Box<TokenSymbol>, Box<AbstractSyntaxTree>),
    Or(u32, u32, Box<AbstractSyntaxTree>, Box<TokenSymbol>, Box<AbstractSyntaxTree>)
}