use std::borrow::BorrowMut;
use std::collections::{HashMap, HashSet};
use std::str::FromStr;
use std::fmt::{Display, Formatter};
use y2021::display_troubleshooting::{ALL_DIGITS, ALL_DISPLAY_SEGMENTS, CharSet, DigitPatterns, Digit, DisplaySegment, SegmentTranslation, DigitPotential};
use y2021::display_troubleshooting::Digit::Eight;
use y2021::utils;

fn validate_solved_segment(output: &HashMap<Digit, HashSet<CharSet>>, translation: &SegmentTranslation, segment: &DisplaySegment) {
    let segments_to_digits: HashMap<DisplaySegment, HashSet<Digit>> = HashMap::from([
        (DisplaySegment::UL, HashSet::from([Digit::Zero, Digit::Four, Digit::Five, Digit::Six, Digit::Eight, Digit::Nine])),
        (DisplaySegment::U, HashSet::from([Digit::Zero, Digit::Two, Digit::Three, Digit::Five, Digit::Six, Digit::Seven, Digit::Eight, Digit::Nine])),
        (DisplaySegment::UR, HashSet::from([Digit::Zero, Digit::One, Digit::Two, Digit::Three, Digit::Four, Digit::Seven, Digit::Eight, Digit::Nine])),
        (DisplaySegment::C, HashSet::from([Digit::Two, Digit::Three, Digit::Four, Digit::Five, Digit::Six, Digit::Eight, Digit::Nine])),
        (DisplaySegment::LL, HashSet::from([Digit::Zero, Digit::Two, Digit::Six, Digit::Eight])),
        (DisplaySegment::L, HashSet::from([Digit::Zero, Digit::Two, Digit::Three, Digit::Five, Digit::Six, Digit::Eight, Digit::Nine])),
        (DisplaySegment::LR, HashSet::from([Digit::Zero, Digit::One, Digit::Three, Digit::Four, Digit::Five, Digit::Six, Digit::Seven, Digit::Eight, Digit::Nine])),
    ]);

    let c = translation.solved[segment];
    let valid_digits = segments_to_digits[segment].clone();

    for d in ALL_DIGITS {
        if valid_digits.contains(d) {
            let char_sets = output[d].clone();
            if !char_sets.iter().fold(true, |result, char_set| {
                result && char_set.contains(&c)
            }) {
                panic!("Validation failed for position {:?} as {:?}. Digit {:?} is valid, but one of the charsets didn't contain {:?}. CharSets={:?}", segment, c, d, c, char_sets);
            }
        } else {
            let char_sets = output[d].clone();
            if char_sets.iter().fold(false, |result, char_set| {
                result || char_set.contains(&c)
            }) {
                panic!("Validation failed for position {:?} as {:?}. Digit {:?} is invalid, and one of the charsets contained {:?}. CharSets={:?}", segment, c, d, c, char_sets);
            }
        }
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
            DigitPotential::from_str(code.as_str()).unwrap()
        }).collect::<Vec<DigitPotential>>();

        let mut output = digits.into_iter().fold(HashMap::new(), |mut result, digit| {
            match digit {
                DigitPotential::Resolved(d, chars) => {
                    if !result.contains_key(&d) {
                        result.insert(d.clone(), HashSet::new());
                    }
                    result.get_mut(&d.clone()).unwrap().insert(chars.clone());
                },
                DigitPotential::Possibilities(ds, chars) => {
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

        let mut result = SegmentTranslation::new();

        // Difference between seven and one is C
        // Since Seven and One both have a unique number of segments; This reveals C
        result.set_solved(
            &DisplaySegment::U,
            get_chars(&output, Digit::Seven).difference(get_chars(&output, Digit::One)).only()
        );

        // Get UL & C
        let mut x = get_chars(&output, Digit::Four).difference(get_chars(&output, Digit::One));

        // Zero, Two, or Three should all have one of UL & C set. Any combination with both or neither UL & C set can be disqualified
        for d in [Digit::Zero, Digit::Two, Digit::Three] {
            output.insert(d.clone(), filter_candidates(&output[&d], |c| {
                x.clone().intersection(c.clone()).len() == 1
            }));
        }

        // Solved C
        result.set_solved(
            &DisplaySegment::C,
            x.clone().difference(get_chars(&output, Digit::Zero)).only()
        );

        // Zero, One, Seven should not have C set
        for d in [Digit::Zero, Digit::One, Digit::Seven] {
            output.insert(d.clone(), filter_candidates(&output[&d], |c| {
                !c.contains(&result.solved[&DisplaySegment::C])
            }));
        }

        // And the reset should have C
        for d in [Digit::Two, Digit::Three, Digit::Four, Digit::Five, Digit::Six, Digit::Eight, Digit::Nine] {
            output.insert(d.clone(), filter_candidates(&output[&d], |c| {
                c.contains(&result.solved[&DisplaySegment::C])
            }));
        }

        // Since x is UL&C, removing C reveals UL
        x.remove(result.solved[&DisplaySegment::C]);

        // Solved UL
        result.set_solved(&DisplaySegment::UL, x.only());

        // Next get the digits for Seven
        let x = get_chars(&output, Digit::Seven);

        // Neither Two, Five, and Six should have all 3 characters in Seven set
        for d in [Digit::Two, Digit::Five, Digit::Six] {
            output.insert(d.clone(), filter_candidates(&output[&d], |c| {
                x.clone().intersection(c.clone()).len() < 3
            }));
        }

        // Three and Nine should should have all 3 characters in Seven set
        for d in [Digit::Three, Digit::Nine] {
            output.insert(d.clone(), filter_candidates(&output[&d], |c| {
                x.clone().intersection(c.clone()).len() == 3
            }));
        }

        // By process of elimination, eliminate combinations that have been taken by other digits
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
            &DisplaySegment::UR,
            result.get_unsolved(&DisplaySegment::UR).unwrap().difference(get_chars(&output, Digit::Six)).only()
        );

        // LR is the only one in One
        result.set_solved(
            &DisplaySegment::LR,
            result.get_unsolved(&DisplaySegment::LR).unwrap().intersection(get_chars(&output, Digit::One)).only()
        );

        // L is the only one in Three
        result.set_solved(
            &DisplaySegment::L,
            result.get_unsolved(&DisplaySegment::L).unwrap().intersection(get_chars(&output, Digit::Three)).only()
        );

        // Validate solution
        for segment in ALL_DISPLAY_SEGMENTS {
            validate_solved_segment(&output, &result, segment);
        }

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
