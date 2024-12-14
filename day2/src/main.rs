use std::{collections::HashMap, fs};

fn main() {
    let input = fs::read_to_string("input/test.txt")
        .expect("no file");
    let mut sum = 0;
    for line in input.lines() {
        let nums: Vec<i32> = line
            .split_whitespace()
            .map(|n| n.parse::<i32>().unwrap())
            .collect();

        let mut safe = true;
        for i in 2..nums.len() {
            
        }
    }    



    println!("{}", sum);
}
