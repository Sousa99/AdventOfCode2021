mod read;
mod lib;

use lib::{Dimension, ArithmeticLogicUnit};

fn main() {

    
    let lines = read::read_lines("input.txt".to_owned());
    // Manually analysizing each digit
    manually_understanding_stuff(lines.clone());
    println!();

    let mut valid_values : Vec<i64> = Vec::new();
    for value_1 in 1..=9 {
        // Efficiency: Jump over invalid: Rule on 14º
        if value_1 <= 1 { continue }

        for value_2 in 1..=9 {
            // Efficiency: Jump over invalid: Rule on 13º
            if value_2 <= 6 { continue }

            for value_3 in 1..=9 {
                // Efficiency: Jump over invalid: Rule on 4º
                if value_3 >= 7 { continue }

                for value_4 in 1..=9 {
                    // Jump over invalid: Rule on 4º
                    if value_3 + 3 != value_4 { continue }
                    
                    for value_5 in 1..=9 {
                        // Efficiency: Jump over invalid: Rule on 12º
                        if value_5 >= 2 { continue }
                        
                        for value_6 in 1..=9 {
                            // Efficiency: Jump over invalid: Rule on 9º
                            if value_6 >= 9 { continue }
                            
                            for value_7 in 1..=9 {
                                // Efficiency: Jump over invalid: Rule on 8º
                                if value_7 <= 8 { continue }
                                
                                for value_8 in 1..=9 {
                                    // Jump over invalid: Rule on 8º
                                    if value_7 - 8 != value_8 { continue }

                                    for value_9 in 1..=9 {
                                        // Jump over invalid: Rule on 9º
                                        if value_6 + 1 != value_9 { continue }

                                        for value_10 in 1..=9 {
                                            // Efficiency: Jump over invalid: Rule on 11º
                                            if value_10 >= 8 { continue }

                                            for value_11 in 1..=9 {
                                                // Jump over invalid: Rule on 11º
                                                if value_10 + 2 != value_11 { continue }

                                                for value_12 in 1..=9 {
                                                    // Jump over invalid: Rule on 12º
                                                    if value_5 + 8 != value_12 { continue }

                                                    for value_13 in 1..=9 {
                                                        // Jump over invalid: Rule on 13º
                                                        if value_2 - 6 != value_13 { continue }

                                                        for value_14 in 1..=9 {
                                                            // Jump over invalid: Rule on 14º
                                                            if value_1 - 1 != value_14 { continue }

                                                            let mut alu = ArithmeticLogicUnit::new(lines.clone());
                                                            let input : Vec<i64> = vec!(value_1, value_2, value_3, value_4, value_5, value_6, value_7,
                                                                value_8, value_9, value_10, value_11, value_12, value_13, value_14).into_iter().rev().collect();
                                                            alu.process_input(input.clone());
                                                            let result = alu.get_dimension(Dimension::Z);
                                                            if result == 0 {
                                                                let number : i64 = input.into_iter().enumerate()
                                                                    .map(|(digit_index, digit)| (10 as i64).pow(digit_index as u32) * digit)
                                                                    .sum();
                                                                valid_values.push(number);
                                                            }

                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    // Part 1
    let maximum_valid = valid_values.iter().max().unwrap();
    println!("\r🔒  The 'maximum' valid Model Number is '{}' (Part 1)", maximum_valid);
    
    // Part 2
    let minimum_valid = valid_values.iter().min().unwrap();
    println!("\r🔒  The 'minimum' valid Model Number is '{}' (Part 2)", minimum_valid);
}

fn manually_understanding_stuff(lines: Vec<String>) {

    let mut lines_sep : Vec<Vec<String>> = Vec::new();
    let mut current_alu_lines : Vec<String> = Vec::new();
    for line in lines.into_iter() {
        if line.contains(&"inp".to_owned()) {
            if current_alu_lines.len() > 0 { lines_sep.push(current_alu_lines); }
            current_alu_lines = Vec::new();
        }
        current_alu_lines.push(line);
    }
    lines_sep.push(current_alu_lines);

    // ================================================== Start testing out ==================================================
    create_and_test_alu(1, lines_sep[0].clone(), (vec!(5), 0, 0, 0, 0));
    create_and_test_alu(2, lines_sep[1].clone(), (vec!(8), 1, 5, 5, 5));
    create_and_test_alu(3, lines_sep[2].clone(), (vec!(6), 1, 11, 141, 8));
    create_and_test_alu(4, lines_sep[3].clone(), (vec!(9), 1, 14, 3680, 6));
    create_and_test_alu(5, lines_sep[4].clone(), (vec!(1), 0, 0, 141, 9));
    create_and_test_alu(6, lines_sep[5].clone(), (vec!(4), 1, 14, 3680, 1));
    create_and_test_alu(7, lines_sep[6].clone(), (vec!(9), 1, 13, 95693, 4));
    create_and_test_alu(8, lines_sep[7].clone(), (vec!(1), 1, 15, 2488033, 9));
    create_and_test_alu(9, lines_sep[8].clone(), (vec!(5), 0, 0, 95693, 1));
    create_and_test_alu(10, lines_sep[9].clone(), (vec!(4), 0, 0, 3680, 5));
    create_and_test_alu(11, lines_sep[10].clone(), (vec!(6), 1, 6, 95686, 4));
    create_and_test_alu(12, lines_sep[11].clone(), (vec!(9), 0, 0, 3680, 6));
    create_and_test_alu(13, lines_sep[12].clone(), (vec!(2), 0, 0, 141, 9));
    create_and_test_alu(14, lines_sep[13].clone(), (vec!(4), 0, 0, 5, 2));
    
    // ================================================== Understanding Lower Bound ==================================================
    // When processing 4º:  if 3º   + 3 == 4º   => z decreases
    // When processing 8º:  if 7º   - 8 == 8º   => z decreases
    // When processing 9º:  if 6º   + 1 == 9º   => z decreases
    // When processing 11º: if 10º  + 2 == 11º  => z decreases
    // When processing 12º: if 5º   + 8 == 12º  => z decreases
    // When processing 13º: if 2º   - 6 == 13º  => z decreases
    // When processing 13º: if 1º   - 1 == 14º  => z decreases
}

fn create_and_test_alu(digit: usize, lines: Vec<String>, input: (Vec<i64>, i64, i64, i64, i64)) {
    let mut current_alu = ArithmeticLogicUnit::new(lines.clone());
    current_alu.mannual_fix_dimension(Dimension::X, input.1);
    current_alu.mannual_fix_dimension(Dimension::Y, input.2);
    current_alu.mannual_fix_dimension(Dimension::Z, input.3);
    current_alu.mannual_fix_dimension(Dimension::W, input.4);
    current_alu.process_input(input.0);

    println!("🧑  Studying Digit {: >2}: {}", digit, current_alu);
}