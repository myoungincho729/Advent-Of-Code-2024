use std::collections::HashMap;

pub fn d1_1(input: String) {
    let mut v_left: Vec<i32> = vec![];
    let mut v_right: Vec<i32> = vec![];
    for line in input.lines() {
        let nums: Vec<i32> = line
            .split_whitespace()
            .map(|n| n.parse::<i32>().unwrap())
            .collect();
        v_left.push(nums[0]);
        v_right.push(nums[1]);
    }    
    v_left.sort();
    v_right.sort();

    let mut sum = 0;
    for i in 0..v_left.len() {
        sum += (v_left[i] - v_right[i]).abs();
    }
    println!("{}", sum);
}

pub fn d1_2(input: String) {
    let mut v_left: Vec<i32> = vec![];
    let mut v_right: Vec<i32> = vec![];
    for line in input.lines() {
        let nums: Vec<i32> = line
            .split_whitespace()
            .map(|n| n.parse::<i32>().unwrap())
            .collect();
        v_left.push(nums[0]);
        v_right.push(nums[1]);
    }    
    v_left.sort();
    v_right.sort();

    let mut sum = 0;

    let mut map: HashMap<i32, i32> = HashMap::new();

    for num in v_right {
        if let Some(val) = map.get_mut(&num) {
            *val += 1;
        } else {
            map.insert(num, 1);
        }
    }
    for num in v_left {
        match map.get(&num) {
            Some(val) => sum += val * num,
            None => continue
        }
    }
    println!("{}", sum);
}