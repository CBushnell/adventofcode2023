use std::fs;
use std::io;
use std::collections::HashSet;
use regex::Regex;
use ndarray::{Array2, s, Dim};

const INPUT_FILEPATH: &str = "src\\input.txt";

fn main() {

    let items: Vec<String> = read_file_to_vect(INPUT_FILEPATH);
    let (graph, all_numbers)= build_graph(&items);
    println!("{:?}", graph);
    let padded_graph = pad_array(&graph);

    let numbers = get_adjacent_numbers(&items, &padded_graph, &all_numbers);
    println!("{:?}" , numbers);

    let sum: i32 = numbers.iter().sum();

    println!("{}", &sum.to_string());
}

fn read_file_to_vect(filepath: &str) -> Vec<String> {
    let contents: Result<String, io::Error> = fs::read_to_string(filepath);
    contents.expect("Didn't load correctly").lines().map(|line| line.to_string()).collect()
}

fn build_graph(contents: &Vec<String>) -> (Array2<usize>, Vec<i32>)  {
    
    let num = Regex::new(r"\d+").unwrap();

    let row_length = contents[0].len();

    //matrix to hold IDs that map to 'all_numbers' + 1 (because of usize)
    let mut matrix:ndarray::ArrayBase<ndarray::OwnedRepr<usize>, ndarray::Dim<[usize; 2]>> = Array2::<usize>::zeros((contents.len(), row_length));

    let mut all_numbers: Vec<i32> = Vec::new(); 

    for (i, e) in contents.iter().enumerate() {
        //get current size of the 'all_numbers' vec for assigning IDs
        let start_index: usize = all_numbers.len();

        //gathering numbers and their digit length from the input
        let numbers: Vec<&str> = num.find_iter(e).map(|m| m.as_str()).collect();
        let number_lengths: Vec<usize> = numbers.iter().map(|n| n.len()).collect();

        //The ID to be added to the 'all_numbers' vec - will be referenced by their ID in the matrix
        let numbers_i32: Vec<i32> = numbers.iter().map(|m| m.parse::<i32>().unwrap()).collect();
        all_numbers.extend(numbers_i32);

        //map from Array -> all_numbers
        let indexes: Vec<usize> = num.find_iter(e).map(|m| start_index + m.start()).collect();

        //populate array with value IDs
        for (j, index) in indexes.iter().enumerate() {
            let start_spot: usize = index - start_index;
            let end_spot: usize = index - start_index + number_lengths[j];
            matrix.slice_mut(s![i, start_spot..end_spot]).fill(j + start_index + 1)
        }
    }
    return (matrix, all_numbers)
} 

fn pad_array(array: &Array2<usize>) -> Array2<usize> {
    
    let padded_shape: Vec<usize> = array.shape().iter().map(|n| n + 2).collect();
    let mut padded_array = Array2::zeros(Dim([padded_shape[0], padded_shape[1]]));

    padded_array.slice_mut(s![1..padded_shape[0] - 1, 1..padded_shape[1] - 1]).assign(array);
    println!("{:?}", padded_array);
    return padded_array
} 

fn get_adjacent_numbers(contents: &Vec<String>, matrix: &Array2<usize>, numbers: &Vec<i32>) -> Vec<i32> {
    // let symbol = Regex::new(r"[\!\@\#\$\%\^\&\*\\\/\=\-\+]").unwrap();
    let symbol = Regex::new(r"\*").unwrap();

    let mut matched_numbers: Vec<i32> = Vec::new();

    // let (lower_bound, right_bound) = (matrix.shape()[0] - 1, matrix.shape()[0] - 1);

    for (i, e) in contents.iter().enumerate() {
        //locations of the symbols in the row, +1 to align with the padded matrix
        let indexes: Vec<usize> = symbol.find_iter(e).map(|m| m.start() + 1).collect();
        //adjusting the i + 1 to align with the padded matrix (ex. i = 0 is actually i = 1 in the matrix)
        let true_i = i + 1;

        for index in indexes {
            let row_start: usize = true_i - 1;
            let row_end: usize = true_i + 2;
            let col_start: usize = index - 1;
            let col_end: usize = index + 2;
            let slice: ndarray::ArrayBase<ndarray::ViewRepr<&usize>, Dim<[usize; 2]>> = matrix.slice(s![row_start..row_end, col_start..col_end]);
            let mut unique_values: Vec<&usize> = slice.into_iter().collect::<HashSet<_>>().into_iter().collect::<Vec<_>>();
            unique_values.retain(|x| **x != 0);
            if unique_values.len() == 2 {
                let converted_unique_values: Vec<usize> = unique_values.iter().map(|x| **x - 1).collect();
                println!("{:?}", converted_unique_values);
                let locally_matched_numbers: Vec<i32> = converted_unique_values.iter().map(|id| numbers[*id]).collect();
                let ratio = locally_matched_numbers[0] * locally_matched_numbers[1];
                matched_numbers.extend(vec![ratio]);
            }
        }

    }
    matched_numbers
}