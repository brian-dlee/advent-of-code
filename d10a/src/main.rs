use std::fmt::{Display, Formatter};
use std::str::FromStr;
use y2021::either::Either;
use y2021::utils;

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum SyntaxError {
    Corrupt(Token),
    Incomplete,
    InvalidCharacter(char),
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Tokens(Vec<Token>);

impl Display for Tokens {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0.iter().map(|c| format!("{}", c)).collect::<Vec<String>>().join(""))
    }
}

impl FromStr for Tokens {
    type Err = SyntaxError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut tokens: Vec<Token> = Vec::new();
        for c in s.chars() {
            match c {
                '(' => tokens.push(Token::LeftParen),
                '[' => tokens.push(Token::LeftBracket),
                '{' => tokens.push(Token::LeftBrace),
                '<' => tokens.push(Token::LeftAngle),
                ')' => tokens.push(Token::RightParen),
                ']' => tokens.push(Token::RightBracket),
                '}' => tokens.push(Token::RightBrace),
                '>' => tokens.push(Token::RightAngle),
                c => {
                    return Err(SyntaxError::InvalidCharacter(c));
                },
            }
        }

        return Ok(Tokens(tokens));
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Token {
    LeftParen,
    RightParen,
    LeftBracket,
    RightBracket,
    LeftBrace,
    RightBrace,
    LeftAngle,
    RightAngle,
}

impl Token {
    pub fn is_open(&self) -> bool {
        self == &Token::LeftParen ||
            self == &Token::LeftBracket ||
            self == &Token::LeftBrace ||
            self == &Token::LeftAngle
    }

    pub fn is_close(&self) -> bool {
        self == &Token::RightParen ||
            self == &Token::RightBracket ||
            self == &Token::RightBrace ||
            self == &Token::RightAngle
    }

    pub fn get_opposite(&self) -> Token {
        match self {
            Token::RightParen => Token::LeftParen,
            Token::RightBracket => Token::LeftBracket,
            Token::RightBrace => Token::LeftBrace,
            Token::RightAngle => Token::LeftAngle,
            Token::LeftParen => Token::RightParen,
            Token::LeftBracket => Token::RightBracket,
            Token::LeftBrace => Token::RightBrace,
            Token::LeftAngle => Token::RightAngle,
        }
    }
}

impl Display for Token {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Token::RightParen => ")",
            Token::RightBracket => "]",
            Token::RightBrace => "}",
            Token::RightAngle => ">",
            Token::LeftParen => "(",
            Token::LeftBracket => "[",
            Token::LeftBrace => "{",
            Token::LeftAngle => "<",
        };
        write!(f, "{}", s)
    }
}

fn check_syntax(code: Vec<Token>) -> Result<Vec<Token>, SyntaxError> {
    match code.len() {
        0 => Ok(Vec::new()),
        1 => Ok(code),
        2 => {
            if code[0].is_open() && code[1].is_open() {
                Err(SyntaxError::Incomplete)
            } else if code[0].is_open() && code[1].is_close() {
                if code[0].get_opposite() != code[1] {
                    Err(SyntaxError::Corrupt(code[1].clone()))
                } else {
                    Ok(Vec::new())
                }
            } else {
                Ok(code)
            }
        },
        _ => match code.split_at(1) {
            (head, tail) if head[0].is_open() && tail[0].is_close() => {
                match check_syntax([head[0].clone(), tail[0].clone()].to_vec()) {
                    Ok(_) => check_syntax(tail[1..].to_vec()),
                    Err(e) => Err(e),
                }
            },
            (head, tail) if head[0].is_close() => {
                return Ok(code.clone());
            },
            (head, tail) if tail[0].is_open() => {
                match check_syntax(tail.to_vec()) {
                    Ok(x) => {
                        let mut next = head.to_vec();
                        next.extend(x);
                        check_syntax(next)
                    },
                    Err(e) => Err(e),
                }
            },
            (head, tail) => {
                if head[0].get_opposite() != tail[0] {
                    Err(SyntaxError::Corrupt(tail[0].clone()))
                } else {
                    check_syntax(tail[1..].to_vec())
                }
            }
        }
    }
}

fn main() {
    println!("Starting Day 10a");
    println!("Finding syntax errors");

    let input: Vec<Tokens> = utils::read_input("./input/input.txt").trim().lines()
        .map(|l| Tokens::from_str(l).unwrap()).collect();

    let solution = input.into_iter().fold(0, |result, line| {
        match check_syntax(line.clone().0).err().unwrap() {
            SyntaxError::Corrupt(c) => {
                match c {
                    Token::RightParen => result + 3,
                    Token::RightBracket => result + 57,
                    Token::RightBrace => result + 1197,
                    Token::RightAngle => result + 25137,
                    _ => result,
                }
            },
            _ => result,
        }
    });

    println!("Solution: {:?}", solution);
}
