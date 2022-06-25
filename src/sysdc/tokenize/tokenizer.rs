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
        let token = if self.hold_token.is_none() {
            match self.tokenize() {
                (Some(orig), _) => Token::from_string(orig),
                (_, Some(orig)) => Token::from_i32(orig),
                _ => panic!("[ERROR] Reading pointer reached end of text")
            }
        } else {
            self.hold_token.clone().unwrap()
        };

        if token.kind == kind {
            Some(token)
        } else {
            self.hold_token = Some(token);
            None
        }
    }

    fn tokenize(&mut self) -> (Option<String>, Option<i32>) {
        if !self.has_token() {
            return (None, None);
        }

        self.skip_space();

        let lead_ref_pos = self.now_ref_pos;
        let lead_c = self.text.chars().nth(lead_ref_pos).unwrap();
        let lead_type = CharType::from(lead_c);
        self.now_ref_pos += 1;

        while self.has_token() {
            let c = self.text.chars().nth(self.now_ref_pos).unwrap();
            match CharType::from(c) {
                CharType::Identifier => {
                    match lead_type {
                        CharType::Identifier => {},
                        _ => break
                    }
                },
                CharType::SymbolOne => break,
                CharType::SymbolTwo => {
                    match lead_type {
                        CharType::SymbolTwo => { self.now_ref_pos += 1; break },
                        _ => break
                    }
                },
                CharType::Number => {
                    match lead_type {
                        CharType::Identifier | CharType::Number => {},
                        _ => break
                    }
                },
                CharType::Space => break,
                CharType::Other => panic!("[ERROR] Dicover unregistered charactor.")
            };

            self.now_ref_pos += 1;
        }

        let (word_begin, _) = self.text.char_indices().nth(lead_ref_pos).unwrap();
        let discovered_word = if self.now_ref_pos != self.text.len() {
            let (word_end, _) = self.text.char_indices().nth(self.now_ref_pos).unwrap();
            self.text[word_begin..word_end].to_string()
        } else {
            self.text[word_begin..].to_string()
        };

        match lead_type {
            CharType::Number => (None, Some(discovered_word.parse::<i32>().unwrap())),
            _ => (Some(discovered_word), None)
        }
    }

    fn skip_space(&mut self) {
        loop {
            match self.text.chars().nth(self.now_ref_pos).unwrap() {
                ' ' | '\t' | '\n' => {},
                _ => break
            }
            self.now_ref_pos += 1;
        }
    }
}

enum CharType {
    Identifier,
    SymbolOne,
    SymbolTwo,
    Number,
    Space,
    Other
}

impl CharType {
    pub fn from(c: char) -> CharType {
        match c {
            'a'..='z' | 'A'..='Z' | '_' => CharType::Identifier,
            '=' | ':' | '.' | ',' | ';' | '{' | '}' | '(' | ')' => CharType::SymbolOne,
            '-' | '>' => CharType::SymbolTwo,
            '0'..='9' => CharType::Number,
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
    pub fn tokenize_text_identifier_and_symbol() {
        let text = "data User { id: int32, name: string } module UserModule binds User as this { greet(text: string) -> None { use = this.text; } }".to_string();
        let correct_tokens = [
            "data", "User", "{", "id", ":", "int32", ",", "name", ":", "string", "}",
            "module", "UserModule", "binds", "User", "as", "this", "{", "greet", "(", "text", ":", "string", ")", "->", "None", 
            "{", "use", "=", "this", ".", "text", ";", "}", "}"
        ];

        let mut tokenizer = Tokenizer::new(&text);
        for token in correct_tokens {
            assert_eq!(tokenizer.tokenize(), (Some(token.to_string()), None));
        }
    }

    #[test]
    pub fn tokenize_number() {
        let text = "0 1 2 3 410 1204".to_string();
        let correct_numbers = [0, 1, 2, 3, 410, 1204];

        let mut tokenizer = Tokenizer::new(&text);
        for number in correct_numbers {
            assert_eq!(tokenizer.tokenize(), (None, Some(number)));
        }
    }

    #[test]
    #[should_panic]
    pub fn tokenize_unregisterd_charactor() {
        let text = "aaaaaaa#".to_string();
        let mut tokenizer = Tokenizer::new(&text);
        assert_eq!(tokenizer.tokenize(), (None, None));
    }

    #[test]
    pub fn expect_kind_all_ok() {
        let text = "data User { id: int32, name: String }".to_string();
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
            TokenKind::BracketEnd
        ];

        let mut tokenizer = Tokenizer::new(&text);
        for token_kind in correct_token_kinds {
            match tokenizer.expect_kind(token_kind) {
                Some(_) => {}
                None => assert!(false)
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
                Some(_) => assert!(token_kind == TokenKind::Data),
                None => assert!(token_kind != TokenKind::Data)
            }
        }
        assert!(!tokenizer.has_token());
    }
}
