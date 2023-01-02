use std::io::{self, Write};
use crate::lexer::Lexer;
use crate::token::TokenType;

const PROMPT: &'static str = ">> ";

pub fn start() {
    loop {
        let mut input = String::new(); 
        print!("{}", PROMPT);
        io::stdout().flush().expect("Can not flush std out");
        
        if io::stdin().read_line(&mut input).unwrap() == 0 {
            break;
        }

        let mut lexer = Lexer::new(&input);

        let mut token = lexer.next_token();

        while token.token_type != TokenType::EOF {
            println!("{:?} {}", token.token_type, token.literal);
            token = lexer.next_token();
        }
    }
}

