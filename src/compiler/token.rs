#[derive(Debug, Clone, PartialEq)]
pub enum TokenKind {
    /* Reserved */
    Layer,              // layer
    Ref,                // ref
    Data,               // data
    Module,             // module
    Binds,              // binds
    As,                 // as
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
    ListBegin,          // [
    ListEnd,            // ]

    /* Others */
    Identifier,
    Number
}

#[derive(Debug, Clone)]
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
            "binds"     => TokenKind::Binds,
            "as"        => TokenKind::As,
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
            "["         => TokenKind::ListBegin,
            "]"         => TokenKind::ListEnd,
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
            None => panic!("[ERROR] get_id called for token {:?}", self.kind)
        }
    }

    pub fn get_number(&self) -> i32 {
        match &self.orig_number {
            Some(number) => *number,
            None => panic!("[ERROR] get_number called for token {:?}", self.kind)
        }
    }
}

#[derive(Debug)]
pub struct Tokenizer<'a> {
    text: &'a String,
    now_ref_pos: usize,
    hold_token: Option<Token>
}

impl<'a> Tokenizer<'a> {
    pub fn new(text: &'a String) -> Tokenizer<'a> {
        let mut tokenizer = Tokenizer {
            text,
            now_ref_pos: 0,
            hold_token: None
        };
        tokenizer.skip_space();
        tokenizer
    }

    pub fn has_token(&self) -> bool {
        self.now_ref_pos != self.text.len()
    }

    pub fn expect(&mut self, kind: TokenKind) -> Option<Token> {
        if let Some(token) = self.tokenize() {
            if token.kind == kind {
                self.hold_token = None;
                Some(token)
            } else {
                self.hold_token = Some(token);
                None
            }
        } else {
            None
        }
    }

    pub fn request(&mut self, kind: TokenKind) -> Token {
        match self.expect(kind.clone()) {
            Some(token) => token,
            None => panic!("[ERROR] Token \"{:?}\" is requested, but not found.", kind)
        }
    }

    fn tokenize(&mut self) -> Option<Token> {
        if !self.hold_token.is_none() {
            return self.hold_token.clone();
        }
        if !self.has_token() {
            return None;
        }

        let lead_ref_pos = self.now_ref_pos;
        let lead_type = CharType::from(self.get_char_at(lead_ref_pos));
        self.now_ref_pos += 1;

        while self.has_token() {
            let now_type = CharType::from(self.get_char_at(self.now_ref_pos));
            match (&lead_type, now_type) {
                // Ok(continue)
                (CharType::Identifier, CharType::Identifier | CharType::Number) => {},
                (CharType::Number, CharType::Number) => {},
                
                // Ok(force stop)
                (CharType::Symbol, _) => break,
                (CharType::SymbolAccessor, CharType::SymbolAccessor) => { self.now_ref_pos += 1; break },
                (CharType::SymbolAccessor, _) => break,
                (CharType::SymbolAllow1, CharType::SymbolAllow2) => { self.now_ref_pos += 1; break },

                // Ng(panic)
                (CharType::SymbolAllow1 | CharType::SymbolAllow2, _) => panic!("[ERROR] Discovered unregistered symbol."),

                // Ok(force stop)
                _ => break
            }

            self.now_ref_pos += 1;
        }

        let discovered_word = self.clip_text(lead_ref_pos, self.now_ref_pos);
        let token = match lead_type {
            CharType::Number => Token::from_i32(discovered_word.parse::<i32>().unwrap()),
            _ => Token::from_string(discovered_word)
        };
        self.skip_space();
        Some(token)
    }

    fn skip_space(&mut self) {
        loop {
            if !self.has_token() {
                break;
            }
            match CharType::from(self.get_char_at(self.now_ref_pos)) {
                CharType::Space => self.now_ref_pos += 1,
                _ => break
            }
        }
    }

    fn get_char_at(&self, pos: usize) -> char {
        self.text.chars().nth(pos).unwrap()
    }

    fn clip_text(&self, begin: usize, end: usize) -> String {
        let (begin_idx, _) = self.text.char_indices().nth(begin).unwrap();
        if end != self.text.len() {
            let (end_idx, _) = self.text.char_indices().nth(end).unwrap();
            self.text[begin_idx..end_idx].to_string()
        } else {
            self.text[begin_idx..].to_string()
        }
    }
}

#[derive(Debug)]
enum CharType {
    Number,
    Identifier,
    Symbol,
    SymbolAllow1,
    SymbolAllow2,
    SymbolAccessor,
    Space,
    Other
}

impl CharType {
    fn from(c: char) -> CharType {
        match c {
            '0'..='9' => CharType::Number,
            'a'..='z' | 'A'..='Z' | '_' => CharType::Identifier,
            '=' | '.' | ',' | ';' | '{' | '}' | '(' | ')' | '[' | ']' => CharType::Symbol,
            '-' => CharType::SymbolAllow1,
            '>' => CharType::SymbolAllow2,
            ':' => CharType::SymbolAccessor,
            ' ' | '\t' | '\n' => CharType::Space,
            _ => CharType::Other
        }
    }
}

#[cfg(test)]
mod test {
    mod token {
        use super::super::{ Token, TokenKind };

        #[test]
        fn create_token_from_string() {
            let str_kind_mapping = [
                ("layer",   TokenKind::Layer),
                ("ref",     TokenKind::Ref),
                ("data",    TokenKind::Data),
                ("module",  TokenKind::Module),
                ("binds",   TokenKind::Binds),
                ("as",      TokenKind::As),
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
                ("[",       TokenKind::ListBegin),
                ("]",       TokenKind::ListEnd)
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

    mod tokenizer {
        use super::super::{ TokenKind, Tokenizer };

        #[test]
        pub fn create_tokenizer() {
            let text = "cocoa 410 cappuccino 1204".to_string();
            Tokenizer::new(&text);
        }

        #[test]
        fn expect_all_ok() {
            let text = "
                layer 0;
                data User {
                    id: int32,
                    name: String
                }
                module UserModule binds User as this {
                    greet() -> None {
                        use = [this.name];
                        link = chain {
                            Printer::print(text: string)
                        }
                    }
                }".to_string();
            let correct_token_kinds = [
                TokenKind::Layer,
                TokenKind::Number,
                TokenKind::Semicolon,
                TokenKind::Data,
                TokenKind::Identifier,
                TokenKind::BracketBegin,
                TokenKind::Identifier,
                TokenKind::Mapping,
                TokenKind::Identifier,
                TokenKind::Separater,
                TokenKind::Identifier,
                TokenKind::Mapping,
                TokenKind::Identifier,
                TokenKind::BracketEnd,
                TokenKind::Module,
                TokenKind::Identifier,
                TokenKind::Binds,
                TokenKind::Identifier,
                TokenKind::As,
                TokenKind::Identifier,
                TokenKind::BracketBegin,
                TokenKind::Identifier,
                TokenKind::ParenthesisBegin,
                TokenKind::ParenthesisEnd,
                TokenKind::Allow,
                TokenKind::Identifier,
                TokenKind::BracketBegin,
                TokenKind::Use,
                TokenKind::Equal,
                TokenKind::ListBegin,
                TokenKind::Identifier,
                TokenKind::Accessor,
                TokenKind::Identifier,
                TokenKind::ListEnd,
                TokenKind::Semicolon,
                TokenKind::Link,
                TokenKind::Equal,
                TokenKind::Chain,
                TokenKind::BracketBegin,
                TokenKind::Identifier,
                TokenKind::PAccessor,
                TokenKind::Identifier,
                TokenKind::ParenthesisBegin,
                TokenKind::Identifier,
                TokenKind::Mapping,
                TokenKind::Identifier,
                TokenKind::ParenthesisEnd,
                TokenKind::BracketEnd,
                TokenKind::BracketEnd,
                TokenKind::BracketEnd
            ];

            let mut tokenizer = Tokenizer::new(&text);
            for token_kind in correct_token_kinds {
                match tokenizer.expect(token_kind.clone()) {
                    Some(_) => {}
                    None => assert!(false, "{:?}", token_kind)
                }
            }
            assert!(!tokenizer.has_token());
        }

        #[test]
        fn expect_all_ng() {
            let text = "data".to_string();

            let mut tokenizer = Tokenizer::new(&text);
            for token_kind in [TokenKind::Layer, TokenKind::Ref, TokenKind::Data] {
                match tokenizer.expect(token_kind.clone()) {
                    Some(_) => assert_eq!(token_kind, TokenKind::Data),
                    None => assert_ne!(token_kind, TokenKind::Data)
                }
            }
            assert!(!tokenizer.has_token());
        }

        #[test]
        fn request_all_ok() {
            let text = "data module cocoa 410".to_string();
            let correct_token_kinds = [
                TokenKind::Data,
                TokenKind::Module,
                TokenKind::Identifier,
                TokenKind::Number
            ];

            let mut tokenizer = Tokenizer::new(&text);
            for token_kind in correct_token_kinds {
                let token = tokenizer.request(token_kind.clone());
                assert_eq!(token.kind, token_kind);
            }
            assert!(!tokenizer.has_token());
        }

        #[test]
        #[should_panic]
        fn request_ng() {
            let text = "data".to_string();
            let mut tokenizer = Tokenizer::new(&text);
            tokenizer.request(TokenKind::Number);
        }
    }
}
