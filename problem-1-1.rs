// Problem 1.1
// Bracket sequences

use std::{fs, path};

const OPENING: &str = "([{";
const CLOSING: &str = ")]}";
const CORRECT: &str = "CORRECT";

fn main() {
    let input = read_input();
    let output = is_balanced(input);
    write_output(output);
}

// Bracket sequence evaluation
fn is_balanced(sequence: String) -> String {
    let mut result_string = CORRECT.to_string();
    let mut stack: Stack<char> = Stack::new();
    for (i, char) in sequence.chars().enumerate() {
        if OPENING.contains(char) {
            stack.push(char);
        } else {
            match stack.last_element() {
                Some(last_element) => {
                    let bracket_num = *last_element as u32;
                    if [char::from_u32(bracket_num + 1).unwrap(), char::from_u32(bracket_num + 2).unwrap()].contains(&char)  {
                        stack.pop();
                    } else {
                        result_string = format!("{}", i);
                        break;
                    }
                },
                None => {
                    result_string = format!("{}", i);
                    break;
                }
            }
        }
    }
    if result_string == CORRECT && stack.length() != 0 {
        result_string = format!("{}", sequence.len());
    }
    result_string
}

// Stack imitation
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
    let input_path = path::Path::new("input/input-1-1.txt");
    fs::read_to_string(input_path)
        .expect("Found no input file.")
}

fn write_output(output: String) {
    let output_path = path::Path::new("output/output-1-1.txt");
    let prefix = output_path.parent().unwrap();
    fs::create_dir_all(prefix).expect("Could not create output dir.");
    fs::write(output_path, output)
        .expect("Error writing an output file.")
}