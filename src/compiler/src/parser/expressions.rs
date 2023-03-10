
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
    fn parse_expression_xor_expr(&self) -> Result<Box<AbstractSyntaxTree>, String>;
    fn parse_expression_and_expr(&self) -> Result<Box<AbstractSyntaxTree>, String>;
    fn parse_expression_shift_expr(&self) -> Result<Box<AbstractSyntaxTree>, String>;
    fn parse_expression_arith_expr(&self) -> Result<Box<AbstractSyntaxTree>, String>;
    fn parse_expression_term(&self) -> Result<Box<AbstractSyntaxTree>, String>;
    fn parse_expression_factor(&self) -> Result<Box<AbstractSyntaxTree>, String>;
    fn parse_expression_power(&self) -> Result<Box<AbstractSyntaxTree>, String>;
    fn parse_expression_atom_expr(&self) -> Result<Box<AbstractSyntaxTree>, String>;
    fn parse_expression_atom(&self) -> Result<Box<AbstractSyntaxTree>, String>;
    fn parse_expression_trailer(&self) -> Result<Box<Vec<Box<AbstractSyntaxTree>>>, String>;
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

    // Rule: xor_expr [ '|' xor_expr ]
    fn parse_expression_or_expr(&self) -> Result<Box<AbstractSyntaxTree>, String> {
        let pos = self.lexer.cur_pos;
        let left = self.parse_expression_xor_expr()?;
        match *self.lexer.symbol.clone()? {
            TokenSymbol::BitwiseOr( _ , _ ) => {
                let symbol = self.lexer.symbol.clone()?;
                self.lexer.advance();
                let right = self.parse_expression_xor_expr()?;
                Ok(Box::new(AbstractSyntaxTree::BitwiseOr(pos, self.lexer.cur_pos, left, symbol, right)))
            },
            _ => Ok(left)
        }
    }

    // Rule: and_expr [ '^' and_expr ]
    fn parse_expression_xor_expr(&self) -> Result<Box<AbstractSyntaxTree>, String> {
        let pos = self.lexer.cur_pos;
        let left = self.parse_expression_and_expr()?;
        match *self.lexer.symbol.clone()? {
            TokenSymbol::BitwiseXor( _ , _ ) => {
                let symbol = self.lexer.symbol.clone()?;
                self.lexer.advance();
                let right = self.parse_expression_and_expr()?;
                Ok(Box::new(AbstractSyntaxTree::BitwiseXor(pos, self.lexer.cur_pos, left, symbol, right)))
            },
            _ => Ok(left)
        }
    }

    // Rule: shift_expr [ '&' shift_expr ]
    fn parse_expression_and_expr(&self) -> Result<Box<AbstractSyntaxTree>, String> {
        let pos = self.lexer.cur_pos;
        let left = self.parse_expression_shift_expr()?;
        match *self.lexer.symbol.clone()? {
            TokenSymbol::BitwiseAnd( _ , _ ) => {
                let symbol = self.lexer.symbol.clone()?;
                self.lexer.advance();
                let right = self.parse_expression_shift_expr()?;
                Ok(Box::new(AbstractSyntaxTree::BitwiseAnd(pos, self.lexer.cur_pos, left, symbol, right)))
            },
            _ => Ok(left)
        }
    }

    // Rule: arith_expr [ ( '<<' | '>>' ) arith_expr ]
    fn parse_expression_shift_expr(&self) -> Result<Box<AbstractSyntaxTree>, String> {
        let pos = self.lexer.cur_pos;
        let left = self.parse_expression_arith_expr()?;
        match *self.lexer.symbol.clone()? {
            TokenSymbol::BitwiseShiftLeft( _ , _ ) => {
                let symbol = self.lexer.symbol.clone()?;
                self.lexer.advance();
                let right = self.parse_expression_arith_expr()?;
                Ok(Box::new(AbstractSyntaxTree::BitwiseShiftLeft(pos, self.lexer.cur_pos, left, symbol, right)))
            },
            TokenSymbol::BitwiseShiftRight( _ , _ ) => {
                let symbol = self.lexer.symbol.clone()?;
                self.lexer.advance();
                let right = self.parse_expression_arith_expr()?;
                Ok(Box::new(AbstractSyntaxTree::BitwiseShiftRight(pos, self.lexer.cur_pos, left, symbol, right)))
            },
            _ => Ok(left)
        }
    }

    // Rule: term [ ( '+' | '-' ) term ]
    fn parse_expression_arith_expr(&self) -> Result<Box<AbstractSyntaxTree>, String> {
        let pos = self.lexer.cur_pos;
        let left = self.parse_expression_term()?;
        match *self.lexer.symbol.clone()? {
            TokenSymbol::Plus( _ , _ ) => {
                let symbol = self.lexer.symbol.clone()?;
                self.lexer.advance();
                let right = self.parse_expression_term()?;
                Ok(Box::new(AbstractSyntaxTree::Plus(pos, self.lexer.cur_pos, left, symbol, right)))
            },
            TokenSymbol::Minus( _ , _ ) => {
                let symbol = self.lexer.symbol.clone()?;
                self.lexer.advance();
                let right = self.parse_expression_term()?;
                Ok(Box::new(AbstractSyntaxTree::Minus(pos, self.lexer.cur_pos, left, symbol, right)))
            },
            _ => Ok(left)
        }
    }

    // Rule: factor [ ( '*' | '%' | '@' | '/' ) factor ]
    fn parse_expression_term(&self) -> Result<Box<AbstractSyntaxTree>, String> {
        let pos = self.lexer.cur_pos;
        let left = self.parse_expression_factor()?;
        match *self.lexer.symbol.clone()? {
            TokenSymbol::Mul( _ , _ ) => {
                let symbol = self.lexer.symbol.clone()?;
                self.lexer.advance();
                let right = self.parse_expression_factor()?;
                Ok(Box::new(AbstractSyntaxTree::Mul(pos, self.lexer.cur_pos, left, symbol, right)))
            },
            TokenSymbol::Modulo( _ , _ ) => {
                let symbol = self.lexer.symbol.clone()?;
                self.lexer.advance();
                let right = self.parse_expression_factor()?;
                Ok(Box::new(AbstractSyntaxTree::Modulo(pos, self.lexer.cur_pos, left, symbol, right)))
            },
            TokenSymbol::Matrice( _ , _ ) => {
                let symbol = self.lexer.symbol.clone()?;
                self.lexer.advance();
                let right = self.parse_expression_factor()?;
                Ok(Box::new(AbstractSyntaxTree::Matrice(pos, self.lexer.cur_pos, left, symbol, right)))
            },
            TokenSymbol::Div( _ , _ ) => {
                let symbol = self.lexer.symbol.clone()?;
                self.lexer.advance();
                let right = self.parse_expression_factor()?;
                Ok(Box::new(AbstractSyntaxTree::Div(pos, self.lexer.cur_pos, left, symbol, right)))
            },
            _ => Ok(left)
        }
    }

    // Rule:  ( ( '+' | '-' | '~ | '++' | '--' ) power_Expr ) | power_Expr [ '++' | '--' ]
    fn parse_expression_factor(&self) -> Result<Box<AbstractSyntaxTree>, String> {
        let pos = self.lexer.cur_pos;
        match *self.lexer.symbol.clone()? {
            TokenSymbol::Plus( _ , _ ) => {
                let symbol = self.lexer.symbol.clone()?;
                self.lexer.advance();
                let right = self.parse_expression_power()?;
                Ok(Box::new(AbstractSyntaxTree::UnaryPlus(pos, self.lexer.cur_pos, symbol, right)))
            },
            TokenSymbol::Minus( _ , _ ) => {
                let symbol = self.lexer.symbol.clone()?;
                self.lexer.advance();
                let right = self.parse_expression_power()?;
                Ok(Box::new(AbstractSyntaxTree::UnaryMinus(pos, self.lexer.cur_pos, symbol, right)))
            },
            TokenSymbol::UnaryBitwiseInvert( _ , _ ) => {
                let symbol = self.lexer.symbol.clone()?;
                self.lexer.advance();
                let right = self.parse_expression_power()?;
                Ok(Box::new(AbstractSyntaxTree::UnaryBitwiseInvert(pos, self.lexer.cur_pos, symbol, right)))
            },
            TokenSymbol::Increment( _ , _ ) => { // '++'
                let symbol = self.lexer.symbol.clone()?;
                self.lexer.advance();
                let right = self.parse_expression_power()?;
                Ok(Box::new(AbstractSyntaxTree::UnaryPreIncrement(pos, self.lexer.cur_pos, symbol, right)))
            },
            TokenSymbol::Decrement( _ , _ ) => { // '--'
                let symbol = self.lexer.symbol.clone()?;
                self.lexer.advance();
                let right = self.parse_expression_power()?;
                Ok(Box::new(AbstractSyntaxTree::UnaryPreDecrement(pos, self.lexer.cur_pos, symbol, right)))
            },
            _ => {
                let right = self.parse_expression_power()?;
                match *self.lexer.symbol.clone()? {
                    TokenSymbol::Increment( _ , _ ) => {
                        let symbol = self.lexer.symbol.clone()?;
                        self.lexer.advance();
                        Ok(Box::new(AbstractSyntaxTree::UnaryPostIncrement(pos, self.lexer.cur_pos, right, symbol)))
                    },
                    TokenSymbol::Decrement( _ , _ ) => {
                        let symbol = self.lexer.symbol.clone()?;
                        self.lexer.advance();
                        Ok(Box::new(AbstractSyntaxTree::UnaryPostDecrement(pos, self.lexer.cur_pos, right, symbol)))
                    },
                    _ => Ok(right)
                }
            }
        }
    }

    // Rule: atom_expr [ '**' factor ]
    fn parse_expression_power(&self) -> Result<Box<AbstractSyntaxTree>, String> {
        let pos = self.lexer.cur_pos;
        let left = self.parse_expression_atom_expr()?;
        match *self.lexer.symbol.clone()? {
            TokenSymbol::Power( _ , _ ) => {
                let symbol = self.lexer.symbol.clone()?;
                self.lexer.advance();
                let right = self.parse_expression_factor()?;
                Ok(Box::new(AbstractSyntaxTree::Power(pos, self.lexer.cur_pos, left, symbol, right)))
            },
            _ => Ok(left)
        }
    }

    // Rule: [ 'await' ] atom [ trailer* ]
    fn parse_expression_atom_expr(&self) -> Result<Box<AbstractSyntaxTree>, String> {
        let pos = self.lexer.cur_pos;
        let mut symbol : Box<TokenSymbol> = Box::new(TokenSymbol::Empty); // 'await'
        match *self.lexer.symbol.clone()? {
            TokenSymbol::Await( _ , _ ) => {
                symbol = self.lexer.symbol.clone()?;
                self.lexer.advance();
            },
            _ => ()
        }
        let left = self.parse_expression_atom()?;
        match ( &*symbol, *self.lexer.symbol.clone()?)  {
            ( _ , TokenSymbol::Dot( _ , _ ) ) |
            ( _ , TokenSymbol::ColonColon( _ , _ ) ) |
            ( _ , TokenSymbol::LeftParen( _ , _ ) ) |
            ( _ , TokenSymbol::LeftBracket( _ , _ ) ) => {
                let right = self.parse_expression_trailer()?;
                let await_symbol_elements = match *symbol { TokenSymbol::Await( _ , _ ) => Some(symbol), _ => None };
                Ok(Box::new(AbstractSyntaxTree::TrailerList(pos, self.lexer.cur_pos, await_symbol_elements, left, Some(right))))
            },
            ( TokenSymbol::Await( _ , _ ), _ ) => {
                let await_symbol = match *symbol { TokenSymbol::Await( _ , _ ) => Some(symbol), _ => None };
                Ok(Box::new(AbstractSyntaxTree::TrailerList(pos, self.lexer.cur_pos, await_symbol, left, None)))
            },
            ( _ , _ ) => Ok(left)
        }
    }

    fn parse_expression_atom(&self) -> Result<Box<AbstractSyntaxTree>, String> {
        Ok(Box::new(AbstractSyntaxTree::Empty(0)))
    }

    fn parse_expression_trailer(&self) -> Result<Box<Vec<Box<AbstractSyntaxTree>>>, String> {
        let a = Box::new( AbstractSyntaxTree::Empty(0) );
        let b = vec! [ a ];
        let c = Box::new( b );
        Ok(c)
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