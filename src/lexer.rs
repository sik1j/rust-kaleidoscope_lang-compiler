use std::io::Read;
use std::str::FromStr;
use crate::lexer::Token::{TokChar, TokDef, TokEof, TokExtern, TokIdentifier, TokNumber};

#[derive(Debug)]
pub enum Token {
    TokEof,

    // commands
    TokDef,
    TokExtern,

    // primary
    TokIdentifier(String),
    TokNumber(f64),

    // unknown type
    TokChar(char)
}

fn get_char() -> char {
    let mut buffer = [0;1];
    let stdin = std::io::stdin();
    let mut handle = stdin.lock();
    match handle.read(&mut buffer) {
        Ok(_) => {if buffer[0] == 0 {0 as char} else {buffer[0] as char}}
        Err(_) => {0 as char}
    }
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
                "def" => TokDef,
                "extern" => TokExtern,
                _ => TokIdentifier(identifier),
            }
        }),
        '0'..='9' | '.' => {
            let mut num_str = String::from(last_char);
            while {last_char = get_char(); last_char.is_alphanumeric() || last_char == '.'} {
                num_str.push(last_char);
            }
            match f64::from_str(num_str.as_str()) {
                Ok(n) => Some(TokNumber(n)),
                Err(_) => None
            }
        },
        '#' => {
            while {last_char = get_char(); last_char != '\0' && last_char != '\n' && last_char != '\r'} {};
            if last_char != '\0' {
                get_tok()
            } else if last_char == '\0' {
                Some(TokEof)
            } else { // unrecognized token, e.g. + or something
                let ch = last_char;
                last_char = get_char();
                Some(TokChar(ch))
            }
        }
        _ => { None }
    }
}
