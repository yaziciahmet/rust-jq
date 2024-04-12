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
}

impl<'a> Tokenizer<'a> {
    pub fn new(contents: &str) -> Tokenizer {
        Tokenizer { contents, pos: 0 }
    }

    pub fn peek_char(&self) -> Option<char> {
        self.contents.chars().nth(self.pos)
    }

    fn next_char(&mut self) -> Option<char> {
        self.contents.chars().nth(self.pos).map(|c| {
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
                '"' => {
                    let mut s = String::new();
                    loop {
                        match self.next_char()? {
                            '"' => break,
                            c => s.push(c),
                        }
                    }
                    Some(Token::String(s))
                }
                't' => {
                    if self.next_char()? == 'r'
                        && self.next_char()? == 'u'
                        && self.next_char()? == 'e'
                    {
                        Some(Token::True)
                    } else {
                        None
                    }
                }
                'f' => {
                    if self.next_char()? == 'a'
                        && self.next_char()? == 'l'
                        && self.next_char()? == 's'
                        && self.next_char()? == 'e'
                    {
                        Some(Token::False)
                    } else {
                        None
                    }
                }
                'n' => {
                    if self.next_char()? == 'u'
                        && self.next_char()? == 'l'
                        && self.next_char()? == 'l'
                    {
                        Some(Token::Null)
                    } else {
                        None
                    }
                }
                '0'..='9' | '-' => {
                    let mut s = c.to_string();
                    loop {
                        match self.peek_char()? {
                            '0'..='9' | '.' | 'e' | 'E' | '+' | '-' => {
                                s.push(self.next_char()?);
                            }
                            _ => break,
                        }
                    }
                    let n = s.parse().ok()?;
                    Some(Token::Number(n))
                }
                _ => None,
            };
        }
    }
}

impl<'a> Iterator for Tokenizer<'a> {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        self.next_token()
    }
}

#[cfg(test)]
mod tokenizer {
    use super::*;

    #[test]
    fn test_simple_json() {
        let contents = r#"
            {
                "key": "value",
                "number": 42,
                "bool": true,
                "null": null,
                "array": [1, 2, 3]
            }
        "#;

        let tokenizer = Tokenizer::new(contents);
        let tokens: Vec<Token> = tokenizer.collect();
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
}
