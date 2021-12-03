mod read;
mod lib;

use lib::Diagnostic;

fn main() {

    let data = read::read_lines("input.txt".to_owned());
    let mut diagnostic = Diagnostic::new(data);
    diagnostic.calculate_rates();

    // Part 1
    let (gamma_rate, epsilon_rate) = diagnostic.get_combustion_rates().unwrap();
    println!("Gamma Rate: {} / {}", gamma_rate.decimal, gamma_rate.binary);
    println!("Epsilon Rate: {} / {}", epsilon_rate.decimal, epsilon_rate.binary);
    println!("🔌 Power Consumption: {}", gamma_rate.decimal * epsilon_rate.decimal);
    
    println!();

    // Part 2
    let (oxygen_rate, dyoxide_rate) = diagnostic.get_support_rates().unwrap();
    println!("Oxygen Rate: {} / {}", oxygen_rate.decimal, oxygen_rate.binary);
    println!("Dyoxide Rate: {} / {}", dyoxide_rate.decimal, dyoxide_rate.binary);
    println!("💕 Support Life Rating: {}", oxygen_rate.decimal * dyoxide_rate.decimal);
}
