use std::error::Error;
use std::fmt::{Debug, Display};
use std::iter::Peekable;
use std::slice::Iter;

use crate::json::tokenizer::Token;

#[derive(Debug, Clone)]
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

    return Ok(node);
}

fn parse_array<'a>(tokens: &mut Peekable<Iter<'a, Token>>) -> Result<ASTNode<'a>, ParseError> {
    let mut node = ASTNode::Array(Vec::new());
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
                // get the value of this array element recursively
                let value = parse_value(tokens);
                match value {
                    // if value is parsed successfully, add it to the array
                    Ok(v) => match &mut node {
                        ASTNode::Array(arr) => {
                            arr.push(v.clone());
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

// #[cfg(test)]
// mod parser {
//     use super::*;
//     use crate::json::tokenizer::Token;

//     #[test]
//     fn test_parse_empty_object() {
//         let tokens = vec![Token::BraceOpen, Token::BraceClose];
//         let mut tokens_iter = tokens.iter().peekable();
//         let result = parse(&mut tokens_iter);
//         assert!(result.is_ok());
//     }
// }
