mod read;
mod lib;

use lib::{EntryProblem, Display};

fn main() {

    let mut entry_problems : Vec<EntryProblem> = Vec::new();

    let lines = read::read_lines("input.txt".to_owned());
    for line in lines {

        let line_split : Vec<&str> = line.split(" | ").collect();
        let input = line_split[0];
        let output = line_split[1];

        let patterns : Vec<String> = input.split_whitespace().map(|code| code.to_owned()).collect();
        let output_coddes : Vec<String> = output.split_whitespace().map(|code| code.to_owned()).collect();

        entry_problems.push(EntryProblem::new(patterns, output_coddes));
    }

    let mut display : Display = Display::new(entry_problems);
    display.solve_entries();

    // Part 1
    let count_1 : usize = display.number_of_digits(vec!(1, 4, 7, 8));
    println!("🖵  The number of {{1, 4, 7, 8}}s is '{}' (Part 1)", count_1);
    
    // Part 2
    let count_2 : u64 = display.sum_outputs();
    println!("🖵  The sum of all outputs is '{}' (Part 2)", count_2);
}