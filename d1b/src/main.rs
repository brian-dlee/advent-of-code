use y2021::utils;

fn main() {
    println!("Starting Day 1");

    let contents = utils::read_input("../input/d1b/input.txt");
    let numbers = utils::transform_lines_to_integers(contents.as_str());

    println!("Finding number of increases in puzzle input.");
    
    let mut increases = 0;
    let mut previous: Option<i32> = None;

    if numbers.len() > 2 {
        for i in 2..numbers.len() {
            let total: i32 = numbers[i-2..=i].into_iter().fold(0, |a, b| a + b);

            if let Some(n) = previous {
                if total > n {
                    increases += 1;
                }
            }

            previous = Some(total)        
        }
    }

    println!("Number of increases: {}", increases);
}
