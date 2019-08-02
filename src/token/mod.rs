use std::fmt;

#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    ILLEGAL,
    EOF,

    // identifers + literals
    IDENT(String),
    INT(i64),

    ASSIGN,
    PLUS,
    MINUS,
    BANG,
    ASTERISK,
    SLASH,

    LT,
    GT,

    COMMA,
    SEMICOLON,

    EQ,
    NOTEQ,

    LPAREN,
    RPAREN,
    LBRACE,
    RBRACE,

    FUNCTION,
    LET,

}

pub fn lookup_ident(ident: &str) -> Token {
    match ident {
        "fn" => return Token::FUNCTION,
        "let" => return Token::LET,
        _ => return Token::IDENT(ident.to_string()),
    }
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Debug::fmt(self, f)
    }
}

#[cfg(test)]
mod tests {
    use token::Token;
    use token::lookup_ident;
    #[test]
    fn test_lookup_ident() {
        assert_eq!(lookup_ident(&String::from("fn")), Token::FUNCTION);
        assert_eq!(lookup_ident(&String::from("let")), Token::LET);
        assert_eq!(lookup_ident(&String::from("bob")), Token::IDENT(String::from("bob")));
    }

}