use std::fs::File;
use std::io::{BufRead, BufReader, Error, ErrorKind};

pub fn read_lines(filename: String) -> Vec<String> {

    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);

    let data: Vec<String> = reader.lines()
        .collect::<Result<_, _>>().unwrap();

    return data;
}

pub fn read_int_lines(filename: String) -> Vec<i64> {

    let data_string: Vec<String> = read_lines(filename);
    let data: Vec<i64> = data_string.iter()
        .map(|line| line.parse().map_err(|e| Error::new(ErrorKind::InvalidData, e)))
        .collect::<Result<_, _>>().unwrap();

    return data;
}