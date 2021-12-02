use y2021::utils;

fn main() {
    println!("Starting Day 1a");

    let contents = utils::read_input("../input/d1a/input.txt");
    let numbers = utils::transform_lines_to_integers(contents.as_str());

    println!("Finding number of increases in puzzle input.");
    
    let mut increases = 0;
    let mut previous: Option<i32> = None;

    for number in numbers {
        if let Some(n) = previous {
            if number > n {
                increases += 1;
            }
        }

        previous = Some(number)        
    }

    println!("Number of increases: {}", increases);
}
