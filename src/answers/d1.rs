use std::{collections::HashMap, fs};

pub fn d1_1(input: String) {
    let mut vLeft: Vec<i32> = vec![];
    let mut vRight: Vec<i32> = vec![];
    for line in input.lines() {
        let nums: Vec<i32> = line
            .split_whitespace()
            .map(|n| n.parse::<i32>().unwrap())
            .collect();
        vLeft.push(nums[0]);
        vRight.push(nums[1]);
    }    
    vLeft.sort();
    vRight.sort();

    let mut sum = 0;
    for i in 0..vLeft.len() {
        sum += (vLeft[i] - vRight[i]).abs();
    }
    println!("{}", sum);
}

pub fn d1_2(input: String) {
    let mut vLeft: Vec<i32> = vec![];
    let mut vRight: Vec<i32> = vec![];
    for line in input.lines() {
        let nums: Vec<i32> = line
            .split_whitespace()
            .map(|n| n.parse::<i32>().unwrap())
            .collect();
        vLeft.push(nums[0]);
        vRight.push(nums[1]);
    }    
    vLeft.sort();
    vRight.sort();

    let mut sum = 0;

    let mut map: HashMap<i32, i32> = HashMap::new();

    for num in vRight {
        if let Some(val) = map.get_mut(&num) {
            *val += 1;
        } else {
            map.insert(num, 1);
        }
    }
    for num in vLeft {
        match map.get(&num) {
            Some(val) => sum += val * num,
            None => continue
        }
    }
    println!("{}", sum);
}