mod read;
mod lib;

use lib::{SnailMathProblem};

fn main() {

    let lines = read::read_lines("input.txt".to_owned());
    let mut problem = SnailMathProblem::new(lines);

    // Part 1
    problem.sum_elements();
    let magnitude = problem.get_magnitude();
    println!("🌍  The magnitude of the sum is '{}' (Part 1)", magnitude);
    
    // Part 2
    let max_info = problem.compute_max_sum().unwrap();
    println!("🌍  The maximum magnitude was of '{}' achieved with indexes '{}' and '{}' (Part 2)", max_info.1, max_info.0.0, max_info.0.1);
}
