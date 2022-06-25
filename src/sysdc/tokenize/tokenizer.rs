pub struct Tokenizer<'a> {
    text: &'a String
}

impl<'a> Tokenizer<'a> {
    pub fn new(text: &'a String) -> Tokenizer {
        Tokenizer { text }
    }
}

#[cfg(test)]
mod test {
    use super::Tokenizer;

    #[test]
    pub fn create_tokenizer() {
        let text = "cocoa 410 cappuccino 1204".to_string();
        Tokenizer::new(&text);
    }
}
