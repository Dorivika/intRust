use crate::token::TokenType;

#[cfg(test)]
mod tests {
    use std::option;

    use super::*;

    #[derive(Debug)]
    struct TestCase {
        expected_type : TokenType,
        expected_literal : &'static str,
        
    }

    struct Lexer {
        input : &'static str,
        position : option<i32>,
        read_postion : option<i32>,
        ch : option<u8>,

    }

    impl Lexer {
        fn read_char(&self){
            if self.read_postion >= self.input.len() {
                self.ch = 0
            } 
            else {
                 self.ch = self.input[self.read_postion.unwrap()]
            }
            self.position.un = self.read_postion;
            self.read_postion += 1;
        }
        
    }

    #[test]
    fn test_next_token() {
        let input = "=+(){},;";
        let tests = vec![
            TestCase {expected_type : token::ASSIGN, expected_literal : "="},
            TestCase {expected_type : token::PLUS, expected_literal : "+"},
            TestCase {expected_type : token::LPAREN, expected_literal : "("},
            TestCase {expected_type : token::RPAREN, expected_literal : ")"},
            TestCase {expected_type : token::LBRACE, expected_literal : "{"},
            TestCase {expected_type : token::RBRACE, expected_literal : "}"},
            TestCase {expected_type : token::COMMA, expected_literal : ","},
            TestCase {expected_type : token::SEMICOLON, expected_literal : ";"},
            TestCase {expected_type : token::EOF, expected_literal : ""},
                                        
        ];

        let l = new(input);

        for (i,tt) in tests.iter().enumerate() {
            let tok = l.next_token();

            assert_eq!(tok.type, tt.expected_type, "test{}- token type wrong. expected {}, got{}",i,tt.expected_type,tok.type);
            assert_eq!(tok.literal, tt.expected_literal, "test{}- literal wrong. expected {}, got{}",i,tt.expected_literal,tok.literal);


        }

    }

    fn new(input : &str) -> Lexer {
        Lexer {
            input : Some(input),
            position : None,
            read_postion : None,
            ch : None
        }
    }
    
}