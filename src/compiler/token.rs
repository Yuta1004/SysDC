use std::str::Chars;
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
    ArrayBegin,         // [
    ArrayEnd,           // ]
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
            "["         => TokenKind::ArrayBegin,
            "]"         => TokenKind::ArrayEnd,
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
    chars: Chars<'a>,
    hold_char: Option<char>,
    hold_chars: Vec<char>,
    hold_token: Option<Token>,
    now_ref_row: i32,
    now_ref_col: i32
}

impl<'a> Tokenizer<'a> {
    pub fn new(text: &'a String) -> Tokenizer<'a> {
        let mut tokenizer = Tokenizer {
            chars: text.chars(),
            hold_char: None,
            hold_chars: vec!(),
            hold_token: None,
            now_ref_row: 1,
            now_ref_col: 1
        };
        tokenizer.skip_space();
        tokenizer
    }

    pub fn exists_next(&mut self) -> bool {
        match self.hold_char {
            c@Some(_) => self.hold_char = c,
            None => {
                self.hold_char = self.chars.next();
                self.now_ref_col += 1;
            }
        }
        self.hold_char.is_some()
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
        if !self.exists_next() {
            return Ok(None);
        }

        let lead_type = CharType::from(self.hold_char.unwrap());
        self.adopt()?;
        while self.exists_next() {
            match (&lead_type, CharType::from(self.hold_char.unwrap())) {
                // Ok(continue)
                (CharType::Identifier, CharType::Identifier | CharType::Number) => {},
                (CharType::Number, CharType::Number) => {},

                // Ok(force stop)
                (CharType::Symbol, _) => break,
                (CharType::SymbolArray1 | CharType::SymbolArray2, _) => break,
                (CharType::SymbolAllow1, CharType::SymbolAllow2) => { self.adopt()?; break }

                // Ng(panic)
                (CharType::SymbolAllow1 | CharType::SymbolAllow2, _) =>
                    return CompileError::new_with_pos(
                        CompileErrorKind::FoundUnregisteredSymbol,
                        (self.now_ref_row, self.now_ref_col)
                    ),

                // Ok(force stop)
                _ => break
            }
            self.adopt()?;
        }
        self.skip_space();
        Ok(Some(Token::from(self.collect(), self.now_ref_row, self.now_ref_col)))
    }

    fn adopt(&mut self) -> Result<(), Box<dyn Error>> {
        match self.hold_char {
            Some(c) => self.hold_chars.push(c),
            None => return CompileError::new(CompileErrorKind::UnexpectedEOF)
        }
        self.hold_char = self.chars.next();
        self.now_ref_col += 1;
        Ok(())
    }

    fn collect(&mut self) -> String {
        let result = self.hold_chars.iter().collect::<String>();
        self.hold_chars = vec!();
        result
    }

    fn skip_space(&mut self) {
        let mut comment = false;
        while self.exists_next() {
            match CharType::from(self.hold_char.unwrap()) {
                CharType::Space => {
                    self.now_ref_col += 1;
                }
                CharType::NewLine => {
                    self.now_ref_row += 1;
                    self.now_ref_col = 1;
                }
                CharType::Comment => {
                    comment = !comment;
                    self.now_ref_col += 1;
                }
                _ => if !comment { break }
            }
            self.hold_char = None;
        }
    }
}

#[derive(Debug)]
enum CharType {
    Number,
    Identifier,

    Symbol,
    SymbolArray1,
    SymbolArray2,
    SymbolAllow1,
    SymbolAllow2,

    Comment,
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
            '[' => CharType::SymbolArray1,
            ']' => CharType::SymbolArray2,
            '-' => CharType::SymbolAllow1,
            '>' => CharType::SymbolAllow2,

            '%' => CharType::Comment,
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
                    y: i32  % Single Comment %
                }

                module BoxModule {
                    move(box: Box, dx: i32, dy: i32) -> Box {
                        @return movedBox

                        %
                            MultiLine
                            Comment
                            あああ？
                        %

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
            assert!(!tokenizer.exists_next());
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
