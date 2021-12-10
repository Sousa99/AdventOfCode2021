mod read;
mod lib;

use lib::NavigationSubsystem;

fn main() {

    let characthers : Vec<Vec<char>> = read::read_chars("input.txt".to_owned());
    let mut navigation_subsystem : NavigationSubsystem = NavigationSubsystem::new(characthers);
    navigation_subsystem.evaluate_lines();

    // Part 1
    let count_1 : u64 = navigation_subsystem.compute_syntax_error_score();
    println!("🗺️  The computed syntax error score is '{}' (Part 1)", count_1);
    
    // Part 2
    let count_2 : u64 = navigation_subsystem.compute_middle_completion_score();
    println!("🗺️  The computed syntax completion score is '{}' (Part 2)", count_2);
}