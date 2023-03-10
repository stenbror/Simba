
use crate::parser::tokens::TokenSymbol;
use crate::parser::tokenizer::*;
use crate::parser::ast_nodes;
use crate::parser::ast_nodes::AbstractSyntaxTree;
use crate::parser::simba_parser::SimbaParser;


pub trait Expressions {
    fn parse_expression_or_test(&self) -> Result<Box<AbstractSyntaxTree>, String>;
    fn parse_expression_and_test(&self) -> Result<Box<AbstractSyntaxTree>, String>;
    fn parse_expression_not_test(&self) -> Result<Box<AbstractSyntaxTree>, String>;
}


impl Expressions for SimbaParser {

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