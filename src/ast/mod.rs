use token;
use std::fmt;

#[derive(Debug)]
pub struct Identifier(pub token::Token);

#[derive(Debug)]
pub enum Statement {
    Let(Identifier, Expression)
}

#[derive(Debug)]
pub enum Expression  {
    Ident(Identifier),
}


pub struct Program {
    pub statements: Vec<Statement>,
}

impl fmt::Display for Identifier {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Debug::fmt(self, f)
    }
}

impl fmt::Display for Expression {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Debug::fmt(self, f)
    }
}

impl fmt::Display for Statement {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Debug::fmt(self, f)
    }
}

impl Program {
    fn token_literal(&mut self) -> String {
        if self.statements.len() > 0 {
            return self.statements[0].to_string();
        } else {
            return String::from("");
        }
    }
} 
