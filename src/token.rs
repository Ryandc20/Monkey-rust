// Contains the definition of diffrent tokens
use std::collections::HashMap;

#[derive(Clone, Copy)]
#[derive(PartialEq, Eq)]
#[derive(Debug)]
pub enum TokenType {
    ILLEGAL,
    EOF,

    IDENT,
    INT,

    ASSIGN, 
    PLUS,
    MINUS,
    BANG,
    ASTERISK,
    SLASH,

    LT,
    GT,

    EQ,
    NOTEQ,

    COMMA,
    SEMICOLON,

    LPAREN,
    RPAREN,
    LBRACE,
    RBRACE,

    FUNCTION,
    LET,
    TRUE,
    FALSE,
    IF,
    ELSE,
    RETURN,
}

pub struct Token {
    pub token_type: TokenType, 
    pub literal: String,
}


/// Takes a string that is either a keyword or identifier. An will return its respective token
/// type
pub fn keywords(ident: &String) -> TokenType {
    let keywords: HashMap<&str, TokenType> = HashMap::from([
        ("fn", TokenType::FUNCTION),
        ("let", TokenType::LET),
        ("true", TokenType::TRUE),
        ("false", TokenType::FALSE),
        ("if", TokenType::IF),
        ("else", TokenType::ELSE),
        ("return", TokenType::RETURN),
    ]);
     
    if keywords.contains_key(ident.as_str()) {
        return keywords[&ident.as_str()];
    }

    // If not a keyword should be of type identifier
    TokenType::IDENT
}
