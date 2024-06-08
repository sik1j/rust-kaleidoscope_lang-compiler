use crate::lexer::{Lexer, Token};

// possible expression nodes
#[derive(Debug)]
pub enum Expr {
    Number(f64),
    Variable(String),
    Binary(char, Box<Expr>, Box<Expr>), // op, lhs, rhs
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
            Token::Eof => -1, //
            t => panic!("No precedence found for token: {:?}", t)
        }
    }
}