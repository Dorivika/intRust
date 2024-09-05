pub type TokenType = &'static str;

pub struct Token {
    pub _type: TokenType,
    pub literal: &'static str,
}

CONST!( EOF: &'static str = "EOF";
 ILLEGAL:&'static str= "ILLEGAL";
 IDENT: &'static str = "IDENT";
 INT: &'static str = "INT";
 ASSIGN: &'static str = "=";
 PLUS: &'static str = "+";
 COMMA: &'static str = ",";
 SEMICOLON: &'static str = ";";
 LPAREN: &'static str = "(";
 RPAREN: &'static str = ")";
 LBRACE: &'static str = "{";
 RBRACE: &'static str = "}";
 FUNCTION: &'static str = "FUNCTION";
 LET: &'static str = "LET";
);


