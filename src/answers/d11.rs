use std::collections::HashMap;

use num_bigint::BigInt;

pub fn d11_1(input: String) {
    let mut v: Vec<BigInt> = Vec::new();
    for num_str in input.split_whitespace().collect::<Vec<&str>>() {
        v.push(BigInt::parse_bytes(num_str.as_bytes(), 10).unwrap());
    }
    let mut mx = BigInt::from(0);
    for _ in 0..25 {
        let mut tmp: Vec<BigInt> = Vec::new();
        for num in v.clone() {
            let transformed: Vec<BigInt> = transform(num);
            for trans_num in transformed {
                tmp.push(trans_num);
            }
        }
        v.clear();
        // for num in &tmp {
        //     print!("{} ", num);
        // }
        // println!();
        for num in tmp {
            if mx < num.clone() {
                mx = num.clone();
            }
            v.push(num);
        }
        // break;
    }
    println!("{} {}", v.len(), mx);

}

fn transform(num: BigInt) -> Vec<BigInt> {
    let mut ret = Vec::new();
    if num == BigInt::from(0) {
        ret.push(BigInt::from(1));
    } else if num.to_string().len() % 2 == 0 {
        let s = num.to_string();
        let (num1, num2) = (&s[0..s.len()/2], &s[s.len()/2..]);
        ret.push(BigInt::parse_bytes(num1.as_bytes(), 10).unwrap());
        ret.push(BigInt::parse_bytes(num2.as_bytes(), 10).unwrap());
    } else {
        ret.push(num * BigInt::from(2024));
    }
    ret
}

pub fn d11_2(input: String) {
    let mut v: Vec<BigInt> = Vec::new();
    let mut dp: HashMap<(BigInt, BigInt), BigInt> = HashMap::new();
    for num_str in input.split_whitespace().collect::<Vec<&str>>() {
        v.push(BigInt::parse_bytes(num_str.as_bytes(), 10).unwrap());
    }
    let mut sum: BigInt = BigInt::from(0);
    for num in v {
        sum += calc(num, BigInt::from(75), &mut dp);
    }
    println!("{}", sum);
}

fn calc(num: BigInt, n: BigInt, dp: &mut HashMap<(BigInt, BigInt), BigInt>) -> BigInt {
    let num_clone = num.clone();
    let n_clone = n.clone();
    if n == BigInt::ZERO {
        return BigInt::from(1);
    }
    if dp.contains_key(&(num.clone(), n.clone())) == true {
        return dp.get(&(num.clone(), n.clone())).unwrap().clone();
    }
    let ret: BigInt;
    if num == BigInt::ZERO {
        ret = calc(BigInt::from(1), n.clone()-1, dp);
    } else if num.to_string().len() % 2 == 0 {
        let s = num.to_string();
        let (num1, num2) = (&s[0..s.len()/2], &s[s.len()/2..]);
        ret = calc(BigInt::parse_bytes(num1.as_bytes(), 10).unwrap(), n.clone()-1, dp)
             + calc(BigInt::parse_bytes(num2.as_bytes(), 10).unwrap(), n.clone()-1, dp);
    } else {
        ret = calc(num * BigInt::from(2024), n.clone()-1, dp);
    }
    dp.insert((num_clone.clone(), n_clone), ret.clone());
    ret
}