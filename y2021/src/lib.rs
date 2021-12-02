
pub mod utils {
    use std::fs;

    pub fn read_input(file: &str) -> String {
        println!("Reading puzzle input: {}", file);

        return fs::read_to_string(file)
            .expect(format!("Unable to read the file {}", file).as_str());
    }

    pub fn transform_lines_to_integers(lines: &str) -> Vec<i32> {
        return lines.lines().map(|l| l.parse::<i32>().unwrap()).collect();
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
