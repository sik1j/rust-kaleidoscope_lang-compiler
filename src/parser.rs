use std::fmt::Debug;
use std::io::Write;
use crate::lexer::{Lexer, Token};

// possible expression nodes
#[derive(Debug)]
pub enum Expr {
    Number(f64),
    Variable(String),
    Binary(char, Box<Expr>, Box<Expr>), // op, lhs, rhs
    Call{callee: String, args: Vec<Expr>},
    Prototype {func_name: String, arg_names: Vec<String>},
    Function {prototype: Box<Expr>, body: Box<Expr>},
}

pub struct Parser {
    pub lexer: Lexer
}

impl Parser {
    pub fn new() -> Parser {
        let mut l = Lexer::new();
        l.get_next_tok();
        Parser {
            lexer: l,
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

    pub fn parse_expr(&mut self) -> Expr {
        let lhs = self.parse_primary();

        self.parse_binary_op_rhs(0, lhs)
    }
    pub fn parse_binary_op_rhs(&mut self, min_precedence: i64, mut lhs: Expr) -> Expr {
        loop {
            let cur_prec = self.get_tok_precedence();
            if cur_prec < min_precedence {
                return lhs;
            }
            let cur_op = match self.lexer.cur_tok {
                Token::Char(c) => c,
                _ => panic!("Unknown operation: {:?}", self.lexer.cur_tok)
            };

            self.lexer.get_next_tok(); // consume op
            let mut rhs =self.parse_primary();

            let next_prec = self.get_tok_precedence();

            if next_prec > cur_prec {
                rhs = self.parse_binary_op_rhs(cur_prec+1, rhs);
            }
            lhs = Expr::Binary(cur_op, Box::from(lhs), Box::from(rhs));
        }
    }
    fn get_tok_precedence(&self) -> i64 {
        match &self.lexer.cur_tok {
            Token::Char('<') => 10,
            Token::Char('+') => 20,
            Token::Char('-') => 30,
            Token::Char('*') => 40,
            _ => -1,
        }
    }

    fn parse_prototype(&mut self) -> Expr {
        let func_name = match &self.lexer.cur_tok {
            Token::Identifier(name) => name.clone(),
            _ => panic!("Expected ")
        };
        self.lexer.get_next_tok(); // consume name

        if !matches!(self.lexer.cur_tok, Token::Char('(')) {
            panic!("Expected a '(' in prototype");
        }

        let mut arg_names = vec![];
        loop {
            self.lexer.get_next_tok();
            arg_names.push(match &self.lexer.cur_tok {
                Token::Identifier(name) => name.clone(),
                _ => break
            })
        }

        if !matches!(self.lexer.cur_tok, Token::Char(')')) {
            panic!("Expected '(' in prototpye");
        }
        self.lexer.get_next_tok(); // consume '('

        Expr::Prototype {func_name, arg_names: arg_names}
    }

    fn parse_definition(&mut self) -> Expr {
        self.lexer.get_next_tok(); // consume 'def'
        let prototype = self.parse_prototype();

        let body = self.parse_expr();
        Expr::Function {prototype: Box::new(prototype), body: Box::new(body)}
    }

    fn parse_extern(&mut self) -> Expr {
        self.lexer.get_next_tok(); // eat extern.
        self.parse_prototype()
    }

    fn parse_top_level(&mut self) -> Expr {
        let expr = self.parse_expr();

        let proto = Expr::Prototype {func_name: String::from(""), arg_names: vec![]};
        Expr::Function {body: Box::from(expr), prototype: Box::from(proto)}
    }

    pub fn main_loop(&mut self) {
        loop {
            match self.lexer.cur_tok {
                Token::Eof => return,
                Token::Char(';') => {self.lexer.get_next_tok(); continue}
                Token::Def => self.handle_definition(),
                Token::Extern => self.handle_extern(),
                _ => self.handle_top_level_expression(),
            }
            print!("ready> "); std::io::stdout().flush().unwrap();
        }
    }


    fn handle_definition(&mut self) {
        self.parse_definition();
        println!("Parsed a function definition.");
    }
    fn handle_extern(&mut self) {
        self.parse_extern();
        println!("Parsed an extern");
    }
    fn handle_top_level_expression(&mut self) {
        self.parse_top_level();
        println!("Parsed a top-level expr"); std::io::stdout().flush().unwrap();
    }
}