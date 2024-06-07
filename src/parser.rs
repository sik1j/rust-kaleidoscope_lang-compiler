use crate::lexer::{Lexer, Token};
use std::collections::hash_map;

// possible expression nodes
#[derive(Debug)]
pub enum Expr {
    Number(f64),
    Variable(String),
    Binary{lhs: Box<Expr>, op: char, rhs: Box<Expr>},
    Call{callee: String, args: Vec<Expr>},
}

// function prototype
struct Prototype {
    name: String,
    args: Vec<String>
}

// function definition
struct Function {
    prototype: Prototype,
    body: Expr
}

pub struct Parser {
    pub lexer: Lexer
}

impl Parser {
    pub fn new() -> Parser {
        Parser {
            lexer: Lexer::new(),
        }
    }
    pub fn parse_primary(&mut self) -> Expr {
        match self.lexer.cur_tok {
            Token::Identifier(_) => {self.parse_identifier_expr()},
            Token::Number(_) => {self.parse_number()},
            Token::Char('(') => {self.parse_paren_expr()},
            _ => panic!("unknown token when expecting an expression"),
        }
    }

    // number_expr ::= number
    fn parse_number(&mut self) -> Expr {
        let res = match self.lexer.cur_tok {
            Token::Number(n) => Expr::Number(n),
            _ => panic!("Expected a number")
        };
        self.lexer.get_next_tok(); // eat number
        res
    }

    // paren_expr = '(' expression ')'
    fn parse_paren_expr(&mut self) -> Expr {
        self.lexer.get_next_tok(); // eat '('
        let expr = self.parse_expr();

        if self.lexer.cur_tok != Token::Char(')') {
            panic!("expected ')'");
        }
        self.lexer.get_next_tok(); // eat ')'
        expr
    }

    // identifier_expr
    //     ::= identifier
    //     ::= identifier '(' expression* ')'
    fn parse_identifier_expr(&mut self) -> Expr {
        let id_name = match &self.lexer.cur_tok {
            Token::Identifier(name) => name.clone(),
            _ => panic!("expected identifier")
        };
        print!("called");

        self.lexer.get_next_tok(); // eat identifier
        if self.lexer.cur_tok != Token::Char('(') { // if simple variable name
            return Expr::Variable(id_name);
        }

        self.lexer.get_next_tok(); // eat '('
        let mut args = vec![];
        if self.lexer.cur_tok != Token::Char(')') {
            loop {
                let arg = self.parse_expr();
                args.push(arg);

                if self.lexer.cur_tok == Token::Char(')') {
                    break;
                }

                if self.lexer.cur_tok != Token::Char(',') {
                    panic!("Expected ')' or ',' in argument list");
                }
                self.lexer.get_next_tok(); // eat ','
            }
        }
        self.lexer.get_next_tok(); // eat ')'

        Expr::Call {callee: id_name, args}
    }

    fn parse_expr(&self) -> Expr {
        todo!()
    }
}