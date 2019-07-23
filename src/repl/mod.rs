use token::Token;
use lexer::Lexer;
use std::io;
use std::io::prelude::*;

const PROMPT: &'static str = ">> ";


pub fn start() {
    print!("{}", PROMPT);
    io::stdout().flush().expect("REPLError: failed to flush stdin");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("REPLError: failed to readline");
    let output: Result<String, _> = input.parse();
    let repl_code = match output {
        Ok(x) => x,
        Err(error) => panic!("REPLError: {}", error),
    };

    let mut lex = Lexer::new(repl_code.as_str());
    let mut tok = lex.next_token();
    while tok != Token::EOF {
        println!("{:#?}", tok);
        tok = lex.next_token();
    }

}
#[cfg(test)]
mod tests {
    use repl;

    #[test]
    fn test_sanity() {
        repl::start();
        assert_eq!(1 + 1, 2);
    }

}