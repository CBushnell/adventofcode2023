use std::convert;
use std::collections::HashMap;
use std::fs;
use std::io;
use std::os::windows::process;

fn main() {

    let items: Vec<String> = read_file_to_vect("src\\input.txt");

    let mut sum: i32 = 0;
    for e in items {
        let value: i32 = get_values(&e);
        sum = sum + value;
        println!("{}, running sum is: {}", value.to_string(), sum.to_string());
    }
    println!("{}", &sum.to_string());
}

fn read_file_to_vect(filepath: &str) -> Vec<String> {
    let contents: Result<String, io::Error> = fs::read_to_string(filepath);
    contents.expect("Didn't load correctly").lines().map(|line| line.to_string()).collect()
}

fn get_values(element: &str) -> i32 {
    // Convert word values to digits
    let number_words = vec![
        String::from("zero"),
        String::from("one"),
        String::from("two"),
        String::from("three"),
        String::from("four"),
        String::from("five"),
        String::from("six"),
        String::from("seven"),
        String::from("eight"),
        String::from("nine")
    ];
    let mut indexes: HashMap<usize, usize> = HashMap::new();
    for (i, w) in number_words.iter().enumerate() {
        let mut start_index: usize = 0;
        while &start_index < &element.len() {
            let curr_string = &element[start_index..];
            let relative_index: Option<usize> = curr_string.find(w);
            match relative_index {
                Some(x) => (),
                None => {
                    break;
                }
            }
            let true_index = relative_index.unwrap() + start_index;
            indexes.insert(true_index, i);
            start_index = true_index + 1;
        }
    }
    println!("{:?}", indexes);

    //Get the digits
    for (i, c) in element.chars().enumerate() {
        if c.is_digit(10) {
            let digit = c.to_digit(10).unwrap();
            indexes.insert(i, digit as usize);
        }
    }

    let mut keys: Vec<usize> = indexes.keys().cloned().collect();
    keys.sort();
    println!("{:?}", indexes);

    if keys.len() > 1 {
        let first_digit = indexes.get(&keys[0]).unwrap();
        let second_digit = indexes.get(&keys[&keys.len() - 1]).unwrap();
        let value = format!("{}{}", first_digit.to_string(), second_digit.to_string());
        return value.parse::<i32>().unwrap()
    } 
    
    let digit = indexes.get(&keys[0]);
    match digit {
        Some(x) => (),
        None => {
            println!("number of matches is: {}", keys.len());
            return 0
        }
    }
    let value = format!("{}{}", digit.unwrap().to_string(), digit.unwrap().to_string());
    return value.parse::<i32>().unwrap()

}