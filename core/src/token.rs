use std::str::Chars;

use super::error::{PError, PErrorKind};
use super::location::Location;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TokenKind {
    /* Reserved */
    Unit,   // unit
    From,   // from
    Import, // import
    Data,   // data
    Module, // module
    Func,   // func
    Proc,   // proc
    Return, // return
    Affect, // affect
    Modify, // modify
    Spawn,  // spawn
    Let,    // let
    Use,    // use

    /* Symbol */
    Allow,            // ->
    Mapping,          // :
    Equal,            // =
    Accessor,         // .
    Separater,        // ,
    Semicolon,        // ;
    ParenthesisBegin, // (
    ParenthesisEnd,   // )
    BracketBegin,     // {
    BracketEnd,       // }
    AtMark,           // @
    Plus,             // +

    /* Others */
    Identifier,
}

#[derive(Debug, Clone)]
pub struct Token {
    pub kind: TokenKind,
    pub orig: String,
    pub location: Location,
}

impl Token {
    pub fn new(orig: String, filename: String, row: i32, col: i32) -> Token {
        let kind = match orig.as_str() {
            "unit" => TokenKind::Unit,
            "from" => TokenKind::From,
            "import" => TokenKind::Import,
            "data" => TokenKind::Data,
            "module" => TokenKind::Module,
            "func" => TokenKind::Func,
            "proc" => TokenKind::Proc,
            "return" => TokenKind::Return,
            "affect" => TokenKind::Affect,
            "modify" => TokenKind::Modify,
            "spawn" => TokenKind::Spawn,
            "let" => TokenKind::Let,
            "use" => TokenKind::Use,
            "->" => TokenKind::Allow,
            ":" => TokenKind::Mapping,
            "=" => TokenKind::Equal,
            "." => TokenKind::Accessor,
            "," => TokenKind::Separater,
            ";" => TokenKind::Semicolon,
            "(" => TokenKind::ParenthesisBegin,
            ")" => TokenKind::ParenthesisEnd,
            "{" => TokenKind::BracketBegin,
            "}" => TokenKind::BracketEnd,
            "@" => TokenKind::AtMark,
            "+" => TokenKind::Plus,
            _ => TokenKind::Identifier,
        };
        let location = Location::new()
            .with_filename(filename)
            .with_coord((row, col - (orig.len() as i32)));
        Token {
            kind,
            orig,
            location,
        }
    }
}

#[derive(Debug)]
pub struct Tokenizer<'a> {
    chars: Chars<'a>,

    hold_char: Option<char>,
    hold_chars: Vec<char>,
    hold_token: Option<Token>,

    filename: String,
    now_ref_row: i32,
    now_ref_col: i32,
}

impl<'a> Tokenizer<'a> {
    pub fn new(filename: String, text: &'a str) -> Tokenizer<'a> {
        let mut tokenizer = Tokenizer {
            chars: text.chars(),
            hold_char: None,
            hold_chars: vec![],
            hold_token: None,
            filename,
            now_ref_row: 1,
            now_ref_col: 1,
        };
        tokenizer.skip_space();
        tokenizer
    }

    pub fn get_now_ref_loc(&mut self) -> Location {
        match &self.hold_token {
            Some(token) => token.location.clone(),
            None => Location::new()
                .with_filename(self.filename.clone())
                .with_coord((self.now_ref_row, self.now_ref_col)),
        }
    }

    pub fn exists_next(&mut self) -> bool {
        match self.hold_char {
            Some(c) => self.hold_char = Some(c),
            None => {
                self.hold_char = self.chars.next();
                self.now_ref_col += 1;
            }
        }
        self.hold_char.is_some()
    }

    pub fn expect(&mut self, kind: TokenKind) -> anyhow::Result<Option<Token>> {
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

    pub fn request(&mut self, kind: TokenKind) -> anyhow::Result<Token> {
        match self.expect(kind.clone())? {
            Some(token) => Ok(token),
            None => Err(PError::from(PErrorKind::RequestedTokenNotFound(kind))
                .with_loc(self.get_now_ref_loc())
                .into()),
        }
    }

    fn tokenize(&mut self) -> anyhow::Result<Option<Token>> {
        if self.hold_token.is_some() {
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
                (CharType::Identifier, CharType::Identifier | CharType::Number) => {}
                (CharType::Number, CharType::Number) => {}

                // Ok(force stop)
                (CharType::Symbol, _) => break,
                (CharType::SymbolAllow1, CharType::SymbolAllow2) => {
                    self.adopt()?;
                    break;
                }

                // Ng(panic)
                (CharType::SymbolAllow1 | CharType::SymbolAllow2, _) => {
                    return Err(PError::from(PErrorKind::FoundUnregisteredSymbol)
                        .with_loc(self.get_now_ref_loc())
                        .into())
                }

                // Ok(force stop)
                _ => break,
            }
            self.adopt()?;
        }
        self.skip_space();

        Ok(Some(Token::new(
            self.collect(),
            self.filename.to_string(),
            self.now_ref_row,
            self.now_ref_col,
        )))
    }

    fn adopt(&mut self) -> anyhow::Result<()> {
        match self.hold_char {
            Some(c) => self.hold_chars.push(c),
            None => {
                return Err(PError::from(PErrorKind::UnexpectedEOF)
                    .with_loc(self.get_now_ref_loc())
                    .into())
            }
        }
        self.hold_char = self.chars.next();
        self.now_ref_col += 1;
        Ok(())
    }

    fn collect(&mut self) -> String {
        let result = self.hold_chars.iter().collect::<String>();
        self.hold_chars = vec![];
        result
    }

    fn skip_space(&mut self) {
        let mut comment = false;
        while self.exists_next() {
            match CharType::from(self.hold_char.unwrap()) {
                CharType::Space => {}
                CharType::NewLine => {
                    self.now_ref_row += 1;
                    self.now_ref_col = 1;
                }
                CharType::Comment => {
                    comment = !comment;
                }
                _ => {
                    if !comment {
                        break;
                    }
                }
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
    SymbolAllow1,
    SymbolAllow2,

    Comment,
    Space,
    NewLine,

    Other,
}

impl From<char> for CharType {
    fn from(c: char) -> CharType {
        match c {
            '0'..='9' => CharType::Number,
            'a'..='z' | 'A'..='Z' | '_' => CharType::Identifier,

            '=' | '.' | ',' | ';' | '{' | '}' | '(' | ')' | ':' => CharType::Symbol,
            '-' => CharType::SymbolAllow1,
            '>' => CharType::SymbolAllow2,

            '%' => CharType::Comment,
            ' ' | '\t' | '\r' => CharType::Space,
            '\n' => CharType::NewLine,

            _ => CharType::Other,
        }
    }
}

#[cfg(test)]
mod test {
    mod token {
        use super::super::{Token, TokenKind};

        #[test]
        fn create_token_from() {
            let str_kind_mapping = [
                ("unit", TokenKind::Unit),
                ("from", TokenKind::From),
                ("import", TokenKind::Import),
                ("data", TokenKind::Data),
                ("module", TokenKind::Module),
                ("return", TokenKind::Return),
                ("affect", TokenKind::Affect),
                ("modify", TokenKind::Modify),
                ("spawn", TokenKind::Spawn),
                ("let", TokenKind::Let),
                ("use", TokenKind::Use),
                ("->", TokenKind::Allow),
                (":", TokenKind::Mapping),
                ("=", TokenKind::Equal),
                (".", TokenKind::Accessor),
                (",", TokenKind::Separater),
                (";", TokenKind::Semicolon),
                ("(", TokenKind::ParenthesisBegin),
                (")", TokenKind::ParenthesisEnd),
                ("{", TokenKind::BracketBegin),
                ("}", TokenKind::BracketEnd),
                ("@", TokenKind::AtMark),
                ("+", TokenKind::Plus),
            ];
            for (_str, kind) in str_kind_mapping {
                assert_eq!(
                    Token::new(_str.to_string(), "test.def".to_string(), 0, 0).kind,
                    kind
                );
            }
        }

        #[test]
        fn get_identifer_from_identifer_token() {
            let id = Token::new("test".to_string(), "test.def".to_string(), 0, 0).orig;
            assert_eq!(id, "test");
        }
    }

    mod tokenizer {
        use super::super::{TokenKind, Tokenizer};

        #[test]
        pub fn create_tokenizer() {
            let text = "cocoa 410 cappuccino 1204".to_string();
            Tokenizer::new("test.def".to_string(), &text);
        }

        #[test]
        fn expect_all_ok() {
            let text = "
                unit box;

                from std import IO;
                from square import Square;

                data Box {
                    x: i32,
                    y: i32  % Single Comment %
                }

                module BoxModule {
                    func move(box: Box, dx: i32, dy: i32) -> Box {
                        @return movedBox

                        %
                            MultiLine
                            Comment
                            あああ？
                        %

                        \n \r\n \r\n \n

                        @affect IO.stdout(box)

                        @modify box {
                            use dx, dy;
                        }

                        @spawn movedBox: Box {
                            use box.x, box.y;
                        }
                    }
                }"
            .to_string();
            let correct_token_kinds = [
                TokenKind::Unit,
                TokenKind::Identifier,
                TokenKind::Semicolon,
                TokenKind::From,
                TokenKind::Identifier,
                TokenKind::Import,
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
                TokenKind::Func,
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
                TokenKind::Affect,
                TokenKind::Identifier,
                TokenKind::Accessor,
                TokenKind::Identifier,
                TokenKind::ParenthesisBegin,
                TokenKind::Identifier,
                TokenKind::ParenthesisEnd,
                TokenKind::AtMark,
                TokenKind::Modify,
                TokenKind::Identifier,
                TokenKind::BracketBegin,
                TokenKind::Use,
                TokenKind::Identifier,
                TokenKind::Separater,
                TokenKind::Identifier,
                TokenKind::Semicolon,
                TokenKind::BracketEnd,
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
                TokenKind::BracketEnd,
            ];

            let mut tokenizer = Tokenizer::new("test.def".to_string(), &text);
            for token_kind in correct_token_kinds {
                match tokenizer.expect(token_kind.clone()).unwrap() {
                    Some(_) => {}
                    None => panic!("{:?}", token_kind),
                }
            }
        }

        #[test]
        fn expect_all_ng() {
            let text = "data".to_string();
            let mut tokenizer = Tokenizer::new("test.def".to_string(), &text);
            assert!(tokenizer.expect(TokenKind::Allow).unwrap().is_none());
        }

        #[test]
        fn request_all_ok() {
            let text = "data module cocoa @".to_string();
            let correct_token_kinds = [
                TokenKind::Data,
                TokenKind::Module,
                TokenKind::Identifier,
                TokenKind::AtMark,
            ];

            let mut tokenizer = Tokenizer::new("test.def".to_string(), &text);
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
            let mut tokenizer = Tokenizer::new("test.def".to_string(), &text);
            tokenizer.request(TokenKind::AtMark).unwrap();
        }
    }
}
