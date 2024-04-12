use std::error::Error;
use std::fmt::{Debug, Display};
use std::iter::Peekable;
use std::slice::Iter;

use crate::json::tokenizer::Token;

#[derive(Debug, Clone)]
pub enum ASTNode {
    Object(Vec<(String, ASTNode)>),
    Array(Vec<ASTNode>),
    String(String),
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

fn parse_value<'a>(tokens: &mut Peekable<Iter<'a, Token>>) -> Result<ASTNode, ParseError> {
    let token = tokens.next();
    if let Some(t) = token {
        match &t {
            Token::String(s) => Ok(ASTNode::String(s.clone())),
            Token::Number(n) => Ok(ASTNode::Number(*n)),
            Token::True => Ok(ASTNode::Boolean(true)),
            Token::False => Ok(ASTNode::Boolean(false)),
            Token::Null => Ok(ASTNode::Null),
            Token::BraceOpen => parse_object(tokens),
            Token::BracketOpen => parse_array(tokens),
            _ => Err(ParseError::new("Unexpected token".into())),
        }
    } else {
        Err(ParseError::new("Unexpected end of input".into()))
    }
}

fn parse_object<'a>(tokens: &mut Peekable<Iter<'a, Token>>) -> Result<ASTNode, ParseError> {
    let mut node = ASTNode::Object(Vec::new());
    let mut expect_next_value = false;

    loop {
        let mut token = tokens.next();
        if let Some(t) = token {
            match t {
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
                    token = tokens.next();
                    if let Some(Token::Colon) = token {
                        // get the value of this key recursively
                        let value = parse_value(tokens);
                        match value {
                            // if value is parsed successfully, add it to the object with the key
                            Ok(v) => match &node {
                                ASTNode::Object(obj) => {
                                    let mut obj = obj.clone();
                                    obj.push((s.clone(), v));
                                    node = ASTNode::Object(obj);
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
        } else {
            return Err(ParseError::new("Unexpected end of input".into()));
        }
    }

    return Ok(node);
}

fn parse_array<'a>(tokens: &mut Peekable<Iter<'a, Token>>) -> Result<ASTNode, ParseError> {
    let mut node = ASTNode::Array(Vec::new());
    let mut expect_next_value = false;

    loop {
        let token = tokens.peek();
        if let Some(t) = token {
            match t {
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
                        Ok(v) => match &node {
                            ASTNode::Array(arr) => {
                                let mut arr = arr.clone();
                                arr.push(v.clone());
                                node = ASTNode::Array(arr);
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
        } else {
            return Err(ParseError::new("Unexpected end of input".into()));
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
