use ast::Program;
use lexer::Lexer;
use token;

pub struct Parser<'a> {
    pub l: &'a mut Lexer<'a>,
    pub cur_token: token::Token,
    peek_token: token::Token,
}

impl<'a> Parser<'a> {
    pub fn new(l: &'a mut Lexer<'a>) -> Parser {
        let cur_token = l.next_token();
        let peek_token = l.next_token();
        return Parser {
            l,
            cur_token,
            peek_token,
        }
    }
    pub fn next_token(&mut self) {
        self.cur_token = self.l.next_token();
        self.peek_token = self.l.next_token();
    }

    // pub parse_program(&mut self) -> Program {

    // }
}


#[cfg(test)]
mod tests {
    use token::Token; 
    use lexer::Lexer;
    #[test]
    fn test_let_statement() {
        let input = r#"let five = 5;
let ten = 10;
let add = fn(x, y) {
  x + y;
};
let result = add(five, ten);
!-/*5;
5 < 10 > 5;
10 == 10; 
10 != 9;
"#;
         
    }
}