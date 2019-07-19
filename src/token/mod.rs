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
    NOT_EQ,

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