pub enum TokenKind {
    /* Reserved */
    Layer,              // layer
    Ref,                // ref
    Data,               // data
    Module,             // module
    Use,                // use
    Modify,             // modify
    Link,               // link
    Branch,             // branch
    Chain,              // chain

    /* Symbol */
    Allow,              // ->
    Mapping,            // :
    Equal,              // =
    Accessor,           // .
    PAccessor,          // ::
    Separater,          // ,
    Semicolon,          // ;
    ParenthesisBegin,   // (
    ParenthesisEnd,     // )
    BracketBegin,       // {
    BracketEnd,         // }

    /* Others */
    Identifier,
    Number
}

pub struct Token {
    pub kind: TokenKind,
    orig_id: Option<String>,
    orig_number: Option<i32>
}

impl Token {
    pub fn new(kind: TokenKind) -> Token {
        Token {
            kind,
            orig_id: None,
            orig_number: None
        }
    }

    pub fn new_id(orig: String) -> Token {
        Token {
            kind: TokenKind::Identifier,
            orig_id: Some(orig),
            orig_number: None
        }
    }

    pub fn new_number(orig: i32) -> Token {
        Token {
            kind: TokenKind::Number,
            orig_id: None,
            orig_number: Some(orig)
        } 
    }

    pub fn get_id(&self) -> String {
        match &self.orig_id {
            Some(id) => id.clone(),
            None => panic!("")
        }
    }

    pub fn get_number(&self) -> i32 {
        match &self.orig_number {
            Some(number) => *number,
            None => panic!("")
        }
    }
}

#[cfg(test)]
mod test {
    use super::{ Token, TokenKind };

    #[test]
    #[should_panic]
    fn get_identifer_from_reserved_token() {
        Token::new(TokenKind::Accessor).get_id();
    }

    #[test]
    #[should_panic]
    fn get_number_from_reserved_token() {
        Token::new(TokenKind::Accessor).get_number();
    }

    #[test]
    fn get_identifer_from_identifer_token() {
        let id = Token::new_id("test".to_string()).get_id();
        assert_eq!(id, "test");
    }

    #[test]
    #[should_panic]
    fn get_number_from_identifer_token() {
        Token::new_id("test".to_string()).get_number();
    }

    #[test]
    #[should_panic]
    fn get_identifer_from_number_token() {
        Token::new_number(1204).get_id();
    }

    #[test]
    fn get_number_from_number_token() {
        let number = Token::new_number(1204).get_number();
        assert_eq!(number, 1204);
    }
}
