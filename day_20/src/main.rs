mod read;
mod lib;

use lib::{EnhancingTool};

fn main() {

    let lines = read::read_lines("input.txt".to_owned());
    let codification_line = lines.get(0).unwrap().clone();
    let image_lines = lines[2..].to_vec();

    let mut tool : EnhancingTool = EnhancingTool::new(codification_line, image_lines);

    // Part 1
    while tool.get_current_iteration() != 2 { tool.do_iteration() }
    let number_filled = tool.compute_finite_number_of_characther('#');
    println!("📦  The total number of filled pixels on the image after '2' enhancements was '{}' (Part 1)", number_filled);
    
    // Part 2
    while tool.get_current_iteration() != 50 { tool.do_iteration() }
    let number_filled = tool.compute_finite_number_of_characther('#');
    println!("📦  The total number of filled pixels on the image after '50' enhancements was '{}' (Part 2)", number_filled);
}
