use crate::parser::tokenizer::{SimbaTokenizer, Tokenizer};

pub struct SimbaParser {
    pub(crate) lexer: Box<SimbaTokenizer>
}


pub trait LanguageParser {
    fn new(tab_size: u8) -> SimbaParser;
}


impl LanguageParser for SimbaParser {
    fn new(tab_size: u8) -> SimbaParser {
        SimbaParser {
            lexer: Box::new(SimbaTokenizer::new(4) ),
        }
    }
}
