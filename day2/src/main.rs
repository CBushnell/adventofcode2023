
use std::fs;
use std::io;
use regex::Regex;

// const RED: i32 = 12;
// const GREEN: i32 = 13;
// const BLUE: i32 = 14;

fn main() {

    let items: Vec<String> = read_file_to_vect("src\\input.txt");

    let mut sum: i32 = 0;
    for e in items {
        sum += assess_element(&e);
    }
    println!("{}", &sum.to_string());
}

fn read_file_to_vect(filepath: &str) -> Vec<String> {
    let contents: Result<String, io::Error> = fs::read_to_string(filepath);
    contents.expect("Didn't load correctly").lines().map(|line| line.to_string()).collect()
}

fn assess_element(element: &str) -> i32 {

    // let game_regex = Regex::new(r"Game \d+").unwrap();
    let red_regex = Regex::new(r"\d+ red").unwrap();
    let green_regex = Regex::new(r"\d+ green").unwrap();
    let blue_regex = Regex::new(r"\d+ blue").unwrap();
    let num: Regex = Regex::new(r"\d+").unwrap();

    // let game: Vec<_> = game_regex.find_iter(element).map(|m| m.as_str()).collect();
    let reds: Vec<_> = red_regex.find_iter(element).map(|m| m.as_str()).collect();
    let greens: Vec<_> = green_regex.find_iter(element).map(|m| m.as_str()).collect();
    let blues: Vec<_> = blue_regex.find_iter(element).map(|m| m.as_str()).collect();

    println!("{:?}", greens);

    let mut red_max = -1;
    let mut green_max = -1;
    let mut blue_max = -1;
    
    for e in reds {
        let count: Vec<_> = num.find_iter(e).map(|m| m.as_str()).collect();
        let red_count: i32 = count[0].parse().unwrap();
        if red_count > red_max {
            red_max = red_count;
        }
    }

    for e in greens {
        let count: Vec<_> = num.find_iter(e).map(|m| m.as_str()).collect();
        let green_count: i32 = count[0].parse().unwrap();
        if green_count > green_max {
            green_max = green_count;
        }
    }

    for e in blues {
        let count: Vec<_> = num.find_iter(e).map(|m| m.as_str()).collect();
        let blue_count: i32 = count[0].parse().unwrap();
        if blue_count > blue_max {
            blue_max = blue_count;
        }
    }

    // let count: Vec<_> = num.find_iter(game[0]).map(|m| m.as_str()).collect();
    // let game_id = count[0].parse().unwrap();
    // return game_id;

    return red_max * blue_max * green_max
}