use std::error::Error;
use std::fmt::{Debug, Display};

#[derive(Debug, PartialEq)]
pub enum Token {
    BraceOpen,
    BraceClose,
    BracketOpen,
    BracketClose,
    Colon,
    Comma,
    String(String),
    Number(f64),
    True,
    False,
    Null,
}

pub struct Tokenizer<'a> {
    contents: &'a str,
    pos: usize,
    error: Option<TokenError>,
}

impl<'a> Tokenizer<'a> {
    pub fn new(contents: &str) -> Tokenizer {
        Tokenizer {
            contents,
            pos: 0,
            error: None,
        }
    }

    fn peek_nth_char(&self, n: usize) -> Option<char> {
        self.contents.chars().nth(n)
    }

    fn peek_char(&self) -> Option<char> {
        self.peek_nth_char(self.pos)
    }

    fn next_char(&mut self) -> Option<char> {
        self.peek_char().map(|c| {
            self.pos += 1;
            c
        })
    }

    fn next_token(&mut self) -> Option<Token> {
        loop {
            let c = self.next_char()?;
            if c.is_whitespace() || c.is_control() {
                continue;
            }

            return match c {
                '{' => Some(Token::BraceOpen),
                '}' => Some(Token::BraceClose),
                '[' => Some(Token::BracketOpen),
                ']' => Some(Token::BracketClose),
                ':' => Some(Token::Colon),
                ',' => Some(Token::Comma),
                '"' => self.read_string(),
                't' => self.read_bool_true(),
                'f' => self.read_bool_false(),
                'n' => self.read_null(),
                '0'..='9' | '-' => self.read_number(c),
                _ => {
                    self.set_error();
                    None
                }
            };
        }
    }

    fn read_string(&mut self) -> Option<Token> {
        let mut s = String::new();
        let mut peek_pos = self.pos;
        loop {
            let c = self.peek_nth_char(peek_pos);
            match c {
                // no multiline strings allowed
                Some('\n') | Some('\r') | None => {
                    self.set_error();
                    return None;
                }
                Some('"') => break,
                Some(c) => {
                    s.push(c);
                    peek_pos += 1;
                }
            }
        }
        self.pos = peek_pos + 1;
        Some(Token::String(s))
    }

    fn read_bool_true(&mut self) -> Option<Token> {
        if self.peek_char() == Some('r')
            && self.peek_nth_char(self.pos + 1) == Some('u')
            && self.peek_nth_char(self.pos + 2) == Some('e')
        {
            self.pos += 3;
            Some(Token::True)
        } else {
            self.set_error();
            None
        }
    }

    fn read_bool_false(&mut self) -> Option<Token> {
        if self.peek_char() == Some('a')
            && self.peek_nth_char(self.pos + 1) == Some('l')
            && self.peek_nth_char(self.pos + 2) == Some('s')
            && self.peek_nth_char(self.pos + 3) == Some('e')
        {
            self.pos += 4;
            Some(Token::False)
        } else {
            self.set_error();
            None
        }
    }

    fn read_null(&mut self) -> Option<Token> {
        if self.peek_char() == Some('u')
            && self.peek_nth_char(self.pos + 1) == Some('l')
            && self.peek_nth_char(self.pos + 2) == Some('l')
        {
            self.pos += 3;
            Some(Token::Null)
        } else {
            self.set_error();
            None
        }
    }

    fn read_number(&mut self, first: char) -> Option<Token> {
        let mut s = first.to_string();
        let mut peek_pos = self.pos;
        loop {
            let c = self.peek_nth_char(peek_pos);
            match c {
                Some(c) => match c {
                    '0'..='9' | '.' | 'e' | 'E' | '+' | '-' => {
                        s.push(c);
                        peek_pos += 1;
                    }
                    _ => break,
                },
                None => break,
            }
        }
        // rust parser allows trailing dot, but it is invalid JSON
        if s.ends_with('.') {
            self.set_error();
            return None;
        }
        // rust parser allows prefix zero (e.g. 01), but it is invalid JSON
        if s.len() > 1 && s.starts_with('0') && !s.starts_with("0.") && !s.starts_with("0e") {
            self.set_error();
            return None;
        }

        if let Ok(n) = s.parse() {
            self.pos = peek_pos;
            Some(Token::Number(n))
        } else {
            self.set_error();
            None
        }
    }

    pub fn try_collect(mut self) -> Result<Vec<Token>, TokenError> {
        let mut tokens = Vec::new();
        loop {
            match self.next_token() {
                Some(token) => {
                    tokens.push(token);
                }
                None => break,
            }
        }

        if let Some(error) = self.error {
            Err(error)
        } else {
            Ok(tokens)
        }
    }

    fn set_error(&mut self) {
        // first character is always consumed, so we need to subtract 1
        self.error = Some(TokenError::new(self.pos - 1));
    }
}

impl<'a> Iterator for Tokenizer<'a> {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        self.next_token()
    }
}

#[derive(Debug)]
pub struct TokenError {
    start_pos: usize,
}

impl TokenError {
    pub fn new(start_pos: usize) -> TokenError {
        TokenError { start_pos }
    }
}

impl Display for TokenError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Unexpected token at position {}", self.start_pos)
    }
}

impl Error for TokenError {}

#[cfg(test)]
mod tokenizer {
    use super::*;

    #[test]
    fn test_tokenize_simple_json() {
        let contents = r#"
            {
                "key": "value",
                "number": 42,
                "bool": true,
                "null": null,
                "array": [1, 2, 3]
            }
        "#;
        let tokens = must_parse_tokens(contents);
        assert_eq!(
            tokens,
            vec![
                Token::BraceOpen,
                Token::String("key".to_string()),
                Token::Colon,
                Token::String("value".to_string()),
                Token::Comma,
                Token::String("number".to_string()),
                Token::Colon,
                Token::Number(42.0),
                Token::Comma,
                Token::String("bool".to_string()),
                Token::Colon,
                Token::True,
                Token::Comma,
                Token::String("null".to_string()),
                Token::Colon,
                Token::Null,
                Token::Comma,
                Token::String("array".to_string()),
                Token::Colon,
                Token::BracketOpen,
                Token::Number(1.0),
                Token::Comma,
                Token::Number(2.0),
                Token::Comma,
                Token::Number(3.0),
                Token::BracketClose,
                Token::BraceClose,
            ]
        );
    }

    #[test]
    fn test_valid_tokens_invalid_json() {
        let contents = r#"
            {
                "key", "value", [], {, 123 true
            }
        "#;
        let tokens = must_parse_tokens(contents);
        assert_eq!(
            tokens,
            vec![
                Token::BraceOpen,
                Token::String("key".to_string()),
                Token::Comma,
                Token::String("value".to_string()),
                Token::Comma,
                Token::BracketOpen,
                Token::BracketClose,
                Token::Comma,
                Token::BraceOpen,
                Token::Comma,
                Token::Number(123.0),
                Token::True,
                Token::BraceClose,
            ]
        );
    }

    #[test]
    fn test_invalid_token_in_json() {
        let contents = r#"
            {
                "key": "value",
                "number": 42,
                "bool": true,
                "null": null,
                "array": [1, 2, 3],
                extra
            }
        "#;
        let err = must_parse_with_error(contents);
        assert_eq!(err.start_pos, contents.find("extra").unwrap());
    }

    #[test]
    fn test_invalid_bool() {
        let contents = "tru";
        let err = must_parse_with_error(contents);
        assert_eq!(err.start_pos, 0);

        let contents = "fals";
        let err = must_parse_with_error(contents);
        assert_eq!(err.start_pos, 0);
    }

    #[test]
    fn test_invalid_str() {
        let contents = r#""abc"#;
        let err = must_parse_with_error(contents);
        assert_eq!(err.start_pos, 0);

        let contents = r#""abc
        ""#;
        let err = must_parse_with_error(contents);
        assert_eq!(err.start_pos, 0);
    }

    #[test]
    fn test_invalid_number() {
        let contents = "1.";
        let err = must_parse_with_error(contents);
        assert_eq!(err.start_pos, 0);

        let contents = "1.1e";
        let err = must_parse_with_error(contents);
        assert_eq!(err.start_pos, 0);

        let contents = "01";
        let err = must_parse_with_error(contents);
        assert_eq!(err.start_pos, 0);
    }

    fn must_parse_tokens(contents: &str) -> Vec<Token> {
        let tokenizer = Tokenizer::new(contents);
        tokenizer.try_collect().expect("Failed to collect tokens")
    }

    fn must_parse_with_error(contents: &str) -> TokenError {
        let tokenizer = Tokenizer::new(contents);
        tokenizer.try_collect().expect_err("Expected error")
    }
}
