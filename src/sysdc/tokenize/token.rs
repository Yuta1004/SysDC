#[derive(Debug)]
#[derive(PartialEq)]
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

#[derive(Debug)]
pub struct Token {
    pub kind: TokenKind,
    orig_id: Option<String>,
    orig_number: Option<i32>
}

impl Token {
    pub fn from_string(orig: String) -> Token {
        let kind = match orig.as_str() {
            "layer"     => TokenKind::Layer,
            "ref"       => TokenKind::Ref,
            "data"      => TokenKind::Data,
            "module"    => TokenKind::Module,
            "use"       => TokenKind::Use,
            "modify"    => TokenKind::Modify,
            "link"      => TokenKind::Link,
            "branch"    => TokenKind::Branch,
            "chain"     => TokenKind::Chain,
            "->"        => TokenKind::Allow,
            ":"         => TokenKind::Mapping,
            "="         => TokenKind::Equal,
            "."         => TokenKind::Accessor,
            "::"        => TokenKind::PAccessor,
            ","         => TokenKind::Separater,
            ";"         => TokenKind::Semicolon,
            "("         => TokenKind::ParenthesisBegin,
            ")"         => TokenKind::ParenthesisEnd,
            "{"         => TokenKind::BracketBegin,
            "}"         => TokenKind::BracketEnd,
            _           => TokenKind::Identifier
        };
        let orig_id = match kind {
            TokenKind::Identifier => Some(orig),
            _ => None
        };

        Token { kind, orig_id, orig_number: None }
    }

    pub fn from_i32(orig: i32) -> Token {
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
    fn create_token_from_string() {
        let str_kind_mapping = [
            ("layer",   TokenKind::Layer),
            ("ref",     TokenKind::Ref),
            ("data",    TokenKind::Data),
            ("module",  TokenKind::Module),
            ("use",     TokenKind::Use),
            ("modify",  TokenKind::Modify),
            ("link",    TokenKind::Link),
            ("branch",  TokenKind::Branch),
            ("chain",   TokenKind::Chain),
            ("->",      TokenKind::Allow),
            (":",       TokenKind::Mapping),
            ("=",       TokenKind::Equal),
            (".",       TokenKind::Accessor),
            ("::",      TokenKind::PAccessor),
            (",",       TokenKind::Separater),
            (";",       TokenKind::Semicolon),
            ("(",       TokenKind::ParenthesisBegin),
            (")",       TokenKind::ParenthesisEnd),
            ("{",       TokenKind::BracketBegin),
            ("}",       TokenKind::BracketEnd),
        ];
        for (_str, kind) in str_kind_mapping {
            assert_eq!(Token::from_string(_str.to_string()).kind, kind);
        }
    }

    #[test]
    #[should_panic]
    fn get_identifer_from_reserved_token() {
        Token::from_string("->".to_string()).get_id();
    }

    #[test]
    #[should_panic]
    fn get_number_from_reserved_token() {
        Token::from_string("->".to_string()).get_number();
    }

    #[test]
    fn get_identifer_from_identifer_token() {
        let id = Token::from_string("test".to_string()).get_id();
        assert_eq!(id, "test");
    }

    #[test]
    #[should_panic]
    fn get_number_from_identifer_token() {
        Token::from_string("test".to_string()).get_number();
    }

    #[test]
    #[should_panic]
    fn get_identifer_from_number_token() {
        Token::from_i32(1204).get_id();
    }

    #[test]
    fn get_number_from_number_token() {
        let number = Token::from_i32(1204).get_number();
        assert_eq!(number, 1204);
    }
}
