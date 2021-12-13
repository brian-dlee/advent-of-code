use std::ops::AddAssign;
use std::str::FromStr;
use y2021::utils;

#[derive(Debug)]
pub enum Digit {
    One(String),
    Two(String),
    Three(String),
    Four(String),
    Five(String),
    Six(String),
    Seven(String),
    Eight(String),
    Nine(String),
}

#[derive(Debug)]
pub enum ParseDigitPatternError {
    MissingDelimiter(String),
}

#[derive(Debug)]
pub struct DigitPattern {
    pub codes: Vec<String>,
    pub output: Vec<String>,
}

impl FromStr for DigitPattern {
    type Err = ParseDigitPatternError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut pipe = false;
        let mut pattern = DigitPattern { codes: Vec::new(), output: Vec::new() };

        for part in s.split_whitespace() {
            if part == "|" {
                pipe = true;
                continue;
            }

            if pipe {
                pattern.output.push(part.to_string());
            } else {
                pattern.codes.push(part.to_string());
            }
        }

        if pipe == false {
            Err(ParseDigitPatternError::MissingDelimiter(s.to_string()))
        } else {
            Ok(pattern)
        }
    }
}

#[derive(Debug)]
pub struct DigitPatterns {
    pub patterns: Vec<DigitPattern>
}

impl FromStr for DigitPatterns {
    type Err = ParseDigitPatternError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut patterns = DigitPatterns { patterns: Vec::new() };

        for line in s.lines() {
            match DigitPattern::from_str(line) {
                Ok(p) => patterns.patterns.push(p),
                Err(e)=> {
                    return Err(e);
                }
            }
        }

        Ok(patterns)
    }
}

fn main() {
    println!("Starting Day 8a");
    println!("Descrambling display");

    let input = DigitPatterns::from_str(
        utils::read_input("./input/input.txt").trim()
    ).unwrap();

    let digits = input.patterns.into_iter().flat_map(|p| p.output).map(|code| {
       match code.len() {
           2 => Some(Digit::One(code)),
           3 => Some(Digit::Seven(code)),
           4 => Some(Digit::Four(code)),
           7 => Some(Digit::Eight(code)),
           _ => None,
       }
    });

    println!("Solution: {:?}", digits.filter(|x| x.is_some()).count());
}
