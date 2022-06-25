use super::token::{ Token, TokenKind };

pub struct Tokenizer<'a> {
    text: &'a String,
    now_ref_pos: usize,
    hold_token: Option<Token>
}

impl<'a> Tokenizer<'a> {
    pub fn new(text: &'a String) -> Tokenizer<'a> {
        Tokenizer {
            text,
            now_ref_pos: 0,
            hold_token: None
        }
    }

    pub fn has_token(&self) -> bool {
        self.now_ref_pos != self.text.len()
    }

    pub fn expect_kind(&mut self, kind: TokenKind) -> Option<Token> {
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

    fn tokenize(&mut self) -> Option<Token> {
        if !self.hold_token.is_none() {
            return self.hold_token.clone();
        }
        if !self.has_token() {
            return None;
        }

        self.skip_space();

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
                (CharType::SymbolOne, _) => break,
                (CharType::SymbolTwo1, CharType::SymbolTwo2) => { self.now_ref_pos += 1; break },

                // Ng(panic)
                (CharType::SymbolTwo1 | CharType::SymbolTwo2, _) => panic!("[ERROR] Discovered unregistered symbol."),

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
        Some(token)
    }

    fn skip_space(&mut self) {
        loop {
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

enum CharType {
    Number,
    Identifier,
    SymbolOne,
    SymbolTwo1,
    SymbolTwo2,
    Space,
    Other
}

impl CharType {
    pub fn from(c: char) -> CharType {
        match c {
            '0'..='9' => CharType::Number,
            'a'..='z' | 'A'..='Z' | '_' => CharType::Identifier,
            '=' | ':' | '.' | ',' | ';' | '{' | '}' | '(' | ')' => CharType::SymbolOne,
            '-' => CharType::SymbolTwo1,
            '>' => CharType::SymbolTwo2,
            ' ' | '\t' | '\n' => CharType::Space,
            _ => CharType::Other
        }
    }
}

#[cfg(test)]
mod test {
    use super::Tokenizer;
    use super::super::token::TokenKind;

    #[test]
    pub fn create_tokenizer() {
        let text = "cocoa 410 cappuccino 1204".to_string();
        Tokenizer::new(&text);
    }

    #[test]
    pub fn expect_kind_all_ok() {
        let text = "layer 0; data User { id: int32, name: String } module UserModule binds User as this { greet() -> None { use = this.name } }".to_string();
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
            TokenKind::Identifier,
            TokenKind::Accessor,
            TokenKind::Identifier,
            TokenKind::BracketEnd,
            TokenKind::BracketEnd
        ];

        let mut tokenizer = Tokenizer::new(&text);
        for token_kind in correct_token_kinds {
            match tokenizer.expect_kind(token_kind.clone()) {
                Some(_) => {}
                None => assert!(false, "{:?}", token_kind)
            }
        }
        assert!(!tokenizer.has_token());
    }

    #[test]
    pub fn expect_kind_all_ng() {
        let text = "data".to_string();

        let mut tokenizer = Tokenizer::new(&text);
        for token_kind in [TokenKind::Layer, TokenKind::Ref, TokenKind::Data] {
            match tokenizer.expect_kind(token_kind.clone()) {
                Some(_) => assert_eq!(token_kind, TokenKind::Data),
                None => assert_ne!(token_kind, TokenKind::Data)
            }
        }
        assert!(!tokenizer.has_token());
    }
}
