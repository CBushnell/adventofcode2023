use std::fs;
use std::io;
use regex::Regex;
use ndarray::Array2;

const input_filepath: &str = "src\\input.txt";

fn main() {

    let items: Vec<String> = read_file_to_vect(input_filepath);
    // let graph: Array2<i32> = build_graph; 
    build_graph(items);

    let mut sum: i32 = 0;


    println!("{}", &sum.to_string());
}

fn read_file_to_vect(filepath: &str) -> Vec<String> {
    let contents: Result<String, io::Error> = fs::read_to_string(filepath);
    contents.expect("Didn't load correctly").lines().map(|line| line.to_string()).collect()
}

fn build_graph(contents: Vec<String>) {
    
    let num = Regex::new(r"\d+").unwrap();

    let row_length = contents[0].len();

    let mut matrix= Array2::<i32>::zeros((contents.len(), row_length));

    for (i, e) in contents.iter().enumerate() {
        let numbers: Vec<_> = num.find_iter(e).map(|m| m.as_str()).collect();
        let indexes = num.find_iter(e);
    }
} 