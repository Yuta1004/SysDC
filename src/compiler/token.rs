#[derive(Debug, Clone, PartialEq)]
pub enum TokenKind {
    /* Reserved */
    Data,               // data
    Module,             // module
    Return,             // return
    Spawn,              // spawn
    Let,                // let
    Use,                // use


    /* Symbol */
    Allow,              // ->
    Mapping,            // :
    Equal,              // =
    Accessor,           // .
    Separater,          // ,
    Semicolon,          // ;
    ParenthesisBegin,   // (
    ParenthesisEnd,     // )
    BracketBegin,       // {
    BracketEnd,         // }
    AtMark,             // @
    Plus,               // +

    /* Others */
    Identifier
}

#[derive(Debug, Clone)]
pub struct Token {
    pub kind: TokenKind,
    orig_id: Option<String>
}

impl Token {
    pub fn from(orig: String) -> Token {
        let kind = match orig.as_str() {
            "data"      => TokenKind::Data,
            "module"    => TokenKind::Module,
            "return"    => TokenKind::Return,
            "let"       => TokenKind::Let,
            "spawn"     => TokenKind::Spawn,
            "use"       => TokenKind::Use,
            "->"        => TokenKind::Allow,
            ":"         => TokenKind::Mapping,
            "="         => TokenKind::Equal,
            "."         => TokenKind::Accessor,
            ","         => TokenKind::Separater,
            ";"         => TokenKind::Semicolon,
            "("         => TokenKind::ParenthesisBegin,
            ")"         => TokenKind::ParenthesisEnd,
            "{"         => TokenKind::BracketBegin,
            "}"         => TokenKind::BracketEnd,
            "@"         => TokenKind::AtMark,
            "+"         => TokenKind::Plus,
            _           => TokenKind::Identifier,
        };
        let orig_id = match kind {
            TokenKind::Identifier => Some(orig),
            _ => None
        };

        Token { kind, orig_id }
    }

    pub fn get_id(&self) -> String {
        match &self.orig_id {
            Some(id) => id.clone(),
            None => panic!("[ERROR] get_id called for token {:?}", self.kind)
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
                (CharType::SymbolAllow1, CharType::SymbolAllow2) => { self.now_ref_pos += 1; break },

                // Ng(panic)
                (CharType::SymbolAllow1 | CharType::SymbolAllow2, _) => panic!("[ERROR] Discovered unregistered symbol."),

                // Ok(force stop)
                _ => break
            }

            self.now_ref_pos += 1;
        }

        let discovered_word = self.clip_text(lead_ref_pos, self.now_ref_pos);
        let token = Token::from(discovered_word);
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
    Space,
    Other
}

impl CharType {
    fn from(c: char) -> CharType {
        match c {
            '0'..='9' => CharType::Number,
            'a'..='z' | 'A'..='Z' | '_' => CharType::Identifier,
            '=' | '.' | ',' | ';' | '{' | '}' | '(' | ')' | ':' => CharType::Symbol,
            '-' => CharType::SymbolAllow1,
            '>' => CharType::SymbolAllow2,
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
        fn create_token_from() {
            let str_kind_mapping = [
                ("data",    TokenKind::Data),
                ("module",  TokenKind::Module),
                ("return",  TokenKind::Return),
                ("let",     TokenKind::Let),
                ("spawn",   TokenKind::Spawn),
                ("use",     TokenKind::Use), 
                ("->",      TokenKind::Allow),
                (":",       TokenKind::Mapping),
                ("=",       TokenKind::Equal),
                (".",       TokenKind::Accessor),
                (",",       TokenKind::Separater),
                (";",       TokenKind::Semicolon),
                ("(",       TokenKind::ParenthesisBegin),
                (")",       TokenKind::ParenthesisEnd),
                ("{",       TokenKind::BracketBegin),
                ("}",       TokenKind::BracketEnd),
                ("@",       TokenKind::AtMark),
                ("+",       TokenKind::Plus)
            ];
            for (_str, kind) in str_kind_mapping {
                assert_eq!(Token::from(_str.to_string()).kind, kind);
            }
        }

        #[test]
        #[should_panic]
        fn get_identifer_from_reserved_token() {
            Token::from("->".to_string()).get_id();
        }

        #[test]
        fn get_identifer_from_identifer_token() {
            let id = Token::from("test".to_string()).get_id();
            assert_eq!(id, "test");
            Token::from("test".to_string()).get_id();
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
                data Box {
                    x: i32,
                    y: i32
                }

                module BoxModule {
                    move(box: Box, dx: i32, dy: i32) -> Box {
                        @return movedBox

                        @spawn movedBox: Box {
                            use box.x, box.y;
                        }
                    }
                }".to_string();
            let correct_token_kinds = [
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
                TokenKind::BracketBegin,
                TokenKind::Identifier,
                TokenKind::ParenthesisBegin,
                TokenKind::Identifier,
                TokenKind::Mapping,
                TokenKind::Identifier,
                TokenKind::Separater,
                TokenKind::Identifier,
                TokenKind::Mapping,
                TokenKind::Identifier,
                TokenKind::Separater,
                TokenKind::Identifier,
                TokenKind::Mapping,
                TokenKind::Identifier,
                TokenKind::ParenthesisEnd,
                TokenKind::Allow,
                TokenKind::Identifier,
                TokenKind::BracketBegin,
                TokenKind::AtMark,
                TokenKind::Return,
                TokenKind::Identifier,
                TokenKind::AtMark,
                TokenKind::Spawn,
                TokenKind::Identifier,
                TokenKind::Mapping,
                TokenKind::Identifier,
                TokenKind::BracketBegin,
                TokenKind::Use,
                TokenKind::Identifier,
                TokenKind::Accessor,
                TokenKind::Identifier,
                TokenKind::Separater,
                TokenKind::Identifier,
                TokenKind::Accessor,
                TokenKind::Identifier,
                TokenKind::Semicolon,
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
            let mut tokenizer =Tokenizer::new(&text);
            assert!(tokenizer.expect(TokenKind::Allow).is_none());
        }

        #[test]
        fn request_all_ok() {
            let text = "data module cocoa @".to_string();
            let correct_token_kinds = [
                TokenKind::Data,
                TokenKind::Module,
                TokenKind::Identifier,
                TokenKind::AtMark
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
            tokenizer.request(TokenKind::AtMark);
        }
    }
}
