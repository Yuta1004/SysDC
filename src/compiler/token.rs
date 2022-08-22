use std::error::Error;

use super::error::{ CompileError, CompileErrorKind };

#[derive(Debug, Clone, PartialEq)]
pub enum TokenKind {
    /* Reserved */
    Unit,               // unit
    From,               // from
    Import,             // import
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
    pub row: i32,
    pub col: i32,
    orig: Option<String>,
}

impl Token {
    pub fn from(orig: String, row: i32, col: i32) -> Token {
        let kind = match orig.as_str() {
            "unit"      => TokenKind::Unit,
            "from"      => TokenKind::From,
            "import"    => TokenKind::Import,
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
        let col = col-(orig.len() as i32);
        let orig = match kind {
            TokenKind::Identifier => Some(orig),
            _ => None
        };

        Token { kind, row, col, orig }
    }

    pub fn get_id(&self) -> Result<String, Box<dyn Error>> {
        match &self.orig {
            Some(id) => Ok(id.clone()),
            None => panic!("Internal Error")
        }
    }
}

#[derive(Debug)]
pub struct Tokenizer<'a> {
    text: &'a String,
    hold_token: Option<Token>,
    now_ref_pos: usize,
    now_ref_row: i32,
    now_ref_col: i32
}

impl<'a> Tokenizer<'a> {
    pub fn new(text: &'a String) -> Tokenizer<'a> {
        let mut tokenizer = Tokenizer {
            text,
            hold_token: None,
            now_ref_pos: 0,
            now_ref_row: 1,
            now_ref_col: 1
        };
        tokenizer.skip_space();
        tokenizer
    }

    pub fn has_token(&self) -> bool {
        self.now_ref_pos != self.text.len()
    }

    pub fn expect(&mut self, kind: TokenKind) -> Result<Option<Token>, Box<dyn Error>> {
        if let Some(token) = self.tokenize()? {
            if token.kind == kind {
                self.hold_token = None;
                Ok(Some(token))
            } else {
                self.hold_token = Some(token);
                Ok(None)
            }
        } else {
            Ok(None)
        }
    }

    pub fn request(&mut self, kind: TokenKind) -> Result<Token, Box<dyn Error>> {
        match self.expect(kind.clone())? {
            Some(token) => Ok(token),
            None => CompileError::new_with_pos(
                CompileErrorKind::RequestedTokenNotFound(kind),
                (self.now_ref_row, self.now_ref_col)
            )
        }
    }

    fn tokenize(&mut self) -> Result<Option<Token>, Box<dyn Error>> {
        if !self.hold_token.is_none() {
            return Ok(self.hold_token.clone());
        }
        if !self.has_token() {
            return Ok(None);
        }

        let lead_ref_pos = self.now_ref_pos;
        let lead_type = CharType::from(self.get_char_at(lead_ref_pos));
        self.now_ref_pos += 1;
        self.now_ref_col += 1;

        while self.has_token() {
            let now_type = CharType::from(self.get_char_at(self.now_ref_pos));
            match (&lead_type, now_type) {
                // Ok(continue)
                (CharType::Identifier, CharType::Identifier | CharType::Number) => {},
                (CharType::Number, CharType::Number) => {},
                
                // Ok(force stop)
                (CharType::Symbol, _) => break,
                (CharType::SymbolAllow1, CharType::SymbolAllow2) => {
                    self.now_ref_pos += 1;
                    self.now_ref_col += 1;
                    break;
                },

                // Ng(panic)
                (CharType::SymbolAllow1 | CharType::SymbolAllow2, _) =>
                    return CompileError::new_with_pos(
                        CompileErrorKind::FoundUnregisteredSymbol,
                        (self.now_ref_row, self.now_ref_col)
                    ),

                // Ok(force stop)
                _ => break
            }

            self.now_ref_pos += 1;
            self.now_ref_col += 1;
        }

        let discovered_word = self.clip_text(lead_ref_pos, self.now_ref_pos);
        let token = Token::from(discovered_word, self.now_ref_row, self.now_ref_col);
        self.skip_space();
        Ok(Some(token))
    }

    fn skip_space(&mut self) {
        loop {
            if !self.has_token() {
                break;
            }
            match CharType::from(self.get_char_at(self.now_ref_pos)) {
                CharType::Space => {
                    self.now_ref_pos += 1;
                    self.now_ref_col += 1;
                }
                CharType::NewLine => {
                    self.now_ref_pos += 1;
                    self.now_ref_row += 1;
                    self.now_ref_col = 1;
                }
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
    NewLine,
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
            ' ' | '\t'  => CharType::Space,
            '\n' => CharType::NewLine,
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
                ("unit",    TokenKind::Unit),
                ("from",    TokenKind::From),
                ("import",  TokenKind::Import),
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
                assert_eq!(Token::from(_str.to_string(), 0, 0).kind, kind);
            }
        }

        #[test]
        #[should_panic]
        fn get_identifer_from_reserved_token() {
            Token::from("->".to_string(), 0, 0).get_id().unwrap();
        }

        #[test]
        fn get_identifer_from_identifer_token() {
            let id = Token::from("test".to_string(), 0, 0).get_id().unwrap();
            assert_eq!(id, "test");
            Token::from("test".to_string(), 0, 0).get_id().unwrap();
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
                unit box;

                from square import Square;

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
                TokenKind::Unit,
                TokenKind::Identifier,
                TokenKind::Semicolon,
                TokenKind::From,
                TokenKind::Identifier,
                TokenKind::Import,
                TokenKind::Identifier,
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
                match tokenizer.expect(token_kind.clone()).unwrap() {
                    Some(_) => {}
                    None => assert!(false, "{:?}", token_kind)
                }
            }
        }

        #[test]
        fn expect_all_ng() {
            let text = "data".to_string();
            let mut tokenizer =Tokenizer::new(&text);
            assert!(tokenizer.expect(TokenKind::Allow).unwrap().is_none());
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
                let token = tokenizer.request(token_kind.clone()).unwrap();
                assert_eq!(token.kind, token_kind);
            }
            assert!(!tokenizer.has_token());
        }

        #[test]
        #[should_panic]
        fn request_ng() {
            let text = "data".to_string();
            let mut tokenizer = Tokenizer::new(&text);
            tokenizer.request(TokenKind::AtMark).unwrap();
        }
    }
}
