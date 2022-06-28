use super::token::Tokenizer;

pub struct Parser<'a> {
    tokenizer: Tokenizer<'a>
}

impl<'a> Parser<'a> {
    pub fn new(tokenizer: Tokenizer<'a>) -> Parser<'a> {
        Parser { tokenizer }
    }
}
