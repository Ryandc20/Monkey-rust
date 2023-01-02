// Contains the test for the lexer to make sure it is working correctly 

#[cfg(test)]
mod tests {
    use crate::lexer::Lexer;
    use crate::token::*;

    #[test]
    fn test_next_token() {
        // Defines the text input
        let input = String::from("let five = 5;

let add = fn(x, y) {
  x + y;
};"
        ); 

        let mut lexer = Lexer::new(input);

        let tests = [
            Token { token_type: TokenType::LET, literal: String::from("let")},
            Token { token_type: TokenType::IDENT, literal: String::from("five")},
            Token { token_type: TokenType::ASSIGN, literal: String::from("=")},
            Token { token_type: TokenType::INT, literal: String::from("5")},
            Token { token_type: TokenType::SEMICOLON, literal: String::from(";")},
            Token { token_type: TokenType::LET, literal: String::from("let")},
            Token { token_type: TokenType::IDENT, literal: String::from("add")},
            Token { token_type: TokenType::ASSIGN, literal: String::from("=")},
            Token { token_type: TokenType::FUNCTION, literal: String::from("fn")},
            Token { token_type: TokenType::LPAREN, literal: String::from("(")},
            Token { token_type: TokenType::IDENT, literal: String::from("x")},
            Token { token_type: TokenType::COMMA, literal: String::from(",")},
            Token { token_type: TokenType::IDENT, literal: String::from("y")},
            Token { token_type: TokenType::RPAREN, literal: String::from(")")},
            Token { token_type: TokenType::LBRACE, literal: String::from("{")},
            Token { token_type: TokenType::IDENT, literal: String::from("x")},
            Token { token_type: TokenType::PLUS, literal: String::from("+")},
            Token { token_type: TokenType::IDENT, literal: String::from("y")},
            Token { token_type: TokenType::SEMICOLON, literal: String::from(";")},
            Token { token_type: TokenType::RBRACE, literal: String::from("}")},
            Token { token_type: TokenType::SEMICOLON, literal: String::from(";")},
            Token { token_type: TokenType::EOF, literal: String::from("")},
        ];
        
        for i in 0..tests.len() {
            let token = lexer.next_token(); 
            // Make sure we get the correct token
            assert_eq!(token.token_type, tests[i].token_type);
            // Make sure we get the right literal
            assert_eq!(token.literal, tests[i].literal);
        }

    }
    #[test]
    fn simple() {
        let input = String::from("let       =");
        let mut lexer = Lexer::new(input);

        let token = lexer.next_token();
        assert_eq!(token.token_type, TokenType::LET);
        assert_eq!(token.literal, String::from("let"));
        let token = lexer.next_token();
        assert_eq!(token.token_type, TokenType::ASSIGN);
        assert_eq!(token.literal, String::from("="));
    }
}
