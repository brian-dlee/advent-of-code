use std::borrow::BorrowMut;
use std::collections::{HashMap, HashSet};
use std::str::FromStr;
use std::fmt::{Display, Formatter};
use y2021::display_troubleshooting::{CharSet, DigitPatterns, DigitResult, Digit};
use y2021::display_troubleshooting::Digit::Eight;
use y2021::utils;

type Solved = char;
type Unsolved = CharSet;

const ALL_CHARACTERS: &'static str = "abcdefg";
const ALL_DIGIT_POSITIONS: &'static [DigitPosition] = &[
    DigitPosition::UL,
    DigitPosition::U,
    DigitPosition::UR,
    DigitPosition::C,
    DigitPosition::LL,
    DigitPosition::L,
    DigitPosition::LR,
];
const ALL_DIGITS: &'static [Digit] = &[
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

#[derive(Debug)]
enum Either<L, R> {
    Left(L),
    Right(R),
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
enum DigitPosition {
    UL,
    U,
    UR,
    C,
    LL,
    L,
    LR,
}

impl Display for DigitPosition {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            DigitPosition::UL => "Upper Left  ",
            DigitPosition::U  => "Top         ",
            DigitPosition::UR => "Upper Right ",
            DigitPosition::C =>  "Center      ",
            DigitPosition::LL => "Lower Left  ",
            DigitPosition::L  => "Bottom      ",
            DigitPosition::LR => "Lower Right ",
        };
        write!(f, "{}", s)
    }
}

struct Result {
    solved: HashMap<DigitPosition, Solved>,
    unsolved: HashMap<DigitPosition, Unsolved>,
}

impl Result {
    pub fn new() -> Result {
        let mut result = Result { solved: HashMap::new(), unsolved: HashMap::new() };

        for p in ALL_DIGIT_POSITIONS {
            result.unsolved.insert(p.clone(), CharSet::new(ALL_CHARACTERS.to_string()));
        }

        return result;
    }

    pub fn get(&self, p: &DigitPosition) -> Either<Solved, Unsolved> {
        if self.solved.contains_key(p) {
            return Either::Left(self.solved.get(p).unwrap().clone());
        }

        return Either::Right(self.unsolved.get(p).unwrap().clone());
    }

    pub fn set_solved(&mut self, p: &DigitPosition, c: char) {
        self.solved.insert(p.clone(), c);

        self.unsolved.remove(p);

        let newly_solved: Vec<(DigitPosition, char)> = self.unsolved.iter_mut().fold(Vec::new(), |mut next, x| {
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

impl Display for Either<Solved, Unsolved> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Either::Left(v) =>  write!(f, "(Solved  ) {}", v),
            Either::Right(v) => write!(f, "(Unsolved) {}", v),
        }
    }
}

impl Display for Result {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let result = writeln!(f, "== Solution ==");
        for p in ALL_DIGIT_POSITIONS.iter() {
            result.map(|_| writeln!(f, " {} {}", p, &self.get(p)));
        }
        result.map(|_| writeln!(f, "=============="));

        return Ok(());
    }
}

fn validate(output: &HashMap<Digit, HashSet<CharSet>>, result: &Result, position: Option<DigitPosition>, digit: Option<Digit>) {
    if let Some(position) = position {
        let position_to_digits: HashMap<DigitPosition, HashSet<Digit>> = HashMap::from([
            (DigitPosition::UL, HashSet::from([Digit::Zero, Digit::Four, Digit::Five, Digit::Six, Digit::Eight, Digit::Nine])),
            (DigitPosition::U,  HashSet::from([Digit::Zero, Digit::Two, Digit::Three, Digit::Five, Digit::Six, Digit::Seven, Digit::Eight, Digit::Nine])),
            (DigitPosition::UR, HashSet::from([Digit::Zero, Digit::One, Digit::Two, Digit::Three, Digit::Four, Digit::Seven, Digit::Eight, Digit::Nine])),
            (DigitPosition::C,  HashSet::from([Digit::Two, Digit::Three, Digit::Four, Digit::Five, Digit::Six, Digit::Eight, Digit::Nine])),
            (DigitPosition::LL, HashSet::from([Digit::Zero, Digit::Two, Digit::Six, Digit::Eight])),
            (DigitPosition::L,  HashSet::from([Digit::Zero, Digit::Two, Digit::Three, Digit::Five, Digit::Six, Digit::Eight, Digit::Nine])),
            (DigitPosition::LR, HashSet::from([Digit::Zero, Digit::One, Digit::Three, Digit::Four, Digit::Five, Digit::Six, Digit::Seven, Digit::Eight, Digit::Nine])),
        ]);

        let c = result.solved[&position];
        let valid_digits = position_to_digits[&position].clone();

        for d in ALL_DIGITS {
            if valid_digits.contains(d) {
                let char_sets = output[d].clone();
                if !char_sets.iter().fold(true, |result, char_set| {
                    result && char_set.contains(&c)
                }) {
                    panic!("Validation failed for position {:?} as {:?}. Digit {:?} is valid, but one of the charsets didn't contain {:?}. CharSets={:?}", position, c, d, c, char_sets);
                }
            } else {
                let char_sets = output[d].clone();
                if char_sets.iter().fold(false, |result, char_set| {
                    result || char_set.contains(&c)
                }) {
                    panic!("Validation failed for position {:?} as {:?}. Digit {:?} is invalid, and one of the charsets contained {:?}. CharSets={:?}", position, c, d, c, char_sets);
                }
            }
        }
    }

    if let Some(digit) = digit {
        let digit_to_positions: HashMap<Digit, HashSet<DigitPosition>> = HashMap::from([
            (Digit::Zero,  HashSet::from([DigitPosition::UL, DigitPosition::U, DigitPosition::UR, DigitPosition::LL, DigitPosition::L, DigitPosition::LR])),
            (Digit::One,   HashSet::from([DigitPosition::UR, DigitPosition::LR])),
            (Digit::Two,   HashSet::from([DigitPosition::U, DigitPosition::UR, DigitPosition::C, DigitPosition::LL, DigitPosition::L])),
            (Digit::Three, HashSet::from([DigitPosition::U, DigitPosition::UR, DigitPosition::C, DigitPosition::L, DigitPosition::LR])),
            (Digit::Four,  HashSet::from([DigitPosition::UL, DigitPosition::C, DigitPosition::UR, DigitPosition::LR])),
            (Digit::Five,  HashSet::from([DigitPosition::UL, DigitPosition::U, DigitPosition::C, DigitPosition::L, DigitPosition::LR])),
            (Digit::Six,   HashSet::from([DigitPosition::UL, DigitPosition::U, DigitPosition::C, DigitPosition::LL, DigitPosition::L, DigitPosition::LR])),
            (Digit::Seven, HashSet::from([DigitPosition::U, DigitPosition::UR, DigitPosition::LR])),
            (Digit::Eight, HashSet::from([DigitPosition::UL, DigitPosition::U, DigitPosition::UR, DigitPosition::C, DigitPosition::LL, DigitPosition::L, DigitPosition::LR])),
            (Digit::Nine,  HashSet::from([DigitPosition::UL, DigitPosition::U, DigitPosition::UR, DigitPosition::C, DigitPosition::L, DigitPosition::LR])),
        ]);
    }
}

fn dump(data: &HashMap<Digit, HashSet<CharSet>>) {
    let digits = [Digit::Zero, Digit::One, Digit::Two, Digit::Three, Digit::Four, Digit::Five, Digit::Six, Digit::Seven, Digit::Eight, Digit::Nine];

    println!("---------------- LINE ---------------");

    digits.iter().for_each(|d| {
        print!(" > Digit {:?}\t", d);
        print!(": ");
        for x in data.get(&d).unwrap().iter() {
            print!("{} ", x);
        }
        print!("\n")
    });

    println!("-------------------------------------");
}

fn get_chars(output: &HashMap<Digit, HashSet<CharSet>>, digit: Digit) -> CharSet {
    output.get(&digit).clone().unwrap().into_iter().fold(CharSet::empty(), |result, x| {
        result.merged(x.clone())
    })
}

fn get_code(output: &HashMap<Digit, HashSet<CharSet>>, digit: Digit) -> CharSet {
    output[&digit].clone().into_iter().collect::<Vec<CharSet>>().first().map(|x| x.clone()).unwrap()
}

fn set_op(a: CharSet, b: CharSet, op: fn(HashSet<char>, HashSet<char>) -> Vec<char>) -> CharSet {
    CharSet::new(
        op(a.to_set(), b.to_set()).into_iter().map(|c| c.to_string()).collect::<Vec<String>>().join("")
    )
}

fn difference(a: CharSet, b: CharSet) -> CharSet {
    set_op(a, b, |a, b| a.difference(&b).map(|c| *c).collect())
}

fn union(a: CharSet, b: CharSet) -> CharSet {
    set_op(a, b, |a, b| a.union(&b).map(|c| *c).collect())
}

fn intersection(a: CharSet, b: CharSet) -> CharSet {
    set_op(a, b, |a, b| a.intersection(&b).map(|c| *c).collect())
}

fn filter_candidates<F>(candidates: &HashSet<CharSet>, f: F) -> HashSet<CharSet> where F: Fn(&CharSet) -> bool {
    HashSet::from_iter(candidates.iter().filter(|c| f(*c)).map(|c| c.clone()))
}

fn decode(decoder: &HashMap<Digit, CharSet>, s: CharSet) -> &str {
    decoder.iter().find(|x| x.1.eq(&s)).map(|x| {
        match x.0 {
            Digit::Zero => "0",
            Digit::One => "1",
            Digit::Two => "2",
            Digit::Three => "3",
            Digit::Four => "4",
            Digit::Five => "5",
            Digit::Six => "6",
            Digit::Seven => "7",
            Digit::Eight => "8",
            Digit::Nine => "9"
        }
    }).unwrap()
}

fn main() {
    println!("Starting Day 8b");
    println!("Descrambling display");

    let input = DigitPatterns::from_str(
        utils::read_input("./input/input.txt").trim()
    ).unwrap();

    let solution = input.patterns.into_iter().fold(0_u32, |total, patterns| {
        let digits = patterns.codes.iter().map(|code| {
            DigitResult::from_str(code.as_str()).unwrap()
        }).collect::<Vec<DigitResult>>();

        let mut output = digits.into_iter().fold(HashMap::new(), |mut result, digit| {
            match digit {
                DigitResult::Resolved(d, chars) => {
                    if !result.contains_key(&d) {
                        result.insert(d.clone(), HashSet::new());
                    }
                    result.get_mut(&d.clone()).unwrap().insert(chars.clone());
                },
                DigitResult::Possibilities(ds, chars) => {
                    ds.into_iter().for_each(|d| {
                        if !result.contains_key(&d) {
                            result.insert(d.clone(), HashSet::new());
                        }
                        result.get_mut(&d.clone()).unwrap().insert(chars.clone());
                    });
                }
            }
            result
        });

        let mut result = Result::new();

        // Difference between seven and one is C
        // Since Seven and One both have a unique number of segments; This reveals C
        result.set_solved(
            &DigitPosition::U,
            difference(get_chars(&output, Digit::Seven), get_chars(&output, Digit::One)).only()
        );

        validate(&output, &result, Some(DigitPosition::U), None);

        // Get UL & C
        let mut x = difference(get_chars(&output, Digit::Four), get_chars(&output, Digit::One));

        // Zero, Two, or Three should all have one of UL & C set. Any combination with both or neither UL & C set can be disqualified
        for d in [Digit::Zero, Digit::Two, Digit::Three] {
            output.insert(d.clone(), filter_candidates(&output[&d], |c| {
                intersection(x.clone(), c.clone()).len() == 1
            }));
        }

        result.set_solved(
            &DigitPosition::C,
            difference(x.clone(), get_chars(&output, Digit::Zero)).only()
        );

        // Zero, One, Seven should not have C set
        for d in [Digit::Zero, Digit::One, Digit::Seven] {
            output.insert(d.clone(), filter_candidates(&output[&d], |c| {
                !c.contains(&result.solved[&DigitPosition::C])
            }));
        }

        // And the reset should have C
        for d in [Digit::Two, Digit::Three, Digit::Four, Digit::Five, Digit::Six, Digit::Eight, Digit::Nine] {
            output.insert(d.clone(), filter_candidates(&output[&d], |c| {
                c.contains(&result.solved[&DigitPosition::C])
            }));
        }

        // Since x is UL&C, removing C reveals UL
        x.remove(result.solved[&DigitPosition::C]);

        result.set_solved(&DigitPosition::UL, x.only());

        // Next get the digits for Seven
        let x = get_chars(&output, Digit::Seven);

        // Neither Two, Five, and Six should have all 3 characters in Seven set
        for d in [Digit::Two, Digit::Five, Digit::Six] {
            output.insert(d.clone(), filter_candidates(&output[&d], |c| {
                intersection(x.clone(), c.clone()).len() < 3
            }));
        }

        // Three and Nine should should have all 3 characters in Seven set
        for d in [Digit::Three, Digit::Nine] {
            output.insert(d.clone(), filter_candidates(&output[&d], |c| {
                intersection(x.clone(), c.clone()).len() == 3
            }));
        }

        for digit in ALL_DIGITS {
            let codes = output[digit].iter().map(|x| x.clone()).collect::<Vec<CharSet>>();

            if codes.len() == 1 {
                for d2 in ALL_DIGITS {
                    if *digit != *d2 {
                        output.get_mut(d2).map(|x| x.remove(codes.first().unwrap()));
                    }
                }
            }
        }

        // UR is not in Six
        result.set_solved(
            &DigitPosition::UR,
            difference(result.unsolved.get(&DigitPosition::UR).map(|x| x.clone()).unwrap(), get_chars(&output, Digit::Six)).only()
        );

        // LR is the only one in One
        result.set_solved(
            &DigitPosition::LR,
            intersection(result.unsolved.get(&DigitPosition::LR).map(|x| x.clone()).unwrap(), get_chars(&output, Digit::One)).only()
        );

        // L is the only one in Three
        result.set_solved(
            &DigitPosition::L,
            intersection(result.unsolved.get(&DigitPosition::L).map(|x| x.clone()).unwrap(), get_chars(&output, Digit::Three)).only()
        );

        validate(&output, &result, Some(DigitPosition::C), None);

        let decoder = ALL_DIGITS.iter().fold(HashMap::new(), |mut result, d| {
            result.insert(d.clone(), get_code(&output, d.clone()));
            return result;
        });

        let value = patterns.output.into_iter().fold("".to_string(), |result, code| {
            result + decode(&decoder, CharSet::new(code))
        });

        return total + value.parse::<u32>().unwrap();
    });

    println!("Solution: {:?}", solution);
}
