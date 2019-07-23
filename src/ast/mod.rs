use token;

pub struct Identifier(pub token::Token);

pub enum Statement {
    Let(Identifier, Expression)
}

pub enum Expression  {
    Ident(Identifier),
}


pub struct Program {
    pub statements: Vec<Statement>,
}

