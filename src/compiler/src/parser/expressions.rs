
use crate::parser::tokens::TokenSymbol;
use crate::parser::tokenizer::*;
use crate::parser::ast_nodes;
use crate::parser::ast_nodes::AbstractSyntaxTree;
use crate::parser::simba_parser::SimbaParser;


pub trait Expressions {
    fn parse_expression_named_expr(&self) -> Result<Box<AbstractSyntaxTree>, String>;
    fn parse_expression_test(&self) -> Result<Box<AbstractSyntaxTree>, String>;
    fn parse_expression_lambda(&self) -> Result<Box<AbstractSyntaxTree>, String>;
    fn parse_expression_or_test(&self) -> Result<Box<AbstractSyntaxTree>, String>;
    fn parse_expression_and_test(&self) -> Result<Box<AbstractSyntaxTree>, String>;
    fn parse_expression_not_test(&self) -> Result<Box<AbstractSyntaxTree>, String>;
    fn parse_expression_comparison(&self) -> Result<Box<AbstractSyntaxTree>, String>;
    fn parse_expression_or_expr(&self) -> Result<Box<AbstractSyntaxTree>, String>;
    fn parse_expression_and_expr(&self) -> Result<Box<AbstractSyntaxTree>, String>;
    fn parse_expression_not_expr(&self) -> Result<Box<AbstractSyntaxTree>, String>;
}


impl Expressions for SimbaParser {

    // Rule: test [ ':=' test ]
    fn parse_expression_named_expr(&self) -> Result<Box<AbstractSyntaxTree>, String> {
        let pos = self.lexer.cur_pos;
        let left = self.parse_expression_test()?;
        match *self.lexer.symbol.clone()? {
            TokenSymbol::ColonAssign( _ , _ ) => {
                let symbol = self.lexer.symbol.clone()?;
                self.lexer.advance();
                let right = self.parse_expression_test()?;
                Ok(Box::new(AbstractSyntaxTree::NamedExpr(pos, self.lexer.cur_pos, left, symbol, right)))
            }
            _ => Ok(left)
        }
    }

    // Rule: or_test [ '?' or_test ':' test ] | lambda
    fn parse_expression_test(&self) -> Result<Box<AbstractSyntaxTree>, String> {
        let pos = self.lexer.cur_pos;
        match *self.lexer.symbol.clone()? {
            TokenSymbol::Fun( _ , _ ) => self.parse_expression_lambda(),
            _ => {
                let left = self.parse_expression_or_test()?;
                match *self.lexer.symbol.clone()? {
                    TokenSymbol::Query( _ , _ ) => {
                        let symbol1 = self.lexer.symbol.clone()?;
                        self.lexer.advance();
                        let right = self.parse_expression_or_test()?;
                        match *self.lexer.symbol.clone()? {
                            TokenSymbol::Colon( _ , _ ) => {
                                let symbol2 = self.lexer.symbol.clone()?;
                                self.lexer.advance();
                                let next = self.parse_expression_test()?;
                                Ok(Box::new(AbstractSyntaxTree::Test(pos, self.lexer.cur_pos, left, symbol1, right, symbol2, next)))
                            },
                            _ => Err(format!("SyntaxError: Missing ':' in query operator '?' at position {}", self.lexer.cur_pos))
                        }
                    },
                    _ => Ok(left)
                }
            }
        }
    }

    // Rule: 'fun' arguments '->' test  // arguments are like () for void or a b c d .... for list of arguments.
    fn parse_expression_lambda(&self) -> Result<Box<AbstractSyntaxTree>, String> {
        Ok(Box::new(AbstractSyntaxTree::Empty(0)))
    }

    // Rule: and_test 'or' and_test | and_test
    fn parse_expression_or_test(&self) -> Result<Box<AbstractSyntaxTree>, String> {
        let pos = self.lexer.cur_pos;
        let left = self.parse_expression_and_test()?;
        match *self.lexer.symbol.clone()? {
            TokenSymbol::Or( _ , _ ) => {
                let symbol = self.lexer.symbol.clone()?;
                self.lexer.advance();
                let right = self.parse_expression_and_test()?;
                Ok(Box::new(AbstractSyntaxTree::Or(pos, self.lexer.cur_pos, left, symbol, right)))
            }
            _ => Ok(left)
        }
    }

    // Rule: not_test 'and' not_test | not_test
    fn parse_expression_and_test(&self) -> Result<Box<AbstractSyntaxTree>, String> {
        let pos = self.lexer.cur_pos;
        let left = self.parse_expression_not_test()?;
        match *self.lexer.symbol.clone()? {
            TokenSymbol::And( _ , _ ) => {
                let symbol = self.lexer.symbol.clone()?;
                self.lexer.advance();
                let right = self.parse_expression_not_test()?;
                Ok(Box::new(AbstractSyntaxTree::And(pos, self.lexer.cur_pos, left, symbol, right)))
            }
            _ => Ok(left)
        }
    }

    // Rule: 'not' not_test |
    fn parse_expression_not_test(&self) -> Result<Box<AbstractSyntaxTree>, String> {
        let pos = self.lexer.cur_pos;
        match *self.lexer.symbol.clone()? {
            TokenSymbol::Not( _ , _ ) => {
                let symbol = self.lexer.symbol.clone()?;
                self.lexer.advance();
                let right = self.parse_expression_not_test()?;
                Ok(Box::new(AbstractSyntaxTree::Not(pos, self.lexer.cur_pos, symbol, right)))
            }
            _ => self.parse_expression_comparison()
        }
    }

    // Rule: or_expr [ ( '<' | '>' | '==' | '!=' | '<>' | '>=' | '<=' | 'including' | 'excluding' | 'is' | 'is' 'not' ) or_Expr ]
    fn parse_expression_comparison(&self) -> Result<Box<AbstractSyntaxTree>, String> {
        let pos = self.lexer.cur_pos;
        let left = self.parse_expression_or_expr()?;
        match *self.lexer.symbol.clone()? {
            TokenSymbol::Including( _ , _ ) => {
                let symbol = self.lexer.symbol.clone()?;
                self.lexer.advance();
                let right = self.parse_expression_or_expr()?;
                Ok(Box::new(AbstractSyntaxTree::Including(pos, self.lexer.cur_pos, left, symbol, right)))
            },
            TokenSymbol::Excluding( _ , _ ) => {
                let symbol = self.lexer.symbol.clone()?;
                self.lexer.advance();
                let right = self.parse_expression_or_expr()?;
                Ok(Box::new(AbstractSyntaxTree::Excluding(pos, self.lexer.cur_pos, left, symbol, right)))
            },
            TokenSymbol::Is( _ , _ ) => {
                let symbol1 = self.lexer.symbol.clone()?;
                self.lexer.advance();
                match *self.lexer.symbol.clone()? {
                    TokenSymbol::Not( _ , _ ) => {
                        let symbol2 = self.lexer.symbol.clone()?;
                        self.lexer.advance();
                        let right = self.parse_expression_or_expr()?;
                        Ok(Box::new(AbstractSyntaxTree::IsNot(pos, self.lexer.cur_pos, left, symbol1, symbol2, right)))
                    }
                    _ => {
                        let right = self.parse_expression_or_expr()?;
                        Ok(Box::new(AbstractSyntaxTree::Is(pos, self.lexer.cur_pos, left, symbol1, right)))
                    }
                }
            },
            TokenSymbol::Less( _ , _ ) => {
                let symbol = self.lexer.symbol.clone()?;
                self.lexer.advance();
                let right = self.parse_expression_or_expr()?;
                Ok(Box::new(AbstractSyntaxTree::Less(pos, self.lexer.cur_pos, left, symbol, right)))
            },
            TokenSymbol::LessEqual( _ , _ ) => {
                let symbol = self.lexer.symbol.clone()?;
                self.lexer.advance();
                let right = self.parse_expression_or_expr()?;
                Ok(Box::new(AbstractSyntaxTree::LessEqual(pos, self.lexer.cur_pos, left, symbol, right)))
            },
            TokenSymbol::Equal( _ , _ ) => {
                let symbol = self.lexer.symbol.clone()?;
                self.lexer.advance();
                let right = self.parse_expression_or_expr()?;
                Ok(Box::new(AbstractSyntaxTree::Equal(pos, self.lexer.cur_pos, left, symbol, right)))
            },
            TokenSymbol::NotEqual( _ , _ ) => {
                let symbol = self.lexer.symbol.clone()?;
                self.lexer.advance();
                let right = self.parse_expression_or_expr()?;
                Ok(Box::new(AbstractSyntaxTree::NotEqual(pos, self.lexer.cur_pos, left, symbol, right)))
            },
            TokenSymbol::Greater( _ , _ ) => {
                let symbol = self.lexer.symbol.clone()?;
                self.lexer.advance();
                let right = self.parse_expression_or_expr()?;
                Ok(Box::new(AbstractSyntaxTree::Greater(pos, self.lexer.cur_pos, left, symbol, right)))
            },
            TokenSymbol::GreaterEqual( _ , _ ) => {
                let symbol = self.lexer.symbol.clone()?;
                self.lexer.advance();
                let right = self.parse_expression_or_expr()?;
                Ok(Box::new(AbstractSyntaxTree::GreaterEqual(pos, self.lexer.cur_pos, left, symbol, right)))
            },
            _ => Ok(left)
        }
    }

    fn parse_expression_or_expr(&self) -> Result<Box<AbstractSyntaxTree>, String> {
        let pos = self.lexer.cur_pos;
        let left = self.parse_expression_and_test()?;
        match *self.lexer.symbol.clone()? {
            TokenSymbol::BitwiseOr( _ , _ ) => {
                let symbol = self.lexer.symbol.clone()?;
                self.lexer.advance();
                let right = self.parse_expression_and_test()?;
                Ok(Box::new(AbstractSyntaxTree::BitwiseOr(pos, self.lexer.cur_pos, left, symbol, right)))
            }
            _ => Ok(left)
        }
    }

    fn parse_expression_and_expr(&self) -> Result<Box<AbstractSyntaxTree>, String> {
        let pos = self.lexer.cur_pos;
        let left = self.parse_expression_and_test()?;
        match *self.lexer.symbol.clone()? {
            TokenSymbol::BitwiseXor( _ , _ ) => {
                let symbol = self.lexer.symbol.clone()?;
                self.lexer.advance();
                let right = self.parse_expression_and_test()?;
                Ok(Box::new(AbstractSyntaxTree::BitwiseXor(pos, self.lexer.cur_pos, left, symbol, right)))
            }
            _ => Ok(left)
        }
    }

    fn parse_expression_not_expr(&self) -> Result<Box<AbstractSyntaxTree>, String> {
        Ok(Box::new(AbstractSyntaxTree::Empty(0)))
    }
}

// Unittests for Expression rules of parser. //////////////////////////////////////////////////////////////////////////

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn dummy_test() {
        assert!(true)
    }
}