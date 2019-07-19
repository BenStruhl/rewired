use token::Token; 
use token::lookup_ident;

#[allow(dead_code)]
struct Lexer<'a> {
    input: &'a str,
    position: usize,
    read_position: usize,
    ch: u8,
}
#[allow(dead_code)]
impl<'a> Lexer<'a> {
    fn read_char(&mut self) {
        if self.read_position >= self.input.len() {
            self.ch = 0;
        } else {
            self.ch = self.input.as_bytes()[self.read_position];
        }
        self.position = self.read_position;
        self.read_position += 1;
    } 
    fn new(input: &'a str) -> Lexer {
        let mut l = Lexer {
            input,
            position: 0,
            read_position: 0,
            ch: 0,
        };
        l.read_char();
        return l;
    }

    fn read_number(&mut self) -> Token {
        let position = self.position;
        while (self.ch as char).is_ascii_digit() {
            self.read_char();
        }
        self.read_position -= 1;
        return Token::INT(self.input[position..self.position].parse::<i64>().unwrap());
    }

    fn read_identifier(&mut self) -> Token {
        let position = self.position;
        while (self.ch as char).is_ascii_alphabetic() {
            self.read_char();
        }
        self.read_position -= 1;
        return lookup_ident(&self.input[position..self.position]);
    }

    fn skip_whitespace(&mut self) {
        while (self.ch as char).is_ascii_whitespace() {
            self.read_char()
        }
    }

    fn peek_char(&mut self) -> u8 {
        if self.read_position >= self.input.len() {
            return 0;
        } else {
            return l.input[l.read_position];
        }
    }

    fn next_token(&mut self) -> Token {
        self.skip_whitespace();
        let tok: Token= match self.ch as char {
            '0' ... '9' => self.read_number(),
            'a'...'z' | 'A'...'Z' | '_'  => self.read_identifier(),
            '='   => { if (self.peek_char as char) == '=' {
                             self.read_char();
                             Token::EQ
                         } else {
                             Token::ASSIGN
                         }
                     },
            '<'   => Token::LT,
            '>'   => Token::GT,
            '-'   => Token::MINUS,
            '!'   =>  { if (self.peek_char as char) == '=' {
                             self.read_char();
                             Token::NOT_EQ
                         } else {
                             Token::BANG
                         }
                     },,
            '*'   => Token::ASTERISK,
            '/'   => Token::SLASH,
            ';'   => Token::SEMICOLON,
            '('   => Token::LPAREN,
            ')'   => Token::RPAREN,
            ','   => Token::COMMA,
            '+'   => Token::PLUS,
            '{'   => Token::LBRACE,
            '}'   => Token::RBRACE,
            '\0'  => Token::EOF,
            _     => Token::ILLEGAL, 

        };

        self.read_char();

        if tok == Token::ILLEGAL {
          panic!("cannot match given token to langauge syntax: {}", self.ch)
        };
        return tok;
    }
}
#[cfg(test)]
mod tests {
    use token::Token; 
    use lexer::Lexer;
    #[test]
    fn test_next_token() {
        let input = r#"let five = 5;
let ten = 10;
let add = fn(x, y) {
  x + y;
};
let result = add(five, ten);
!-/*5;
5 < 10 > 5;


if (5 < 10) {
    return true;
} else {
    return false;
}
10 == 10;
10 != 9;
"#;
        let test_array = vec![
            Token::LET,
            Token::IDENT(String::from("five")),
            Token::ASSIGN,
            Token::INT(5),
            Token::SEMICOLON,
            Token::LET,
            Token::IDENT(String::from("ten")),
            Token::ASSIGN,
            Token::INT(10),
            Token::SEMICOLON,
            Token::LET,
            Token::IDENT(String::from("add")),
            Token::ASSIGN,
            Token::FUNCTION,
            Token::LPAREN,
            Token::IDENT(String::from("x")),
            Token::COMMA,
            Token::IDENT(String::from("y")),
            Token::RPAREN,
            Token::LBRACE,
            Token::IDENT(String::from("x")),
            Token::PLUS,
            Token::IDENT(String::from("y")),
            Token::SEMICOLON,
            Token::RBRACE,
            Token::SEMICOLON,
            Token::LET,
            Token::IDENT(String::from("result")),
            Token::ASSIGN,
            Token::IDENT(String::from("add")),
            Token::LPAREN,
            Token::IDENT(String::from("five")),
            Token::COMMA,
            Token::IDENT(String::from("ten")),
            Token::RPAREN,
            Token::SEMICOLON,
            Token::BANG,
            Token::MINUS,
            Token::SLASH,
            Token::ASTERISK,
            Token::INT(5),
            Token::SEMICOLON,
            Token::INT(5),
            Token::LT,
            Token::INT(10),
            Token::GT,
            Token::INT(5),
            Token::SEMICOLON,
            Token::IF,
            Token::LPAREN,
            Token::INT(5),
            Token::LT,
            Token::INT(10),
            Token::RPAREN,
            Token::LBRACE,
            Token::RETURN,
            Token::BOOL(true),
            Token::SEMICOLON,
            Token::RBRACE,
            Token::ELSE,
            Token::LBRACE,
            Token::RETURN,
            Token::BOOL(false),
            Token::SEMICOLON,
            Token::RBRACE,
            Token::INT(10),
            Token::EQ,
            Token::INT(10),
            Token::SEMICOLON,
            Token::INT(10),
            Token::NOT_EQ,
            Token::INT(9),
            Token::SEMICOLON,
            Token::INT(10),
            Token::LT_EQ,
            Token::INT(10),
            Token::SEMICOLON,
            Token::INT(10),
            Token::GT_EQ,
            Token::INT(10),
            Token::SEMICOLON
            Token::EOF,
        ];
        let mut lexerr = Lexer::new(&input);
        for tt in test_array.iter() {
            let tok = lexerr.next_token();
            assert_eq!(tok, *tt);
        }
    }
}