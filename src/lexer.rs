use crate::token::TokenType;

#[cfg(test)]
mod tests {

    use crate::token;

    use super::*;

    #[derive(Debug)]
    struct TestCase {
        expected_type : TokenType,
        expected_literal : &'static str,
        
    }

    struct Lexer {
        input : &'static str,
        position : Option<i32>,
        read_position : Option<i32>,
        ch : Option<u8>,

    }

    impl Lexer {
        fn read_char(&mut self){
            if self.read_position.unwrap() >= self.input.len().try_into().unwrap() {
                self.ch = Some(0)
            } 
            else {
                self.ch = self.input.bytes().nth(self.read_position.unwrap() as usize)
            }
            self.position = self.read_position;
            self.read_position = Some(self.read_position.unwrap() + 1);
        }

        fn next_token(&mut self) -> token::Token {
            let tok : token::Token; 

            tok = match self.ch.unwrap() {
                b'=' => token::Token { _type: token::ASSIGN, literal: (self.ch.unwrap() as char) },
                b'+' => token::Token { _type: token::PLUS, literal: &(self.ch.unwrap() as char).to_string() },
                b'(' => token::Token { _type: token::LPAREN, literal: &(self.ch.unwrap() as char).to_string() },
                b')' => token::Token { _type: token::RPAREN, literal: &(self.ch.unwrap() as char).to_string()},
                b'{' => token::Token { _type: token::LBRACE, literal: &(self.ch.unwrap() as char).to_string()},
                b'}' => token::Token { _type: token::RBRACE, literal: &(self.ch.unwrap() as char).to_string() },
                b',' => token::Token { _type: token::COMMA, literal: &(self.ch.unwrap() as char).to_string() },
                b';' => token::Token { _type: token::SEMICOLON, literal: &(self.ch.unwrap() as char).to_string() },
                0 => token::Token { _type: token::EOF, literal: &(self.ch.unwrap() as char).to_string() },
                _ => token::Token { _type: token::ILLEGAL, literal: &(self.ch.unwrap() as char).to_string() }
            };
            self.read_char();
            tok
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

        let mut l = new(input);

        for (i,tt) in tests.iter().enumerate() {
            let tok = l.next_token();

            assert_eq!(tok._type, tt.expected_type, "test{}- token type wrong. expected {}, got{}",i,tt.expected_type,tok.type);
            assert_eq!(tok.literal, tt.expected_literal, "test{}- literal wrong. expected {}, got{}",i,tt.expected_literal,tok.literal);


        }

    }

    fn new(input : &str) -> Lexer {
        Lexer {
            input : Some(input),
            position : None,
            read_position : None,
            ch : None
        }
    }
    
}