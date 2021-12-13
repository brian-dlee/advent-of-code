use std::collections::binary_heap::IntoIter;
use std::collections::{HashMap, HashSet};
use std::str::FromStr;
use std::hash::Hash;
use std::cmp::Eq;
use std::fmt::{Display, Formatter};
use super::either::Either;

pub const ALL_CHARACTERS: &'static str = "abcdefg";
pub const ALL_DISPLAY_SEGMENTS: &'static [DisplaySegment] = &[
    DisplaySegment::UL,
    DisplaySegment::U,
    DisplaySegment::UR,
    DisplaySegment::C,
    DisplaySegment::LL,
    DisplaySegment::L,
    DisplaySegment::LR,
];
pub const ALL_DIGITS: &'static [Digit] = &[
    Digit::Zero,
    Digit::One,
    Digit::Two,
    Digit::Three,
    Digit::Four,
    Digit::Five,
    Digit::Six,
    Digit::Seven,
    Digit::Eight,
    Digit::Nine,
];

pub type Solved = char;
pub type Unsolved = CharSet;

impl Display for Either<Solved, Unsolved> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Either::Left(v) =>  write!(f, "(Solved  ) {}", v),
            Either::Right(v) => write!(f, "(Unsolved) {}", v),
        }
    }
}

#[derive(Debug)]
pub enum ParseDigitPatternError {
    MissingDelimiter(String),
    InvalidCharacter(String),
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub enum DisplaySegment {
    UL,
    U,
    UR,
    C,
    LL,
    L,
    LR,
}

impl Display for DisplaySegment {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            DisplaySegment::UL => "Upper Left  ",
            DisplaySegment::U  => "Top         ",
            DisplaySegment::UR => "Upper Right ",
            DisplaySegment::C =>  "Center      ",
            DisplaySegment::LL => "Lower Left  ",
            DisplaySegment::L  => "Bottom      ",
            DisplaySegment::LR => "Lower Right ",
        };
        write!(f, "{}", s)
    }
}

pub struct SegmentTranslation {
    pub solved: HashMap<DisplaySegment, Solved>,
    pub unsolved: HashMap<DisplaySegment, Unsolved>,
}

impl SegmentTranslation {
    pub fn new() -> SegmentTranslation {
        let mut result = SegmentTranslation { solved: HashMap::new(), unsolved: HashMap::new() };

        for p in ALL_DISPLAY_SEGMENTS {
            result.unsolved.insert(p.clone(), CharSet::new(ALL_CHARACTERS.to_string()));
        }

        return result;
    }

    pub fn get(&self, p: &DisplaySegment) -> Either<Solved, Unsolved> {
        if self.solved.contains_key(p) {
            return Either::Left(self.solved.get(p).unwrap().clone());
        }

        return Either::Right(self.unsolved.get(p).unwrap().clone());
    }

    pub fn get_unsolved(&self, p: &DisplaySegment) -> Option<Unsolved> {
        return self.unsolved.get(p).map(|x| x.clone())
    }

    pub fn set_solved(&mut self, p: &DisplaySegment, c: char) {
        self.solved.insert(p.clone(), c);

        self.unsolved.remove(p);

        let newly_solved: Vec<(DisplaySegment, char)> = self.unsolved.iter_mut().fold(Vec::new(), |mut next, x| {
            if x.1.remove(c) && x.1.len() == 1 {
                next.push((x.0.clone(), x.1.to_vec().first().map(|c| *c).unwrap()));
            }
            return next;
        });

        newly_solved.iter().for_each(|(position, c)| {
            self.set_solved(position, *c);
        });
    }
}

impl Display for SegmentTranslation {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let result = writeln!(f, "== Solution ==");
        for p in ALL_DISPLAY_SEGMENTS.iter() {
            result.map(|_| writeln!(f, " {} {}", p, &self.get(p)));
        }
        result.map(|_| writeln!(f, "=============="));

        return Ok(());
    }
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

    pub fn difference(&self, b: CharSet) -> CharSet {
        self.perform_set_operation(b, |a, b| a.difference(&b).map(|c| *c).collect())
    }

    pub fn union(&self, b: CharSet) -> CharSet {
        self.perform_set_operation(b, |a, b| a.union(&b).map(|c| *c).collect())
    }

    pub fn intersection(&self, b: CharSet) -> CharSet {
        self.perform_set_operation(b, |a, b| a.intersection(&b).map(|c| *c).collect())
    }

    fn perform_set_operation(&self, b: CharSet, op: fn(HashSet<char>, HashSet<char>) -> Vec<char>) -> CharSet {
        CharSet::new(
            op(self.to_set(), b.to_set()).into_iter().map(|c| c.to_string()).collect::<Vec<String>>().join("")
        )
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
pub enum DigitPotential {
    Resolved(Digit, CharSet),
    Possibilities(Vec<Digit>, CharSet),
}

impl FromStr for DigitPotential {
    type Err = ParseDigitPatternError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let char_set = CharSet::new(s.to_string());

        match char_set.len() {
            2 => Ok(DigitPotential::Resolved(Digit::One, char_set.clone())),
            3 => Ok(DigitPotential::Resolved(Digit::Seven, char_set.clone())),
            4 => Ok(DigitPotential::Resolved(Digit::Four, char_set.clone())),
            5 => Ok(DigitPotential::Possibilities([Digit::Two, Digit::Three, Digit::Five].to_vec(), char_set.clone())),
            6 => Ok(DigitPotential::Possibilities([Digit::Zero, Digit::Six, Digit::Nine].to_vec(), char_set.clone())),
            7 => Ok(DigitPotential::Resolved(Digit::Eight, char_set.clone())),
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
