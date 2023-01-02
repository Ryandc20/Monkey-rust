use crate::token::*;
use std::str::from_utf8;

pub struct Lexer {
    input: Vec<u8>,         // Represent the input text
    position: usize,        // Current position in input location of char
    read_position: usize,   // Current position to read from 
    ch: Option<u8>,                 // Current character
}

impl Lexer {
    // Main interface functions
    
    /// Creates a new lexer from on a string
    pub fn new(input: String) -> Self {
        let mut lexer = Self{ 
            input: input.as_bytes().to_vec(), 
            position: 0, read_position: 0, 
            ch: None,
        }; 
        lexer.read_char();
        lexer   
    }
    
    /// Gets the next token from the string
    pub fn next_token(&mut self) -> Token {
        // Eat the whitespace 
        self.skip_whitespace();
        
        let tok: Token = match self.ch {
            Some(x) => {
                match x as char {
                    '=' => {
                        if self.peek_char() == '=' as u8 {
                            let ch = self.ch.unwrap();
                            self.read_char();
                            let literal = format!("{}{}", ch as char, self.ch.unwrap() as char);
                            Token { token_type: TokenType::EQ, literal }
                        } else {
                            new_token(TokenType::ASSIGN, self.ch.unwrap())
                        }
                    },
                    '+' => new_token(TokenType::PLUS, self.ch.unwrap()),
                    '/' => new_token(TokenType::SLASH, self.ch.unwrap()),
                    '*' => new_token(TokenType::ASTERISK, self.ch.unwrap()),
                    '<' => new_token(TokenType::LT, self.ch.unwrap()),
                    '>' => new_token(TokenType::GT, self.ch.unwrap()),
                    ';' => new_token(TokenType::SEMICOLON, self.ch.unwrap()),
                    ',' => new_token(TokenType::COMMA, self.ch.unwrap()),
                    '{' => new_token(TokenType::LBRACE, self.ch.unwrap()),
                    '}' => new_token(TokenType::RBRACE, self.ch.unwrap()),
                    '(' => new_token(TokenType::LPAREN, self.ch.unwrap()),
                    ')' => new_token(TokenType::RPAREN, self.ch.unwrap()),
                    
                    
                    _ => {
                        if self.is_letter() {
                            let literal: String = self.read_identifier(); 
                            Token {token_type: keywords(&literal) , literal }
                        } else if self.is_digit() {
                            let literal: String = self.read_number(); 
                            Token { token_type: TokenType::INT, literal }
                        } else {
                            new_token(TokenType::ILLEGAL, self.ch.unwrap())
                        }
                    }
                }
            },
             
            None => Token { token_type: TokenType::EOF, literal: String::from("")},
        }; 
        self.read_char();
        tok
    }
    
    // Helper functions

    /// Reads the next character for the Lexer
    fn read_char(&mut self) {
        // If reach the end reprsent that with the end of file character
        if self.read_position >= self.input.len()  {
            self.ch = None;
        } else {
            self.ch = Some(self.input[self.read_position]);
        }
        self.position = self.read_position;
        self.read_position += 1;
    }

    /// Peaks the next char within the input
    fn peek_char(&mut self) -> u8 {
        if self.read_position >= self.input.len() {
            0
        } else {
            self.input[self.read_position]
        }
    }

    /// Skips through all the white space in the string
    fn skip_whitespace(&mut self) {
        while self.ch.unwrap() == ' ' as u8 || self.ch.unwrap() == '\t' as u8 
        || self.ch.unwrap() == '\n' as u8 || self.ch.unwrap() == '\r' as u8 {
            self.read_char();
        }
    }

    /// Gets the variable name
    fn read_identifier(&mut self) -> String {
        let position = self.position;

        // Continuously reads characters until it encounters a non letter
        while self.is_letter() {
            self.read_char();
        }
        // Gets the vector slice and converts it to a string
        from_utf8(&self.input[position..self.position]).unwrap().to_string()
    }

    /// Gets the number
    fn read_number(&mut self) -> String {
        let position = self.position;

        // Continuously reads characters until it encounters a non letter
        while self.is_digit() {
            self.read_char();
        }

        from_utf8(&self.input[position..self.position]).unwrap().to_string()
    }

    /// Determines if the function is a letter 
    fn is_letter(&self) -> bool {
        'a' as u8 <= self.ch.unwrap() && self.ch.unwrap() >= 'z' as u8 || 
        'A' as u8 <= self.ch.unwrap() && self.ch.unwrap() >= 'Z' as u8 || 
        self.ch.unwrap() == '_' as u8
    }

    /// Determines if the function is a digit 
    fn is_digit(&self) -> bool {
       '0' as u8 <= self.ch.unwrap() && self.ch.unwrap() >= '9' as u8 
    }

}

// Helper functions that do not need acess to self
fn new_token(token_type: TokenType, byte: u8) -> Token {
    Token { token_type, literal: from_utf8(&[byte]).unwrap().to_string() }
}
