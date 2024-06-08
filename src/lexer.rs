use std::io::Read;
use std::str::FromStr;
use crate::lexer::Token::Char;

#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    Eof,

    // commands
    Def,
    Extern,

    // primary
    Identifier(String),
    Number(f64),

    // unknown type
    Char(char)
}

pub struct Lexer {
    pub cur_tok: Token,
    pub last_char: char,
}

impl Lexer {
    pub fn new() -> Self {
        Lexer {
            cur_tok: Token::Eof,
            last_char: Self::get_char(),
        }
    }
    fn get_char() -> char {
        let mut buffer = [0;1];
        let stdin = std::io::stdin();
        let mut handle = stdin.lock();

        handle.read_exact(&mut buffer).map_or(0 as char, |_| {buffer[0] as char})
    }

    /**
    gets the next token from stdin
     */
    fn get_tok(&mut self) -> Option<Token> {
        while self.last_char.is_whitespace() {self.last_char = Lexer::get_char(); }

        match self.last_char {
            'a'..='z' | 'A'..='Z' => Some({
                let mut identifier = String::from(self.last_char);
                while { self.last_char = Lexer::get_char(); self.last_char.is_alphanumeric() } {
                    identifier.push(self.last_char);
                };
                match identifier.as_str() {
                    "def" => Token::Def,
                    "extern" => Token::Extern,
                    _ => Token::Identifier(identifier.clone()),
                }
            }),
            '0'..='9' | '.' => {
                let mut num_str = String::from(self.last_char);
                while {self.last_char = Lexer::get_char(); self.last_char.is_alphanumeric() || self.last_char == '.'} {
                    num_str.push(self.last_char);
                }
                f64::from_str(num_str.as_str()).ok().map(|n| {Token::Number(n)})
            },
            '#' => {
                while {self.last_char = Lexer::get_char(); self.last_char != '\0' && self.last_char != '\n' && self.last_char != '\r'} {};
                self.get_tok()
            }
            '\0' => {Some(Token::Eof)},
            _ => {
                let res = self.last_char;
                self.last_char = Lexer::get_char(); // move to next char
                Some(Char(res))
            },
        }
    }

    pub fn get_next_tok(&mut self) {
        match self.get_tok() {
            None => {panic!("Could not get token")}
            Some(tok) => {self.cur_tok = tok;}
        }
    }
}