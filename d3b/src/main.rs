use std::cmp::max;
use std::collections::hash_map::Keys;
use std::collections::hash_set::Iter;
use std::str::FromStr;
use std::collections::HashSet;
use std::iter::Map;
use y2021::diagnostic_reading::{DiagnosticReading, Selection};
use y2021::utils;
use y2021::submarine;

fn select_indices(objects: Vec<char>, f: fn(&char) -> bool) -> HashSet<usize> {
    objects.clone().iter().enumerate()
        .filter(|(i, c)| f(*c))
        .map(|(i, c)| i)
        .collect::<HashSet<usize>>()
}

fn process(mut report: DiagnosticReading, choose: fn((HashSet<usize>, HashSet<usize>)) -> Selection) -> u32 {
    for position in 0..report.cols {
        let counts = (
            select_indices(report.vslice(position), |c| *c == '0'),
            select_indices(report.vslice(position), |c| *c == '1'),
        );

        report = report.without_rows(choose(counts).remove.iter());

        if report.rows < 2 {
            break;
        }
    }

    y2021::utils::binary_to_number(report.hslice(0))
}

fn get_oxygen_generator_rating(mut report: DiagnosticReading) -> u32 {
    process(report, |partitions| {
        if partitions.0.len() > partitions.1.len() {
            Selection{ keep: partitions.0, remove: partitions.1 }
        } else {
            Selection{ keep: partitions.1, remove: partitions.0 }
        }
    })
}

fn get_co2_scrubber_rating(mut report: DiagnosticReading) -> u32 {
    process(report, |partitions| {
        if partitions.0.len() <= partitions.1.len() {
            Selection{ keep: partitions.0, remove: partitions.1 }
        } else {
            Selection{ keep: partitions.1, remove: partitions.0 }
        }
    })
}

fn main() {
    println!("Starting Day 3b");
    println!("Reading diagnostic report.");

    let report = DiagnosticReading::new(
        utils::read_input("./input/input.txt").lines().map(|l| l.to_string()).collect::<Vec<String>>()
    );

    let result = (
        get_oxygen_generator_rating(report.clone()),
        get_co2_scrubber_rating(report.clone()),
    );

    println!("Counts: OxyGen={:?}, CO2Scrub={:?}", result.0, result.1);
    println!("Final solution: {}", result.0 * result.1);
}
