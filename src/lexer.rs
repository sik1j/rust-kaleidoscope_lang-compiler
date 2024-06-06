use std::io::Read;
use std::str::FromStr;
use crate::lexer::Token::Char;

#[derive(Debug)]
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

fn get_char() -> char {
    let mut buffer = [0;1];
    let stdin = std::io::stdin();
    let mut handle = stdin.lock();

    handle.read_exact(&mut buffer).map_or(0 as char, |_| {buffer[0] as char})
}

/**
    gets the next token from stdin
 */
pub fn get_tok() -> Option<Token> {
    let mut last_char = ' ';
    while last_char.is_whitespace() {last_char = get_char(); }

    match last_char {
        'a'..='z' | 'A'..='Z' => Some({
            let mut identifier = String::from(last_char);
            while { last_char = get_char(); last_char.is_alphanumeric() } {
                identifier.push(last_char);
            };
            match identifier.as_str() {
                "def" => Token::Def,
                "extern" => Token::Extern,
                _ => Token::Identifier(identifier),
            }
        }),
        '0'..='9' | '.' => {
            let mut num_str = String::from(last_char);
            while {last_char = get_char(); last_char.is_alphanumeric() || last_char == '.'} {
                num_str.push(last_char);
            }
            f64::from_str(num_str.as_str()).ok().map(|n| {Token::Number(n)})
        },
        '#' => {
            while {last_char = get_char(); last_char != '\0' && last_char != '\n' && last_char != '\r'} {};
            get_tok()
        }
        '\0' => {Some(Token::Eof)},
        _ => {
            Some(Char(last_char))
        },
    }
}
