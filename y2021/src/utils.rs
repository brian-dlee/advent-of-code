use std::fs;

pub fn binary_to_number(input: Vec<char>) -> u32 {
    input.iter().rev().enumerate().fold(0_u32, |result, (i, c)| {
        match c {
            '1' => result + 2_u32.pow(i as u32),
            _ => result
        }
    })
}

pub fn read_input(file: &str) -> String {
    println!("Reading puzzle input: {}", file);

    return fs::read_to_string(file)
        .expect(format!("Unable to read the file {}", file).as_str());
}

pub fn transform_lines_to_integers(lines: &str) -> Vec<i32> {
    return lines.lines().map(|l| l.parse::<i32>().unwrap()).collect();
}

pub fn first<T: Clone>(iterable: Vec<T>) -> Option<T> {
    for item in iterable.iter() {
        return Some(item.clone());
    }

    return None;
}
