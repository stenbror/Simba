

pub struct SimbaParser {

}


pub trait LanguageParser {
    fn new(tab_size: u8) -> SimbaParser;
}


impl LanguageParser for SimbaParser {
    fn new(tab_size: u8) -> SimbaParser {
        SimbaParser {

        }
    }
}
