use std::{cmp::Ordering, collections::{HashMap, HashSet}};

pub fn d5_1(input: String) { 
    let inputs: Vec<&str> = input.split("\n\n").collect();
    let mut graph: HashMap<i32, Vec<i32>> = HashMap::new();
    inputs[0]
        .split_whitespace()
        .for_each(|line| {
            let nums:Vec<i32> = line.split('|').into_iter()
                .map(|n| n.parse::<i32>().unwrap())
                .collect();
            match graph.get_mut(&nums[0]) {
                Some(v) => v.push(nums[1]),
                None => {
                    let mut v = vec![nums[1]];
                    graph.insert(nums[0], v);
                }
            }
        });
    // println!("{:?}", graph);
    let mut sum = 0;
    for line in inputs[1].lines() {
        let nums = line
            .split(',')
            .into_iter()
            .map(|n| n.parse::<i32>().unwrap())
            .collect::<Vec<i32>>();

        let mut can_go = true;
        for i in 0..nums.len()-1 {
            let current = nums[i];
            let target = nums[i+1];
            match graph.get(&current) {
                Some(v) => {
                    if !v.contains(&target) {
                        can_go = false;
                        break;
                    }
                },
                None => {
                    can_go = false;
                    break;
                }
            }
        }
        if can_go == true {
            sum += nums[nums.len() / 2];
        }
    }
    println!("{}", sum);
}

#[allow(dead_code)]
pub fn d5_2(input: String) { 
    let (orderings, updates) = input.split_once("\n\n").unwrap();

    let orderings: HashSet<(usize, usize)> = orderings
        .lines()
        .map(|line| (line[0..2].parse().unwrap(), line[3..].parse().unwrap()))
        .collect();

    let ordering = |x: &usize, y: &usize| {
        let (x, y) = (*x, *y);
        if orderings.contains(&(x, y)) {
            return Ordering::Less;
        } else if orderings.contains(&(y, x)) {
            return Ordering::Greater;
        } else {
            return Ordering::Equal;
        }
    };


    let sum = updates
        .split('\n')
        .map(|line| {
            let mut nums: Vec<usize> = line
                .split(',')
                .map(|n| n.parse::<usize>().unwrap())
                .collect();
            // if nums.is_sorted_by(|l, r| ordering(l, r) != Ordering::Greater) {
            //     return 0;
            // } else {
            //     nums.sort_by(ordering);
            //     return nums[nums.len() / 2];
            // };
            let is_sorted = nums.windows(2).all(|w| ordering(&w[0], &w[1]) != std::cmp::Ordering::Greater);

            if is_sorted {
                return 0;
            } else {
                nums.sort_by(ordering);
                return nums[nums.len() / 2];
            }
        })
        .sum::<usize>();
    
    println!("{}", sum);
}
