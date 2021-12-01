mod read;

#[derive(PartialEq)]
enum VariationType { Decrement, Constant, Increment }

fn count_increments(data: &Vec<i64>, window: usize) -> usize {

    let mut info: Vec<VariationType> = Vec::new();
    let mut previous: Option<i64> = None;
    let mut sum: i64 = 0;

    for index in 0..(data.len() - window + 1) {

        for sub_index in index..(index + window) {
            let value = data[sub_index];
            sum = sum + value;
        }

        match (previous, sum) {
            (None, _) => info.push(VariationType::Constant),
            (Some(prev_value), _) if sum == prev_value => info.push(VariationType::Constant),
            (Some(prev_value), _) if sum > prev_value => info.push(VariationType::Increment),
            (Some(prev_value), _) if sum < prev_value => info.push(VariationType::Decrement),
            (Some(_), _) => {},
        }

        previous = Some(sum);
        sum = 0;
    }

    let increments: usize = info.iter()
        .filter(|&variation| *variation == VariationType::Increment)
        .count();

    return increments;
}

fn main() {

    let data = read::read_int_lines("input.txt".to_owned());
    // Part 1
    println!("📈 Increments found (window of 1): {}", count_increments(&data, 1));
    // Part 2
    println!("📈 Increments found (window of 3): {}", count_increments(&data, 3));
}
