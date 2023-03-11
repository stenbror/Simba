use crate::parser::tokens::TokenSymbol;

#[derive(Clone, Debug, PartialEq)]
pub enum AbstractSyntaxTree {
    Empty(u32),
    NamedExpr(u32, u32, Box<AbstractSyntaxTree>, Box<TokenSymbol>, Box<AbstractSyntaxTree>),
    Lambda(u32, u32, Box<AbstractSyntaxTree>, Box<TokenSymbol>, Box<AbstractSyntaxTree>),
    Test(u32, u32, Box<AbstractSyntaxTree>, Box<TokenSymbol>, Box<AbstractSyntaxTree>, Box<TokenSymbol>, Box<AbstractSyntaxTree>),
    Or(u32, u32, Box<AbstractSyntaxTree>, Box<TokenSymbol>, Box<AbstractSyntaxTree>),
    And(u32, u32, Box<AbstractSyntaxTree>, Box<TokenSymbol>, Box<AbstractSyntaxTree>),
    Not(u32, u32, Box<TokenSymbol>, Box<AbstractSyntaxTree>),
    Including(u32, u32, Box<AbstractSyntaxTree>, Box<TokenSymbol>, Box<AbstractSyntaxTree>),
    Excluding(u32, u32, Box<AbstractSyntaxTree>, Box<TokenSymbol>, Box<AbstractSyntaxTree>),
    Is(u32, u32, Box<AbstractSyntaxTree>, Box<TokenSymbol>, Box<AbstractSyntaxTree>),
    IsNot(u32, u32, Box<AbstractSyntaxTree>, Box<TokenSymbol>, Box<TokenSymbol>, Box<AbstractSyntaxTree>),
    Less(u32, u32, Box<AbstractSyntaxTree>, Box<TokenSymbol>, Box<AbstractSyntaxTree>),
    LessEqual(u32, u32, Box<AbstractSyntaxTree>, Box<TokenSymbol>, Box<AbstractSyntaxTree>),
    Equal(u32, u32, Box<AbstractSyntaxTree>, Box<TokenSymbol>, Box<AbstractSyntaxTree>),
    GreaterEqual(u32, u32, Box<AbstractSyntaxTree>, Box<TokenSymbol>, Box<AbstractSyntaxTree>),
    Greater(u32, u32, Box<AbstractSyntaxTree>, Box<TokenSymbol>, Box<AbstractSyntaxTree>),
    NotEqual(u32, u32, Box<AbstractSyntaxTree>, Box<TokenSymbol>, Box<AbstractSyntaxTree>),
    BitwiseOr(u32, u32, Box<AbstractSyntaxTree>, Box<TokenSymbol>, Box<AbstractSyntaxTree>),
    BitwiseXor(u32, u32, Box<AbstractSyntaxTree>, Box<TokenSymbol>, Box<AbstractSyntaxTree>),
    BitwiseAnd(u32, u32, Box<AbstractSyntaxTree>, Box<TokenSymbol>, Box<AbstractSyntaxTree>),
    BitwiseShiftLeft(u32, u32, Box<AbstractSyntaxTree>, Box<TokenSymbol>, Box<AbstractSyntaxTree>),
    BitwiseShiftRight(u32, u32, Box<AbstractSyntaxTree>, Box<TokenSymbol>, Box<AbstractSyntaxTree>),
    Plus(u32, u32, Box<AbstractSyntaxTree>, Box<TokenSymbol>, Box<AbstractSyntaxTree>),
    Minus(u32, u32, Box<AbstractSyntaxTree>, Box<TokenSymbol>, Box<AbstractSyntaxTree>),
}