use std::collections::binary_heap::IntoIter;
use std::collections::HashSet;
use std::str::FromStr;
use std::hash::Hash;
use std::cmp::Eq;
use std::fmt::{Display, Formatter};

#[derive(Debug)]
pub enum ParseDigitPatternError {
    MissingDelimiter(String),
    InvalidCharacter(String),
}

#[derive(Clone, Debug, Eq, Hash)]
pub struct CharSet {
    sequence: Vec<char>,
}

impl CharSet {
    pub fn new(s: String) -> CharSet {
        let chars: HashSet<char> = HashSet::from_iter(s.chars());
        let mut sequence: Vec<char> = Vec::from_iter(chars.into_iter());
        sequence.sort();
        return CharSet { sequence: sequence };
    }

    pub fn empty() -> CharSet {
        return CharSet { sequence: Vec::new() };
    }

    pub fn merged(&self, other: CharSet) -> CharSet {
        let mut new = self.to_vec();
        new.append(&mut other.to_vec());
        return CharSet::new(new.iter().map(|c| c.to_string()).collect::<Vec<String>>().join(""));
    }

    pub fn only(&self) -> char {
        return self.sequence.first().unwrap().clone();
    }

    pub fn remove(&mut self, c: char) -> bool {
        if !self.sequence.contains(&c) {
            return false;
        }

        let mut new: HashSet<char> = HashSet::from_iter(self.sequence.clone().into_iter());
        new.remove(&c);

        self.sequence = new.iter().map(|c| *c).collect::<Vec<char>>();
        self.sequence.sort();

        return true;
    }

    pub fn len(&self) -> usize {
        return self.sequence.len();
    }

    pub fn contains(&self, c: &char) -> bool {
        let chars: HashSet<char> = self.to_set();
        return chars.contains(c);
    }

    pub fn to_vec(&self) -> Vec<char> {
        return self.sequence.clone();
    }

    pub fn to_set(&self) -> HashSet<char> {
        return HashSet::from_iter(self.sequence.clone().into_iter());
    }
}

impl Display for CharSet {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        return write!(f, "{}", self.sequence.iter().map(|s| s.to_string()).collect::<Vec<String>>().join(""));
    }
}

impl PartialEq for CharSet {
    fn eq(&self, other: &Self) -> bool {
        let a: HashSet<char> = HashSet::from_iter(self.sequence.clone());
        let b: HashSet<char> = HashSet::from_iter(other.sequence.clone());
        let c: HashSet<char> = HashSet::from_iter(a.union(&b).map(|x| *x));

        return &a.len() == &c.len() && &b.len() == &c.len();
    }

    fn ne(&self, other: &Self) -> bool {
        return !self.eq(other);
    }
}

#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub enum Digit {
    Zero,
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
}

#[derive(Clone, Debug)]
pub enum DigitResult {
    Resolved(Digit, CharSet),
    Possibilities(Vec<Digit>, CharSet),
}

impl DigitResult {

}

impl FromStr for DigitResult {
    type Err = ParseDigitPatternError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let char_set = CharSet::new(s.to_string());

        match char_set.len() {
            2 => Ok(DigitResult::Resolved(Digit::One, char_set.clone())),
            3 => Ok(DigitResult::Resolved(Digit::Seven, char_set.clone())),
            4 => Ok(DigitResult::Resolved(Digit::Four, char_set.clone())),
            5 => Ok(DigitResult::Possibilities([Digit::Two, Digit::Three, Digit::Five].to_vec(), char_set.clone())),
            6 => Ok(DigitResult::Possibilities([Digit::Zero, Digit::Six, Digit::Nine].to_vec(), char_set.clone())),
            7 => Ok(DigitResult::Resolved(Digit::Eight, char_set.clone())),
            _ => Err(ParseDigitPatternError::InvalidCharacter(s.to_string())),
        }
    }
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
