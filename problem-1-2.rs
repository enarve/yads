// Problem 
// Smallest older-indexed counterpart for each value in a row

use std::{fs, path};

fn main() {
    // Reading
    let input = read_input();
    let mut input_parts = input.split("\n");
    let n = input_parts.next()
        .expect("Can't find number of values from input.")
        .parse::<usize>()
        .expect("n is not a number");
    let values: Vec<i64> = input_parts.next()
        .expect("Can't read values from input.")
        .split(" ")
        .map(|s| s.parse::<i64>().expect("Error reading value."))
        .collect();

    // Finding problem solution
    let result: Vec<i64> = get_result(values, n);

    // Output
    let result_strings: Vec<String> = result.iter().map(|i| i.to_string()).collect();
    let formatted_res_string = format!("{}", result_strings.join(" "));
    write_output(formatted_res_string);
}

// For each value get appropriate pair: smallest value with bigger index
fn get_result(values: Vec<i64>, length: usize) -> Vec<i64> {
    assert!(values.len() == length);
    let mut result: Vec<i64> = vec![];
    for (i, a) in values.iter().enumerate() {
        let mut min_index: i64 = -1;
        for (j, b) in values.iter().enumerate() {
            if i < j && b > a  {
                min_index = (j + 1) as i64;
                break;
            }
        }
        result.push(min_index);
    }
    result
}

fn read_input() -> String {
    let input_path = path::Path::new("input/input-1-2.txt");
    fs::read_to_string(input_path)
        .expect("Found no input file.")
}

fn write_output(output: String) {
    let output_path = path::Path::new("output/output-1-2.txt");
    let prefix = output_path.parent().unwrap();
    fs::create_dir_all(prefix).expect("Could not create output dir.");
    fs::write(output_path, output)
        .expect("Error writing an output file.")
}