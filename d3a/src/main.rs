use std::str::FromStr;
use y2021::diagnostic_reading::DiagnosticReading;
use y2021::utils;
use y2021::submarine;

fn main() {
    println!("Starting Day 3a");

    let report = DiagnosticReading::new(
        utils::read_input("./input/input.txt").lines().map(|l| l.to_string()).collect::<Vec<String>>()
    );

    println!("Reading diagnostic report.");

    let mut result = (
        Vec::new(),
        Vec::new(),
    );

    for column in 0..report.cols {
        let slice = report.vslice(column);
        let counts = (
          slice.iter().filter(|c| **c == '0').count(),
          slice.iter().filter(|c| **c == '1').count(),
        );

        if counts.0 > counts.1 {
            result.0.push('0');
            result.1.push('1');
        } else {
            result.0.push('1');
            result.1.push('0');
        }
    }

    let values = (
        utils::binary_to_number(result.0),
        utils::binary_to_number(result.1),
    );

    println!("Diagnostic result: gamma={}, epsilon={}, result={}", values.0, values.1, values.0 * values.1);
    // println!("Final solution: {}", position.horizontal * position.depth);
}
