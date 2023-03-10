use crate::parser::tokens::TokenSymbol;

#[derive(Clone)]
pub enum AbstractSyntaxTree {
    And(u32, u32, Box<AbstractSyntaxTree>, Box<TokenSymbol>, Box<AbstractSyntaxTree>),
    Not(u32, u32, Box<TokenSymbol>, Box<AbstractSyntaxTree>),
    Or(u32, u32, Box<AbstractSyntaxTree>, Box<TokenSymbol>, Box<AbstractSyntaxTree>)
}