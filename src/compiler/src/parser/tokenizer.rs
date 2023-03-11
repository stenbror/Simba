
use crate::parser::tokens::TokenSymbol;
use crate::parser::tokens::TokenSymbol::EOF;


// SimbaTokenizer data structure //////////////////////////////////////////////////////////////////////////////////////

pub struct SimbaTokenizer {
    pub(crate) symbol: Result<Box<TokenSymbol>, String>,
    pub(crate) cur_pos: u32
}

// Trait: Tokenizer ///////////////////////////////////////////////////////////////////////////////////////////////////

pub trait Tokenizer {
    fn new(tab_size: u8) -> SimbaTokenizer;
    fn advance(&self);
    fn is_keyword(&self, text: &str, start_pos: u32, end_pos: u32) -> Option<TokenSymbol>;
    fn is_operator_or_delimiter(&self, chars: ( char, char, char ), start_pos: u32, end_pos: u32) -> Option<(TokenSymbol, u8)>;
}

// Implements all functions of trait: Tokenizer for SimbaTokenizer ////////////////////////////////////////////////////

impl Tokenizer for SimbaTokenizer {

    fn new(tab_size: u8) -> SimbaTokenizer {
        SimbaTokenizer {
            symbol: Ok(Box::new(TokenSymbol::EOF)),
            cur_pos: 0
        }
    }

    fn advance(&self) {

    }

    fn is_keyword(&self, text: &str, start_pos: u32, end_pos: u32) -> Option<TokenSymbol> {
        match text {
            "and" => Some(EOF),
            "constructor" => Some(EOF),
            "destructor" => Some(EOF),
            "ensure" => Some(EOF), // assert
            "excluding" => Some(EOF), // not in
            "fun" => Some(EOF),
            "including" => Some(EOF), // in
            "match" => Some(EOF),
            "method" => Some(EOF),
            "mutable" => Some(EOF),
            "or" => Some(EOF),
            "property" => Some(EOF),
            "not" => Some(EOF),
            "type" => Some(EOF),
            "use" => Some(EOF),
            "with" => Some(EOF),
            _ => None
        }
    }

    fn is_operator_or_delimiter(&self, chars: ( char, char, char ), start_pos: u32, end_pos: u32) -> Option<(TokenSymbol, u8)> {
        match chars {
            ( '(', ')', _  ) => Some((EOF, 2)), // Unit = Void
            ( ':', ':', _  ) => Some((EOF, 2)),
            ( ':' , '=', _ ) => Some((EOF, 2)),
            _ => None
        }
    }
}

// Unittests for Tokenizer module of Simba Language ///////////////////////////////////////////////////////////////////

#[cfg(test)]
mod tests {
    use super::*;


    #[test]
    fn dummy_test() {
        assert!(true)
    }
}