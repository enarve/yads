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
    let result: Vec<i64> = get_result_faster(values, n);

    // Output
    let result_strings: Vec<String> = result.iter().map(|i| i.to_string()).collect();
    let formatted_res_string = format!("{}", result_strings.join(" "));
    write_output(formatted_res_string);
}

// For each value get appropriate pair: smallest value with bigger index
fn _get_result(values: Vec<i64>, length: usize) -> Vec<i64> {
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

// Stack version
fn get_result_faster(values: Vec<i64>, length: usize) -> Vec<i64> {
    assert!(values.len() == length);
    let mut result: Vec<i64> = vec![];
    let mut stack: Stack<i64> = Stack::new();

    for (i, value) in values.iter().enumerate() {
        if stack.length() == 0 {
            stack.push(*value);
        } else {
            if value > stack.last_element().unwrap() {
                while stack.length() != 0 {
                    stack.pop();
                    result.push((i+1) as i64);
                }
                stack.push(*value);
            } else {
                stack.push(*value);
            }
        }
    }
    for _ in 0..stack.length() {
        result.push(-1);
    }
    result
}

struct Stack<T> { items: Vec<T> }

impl<T> Stack<T> {
    fn new() -> Self { Self { items: vec![] } }
    fn push(&mut self, item: T) { self.items.push(item); }
    fn pop(&mut self) -> Option<T> { self.items.pop() }
    fn last_element(&self) -> Option<&T> { self.items.last() }
    fn length(&self) -> usize { self.items.len() }
}

// IO

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