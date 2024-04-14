use std::error::Error;
use std::fmt::{Debug, Display};
use std::iter::Peekable;
use std::slice::Iter;

use super::tokenizer::Token;

#[derive(Debug, PartialEq)]
pub enum ASTNode<'a> {
    Object(Vec<(&'a str, ASTNode<'a>)>),
    Array(Vec<ASTNode<'a>>),
    String(&'a str),
    Number(f64),
    Boolean(bool),
    Null,
}

pub fn parse<'a>(tokens: &mut Peekable<Iter<'a, Token>>) -> Result<(), ParseError> {
    if tokens.len() == 0 {
        return Ok(());
    }

    parse_value(tokens)?;

    Ok(())
}

fn parse_value<'a>(tokens: &mut Peekable<Iter<'a, Token>>) -> Result<ASTNode<'a>, ParseError> {
    // consume first token here
    let token = tokens
        .next()
        .ok_or(ParseError::new("Unexpected end of input".into()))?;
    match &token {
        Token::String(s) => Ok(ASTNode::String(s)),
        Token::Number(n) => Ok(ASTNode::Number(*n)),
        Token::True => Ok(ASTNode::Boolean(true)),
        Token::False => Ok(ASTNode::Boolean(false)),
        Token::Null => Ok(ASTNode::Null),
        Token::BraceOpen => parse_object(tokens),
        Token::BracketOpen => parse_array(tokens),
        _ => Err(ParseError::new("Unexpected token".into())),
    }
}

fn parse_object<'a>(tokens: &mut Peekable<Iter<'a, Token>>) -> Result<ASTNode<'a>, ParseError> {
    let mut node = ASTNode::Object(Vec::new());
    let mut is_first = true;
    let mut expect_next_value = false;

    loop {
        let token = tokens
            .next()
            .ok_or(ParseError::new("Unexpected end of input".into()))?;
        match token {
            // end of object
            Token::BraceClose => {
                if expect_next_value {
                    return Err(ParseError::new(
                        "Unexpected comma before end of object".into(),
                    ));
                }
                break;
            }
            // object key
            Token::String(s) => {
                // if not first key, expect comma before next key
                if !is_first && !expect_next_value {
                    return Err(ParseError::new("Missing comma".into()));
                }
                is_first = false;

                let token = tokens.next();
                if let Some(Token::Colon) = token {
                    // get the value of this key recursively
                    let value = parse_value(tokens);
                    match value {
                        // if value is parsed successfully, add it to the object with the key
                        Ok(v) => match &mut node {
                            ASTNode::Object(obj) => {
                                obj.push((s, v));
                                // if comma is after value, skip it and expect next value
                                if let Some(Token::Comma) = tokens.peek() {
                                    tokens.next();
                                    expect_next_value = true;
                                } else {
                                    expect_next_value = false;
                                }
                            }
                            _ => panic!("Should never happen!"),
                        },
                        Err(e) => return Err(e),
                    }
                } else {
                    return Err(ParseError::new("Expected colon after string key".into()));
                }
            }
            _ => return Err(ParseError::new("Unexpected object key".into())),
        }
    }

    Ok(node)
}

fn parse_array<'a>(tokens: &mut Peekable<Iter<'a, Token>>) -> Result<ASTNode<'a>, ParseError> {
    let mut node = ASTNode::Array(Vec::new());
    let mut is_first = true;
    let mut expect_next_value = false;

    loop {
        let token = tokens
            .peek()
            .ok_or(ParseError::new("Unexpected end of input".into()))?;
        match token {
            // end of array
            Token::BracketClose => {
                if expect_next_value {
                    return Err(ParseError::new(
                        "Unexpected comma before end of array".into(),
                    ));
                }
                tokens.next();
                break;
            }
            _ => {
                // if not first value, expect comma before next value
                if !is_first && !expect_next_value {
                    return Err(ParseError::new("Missing comma".into()));
                }
                is_first = true;

                // get the value of this array element recursively
                let value = parse_value(tokens);
                match value {
                    // if value is parsed successfully, add it to the array
                    Ok(v) => match &mut node {
                        ASTNode::Array(arr) => {
                            arr.push(v);
                            // if there is a comma after value, skip it and expect next value
                            if let Some(Token::Comma) = tokens.peek() {
                                tokens.next();
                                expect_next_value = true;
                            } else {
                                expect_next_value = false;
                            }
                        }
                        _ => panic!("Should never happen!"),
                    },
                    Err(e) => return Err(e),
                }
            }
        }
    }

    return Ok(node);
}

#[derive(Debug, Clone)]
pub struct ParseError {
    msg: String,
}

impl<'a> ParseError {
    pub fn new(msg: String) -> ParseError {
        ParseError { msg: msg }
    }
}

impl Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Invalid JSON: {}", self.msg)
    }
}

impl Error for ParseError {}

#[cfg(test)]
mod parser {
    use super::Token;
    use super::*;

    #[test]
    fn test_parse_simple_json() {
        let tokens = vec![
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
        ];
        let result = parse_tokens(tokens);
        assert!(result.is_ok());
    }

    #[test]
    fn test_parse_nested_json() {
        let tokens = vec![
            Token::BraceOpen,
            Token::String("key".to_string()),
            Token::Colon,
            Token::BraceOpen,
            Token::String("inner_key".to_string()),
            Token::Colon,
            Token::BracketOpen,
            Token::Number(1.0),
            Token::Comma,
            Token::Number(2.0),
            Token::BracketClose,
            Token::BraceClose,
            Token::BraceClose,
        ];
        let result = parse_tokens(tokens);
        assert!(result.is_ok());
    }

    #[test]
    fn test_parse_empty_object() {
        let tokens = vec![Token::BraceOpen, Token::BraceClose];
        let result = parse_tokens(tokens);
        assert!(result.is_ok());
    }

    #[test]
    fn test_parse_empty_array() {
        let tokens = vec![Token::BracketOpen, Token::BracketClose];
        let result = parse_tokens(tokens);
        assert!(result.is_ok());
    }

    #[test]
    fn test_parse_object() {
        let tokens = vec![
            // parse_object fn requires the first token (Token::BraceOpen) to be consumed
            // Token::BraceOpen,
            Token::String("key".to_string()),
            Token::Colon,
            Token::String("value".to_string()),
            Token::BraceClose,
        ];
        let result = parse_object(&mut tokens.iter().peekable());
        assert!(result.is_ok());
        assert_eq!(
            result.unwrap(),
            ASTNode::Object(vec![("key", ASTNode::String("value"))])
        );
    }

    #[test]
    fn test_parse_array() {
        let tokens = vec![
            // parse_array fn requires the first token (Token::BracketOpen) to be consumed
            // Token::BracketOpen,
            Token::Number(1.0),
            Token::Number(2.0),
            Token::BracketClose,
        ];
        let result = parse_array(&mut tokens.iter().peekable());
        assert!(result.is_ok());
        assert_eq!(
            result.unwrap(),
            ASTNode::Array(vec![ASTNode::Number(1.0), ASTNode::Number(2.0)])
        );
    }

    #[test]
    fn test_parse_invalid_trailing_comma() {
        let tokens = vec![
            Token::BraceOpen,
            Token::String("key".to_string()),
            Token::Colon,
            Token::String("value".to_string()),
            Token::Comma,
            Token::BraceClose,
        ];
        let result = parse_tokens(tokens);
        assert!(result.is_err());
        assert!(result.unwrap_err().msg == "Unexpected comma before end of object");

        let tokens = vec![
            Token::BracketOpen,
            Token::Number(1.0),
            Token::Comma,
            Token::Number(2.0),
            Token::Comma,
            Token::BracketClose,
        ];
        let result = parse_tokens(tokens);
        assert!(result.is_err());
        assert!(result.unwrap_err().msg == "Unexpected comma before end of array");
    }

    #[test]
    fn test_parse_invalid_key_type() {
        let tokens = vec![
            Token::BraceOpen,
            Token::Number(1.0),
            Token::Colon,
            Token::String("value".to_string()),
            Token::BraceClose,
        ];
        let result = parse_tokens(tokens);
        assert!(result.is_err());
        assert!(result.unwrap_err().msg == "Unexpected object key");

        let tokens = vec![
            Token::BraceOpen,
            Token::BracketOpen,
            Token::Colon,
            Token::String("value".to_string()),
            Token::BraceClose,
        ];
        let result = parse_tokens(tokens);
        assert!(result.is_err());
        assert!(result.unwrap_err().msg == "Unexpected object key");
    }

    #[test]
    fn test_parse_missing_colon() {
        let tokens = vec![
            Token::BraceOpen,
            Token::String("key".to_string()),
            Token::String("value".to_string()),
            Token::BraceClose,
        ];
        let result = parse_tokens(tokens);
        assert!(result.is_err());
        assert!(result.unwrap_err().msg == "Expected colon after string key");
    }

    #[test]
    fn test_parse_unexpected_end_of_input() {
        let tokens = vec![Token::BraceOpen];
        let result = parse_tokens(tokens);
        assert!(result.is_err());
        assert!(result.unwrap_err().msg == "Unexpected end of input");
    }

    fn parse_tokens(tokens: Vec<Token>) -> Result<(), ParseError> {
        let mut tokens_iter = tokens.iter().peekable();
        parse(&mut tokens_iter)
    }
}
