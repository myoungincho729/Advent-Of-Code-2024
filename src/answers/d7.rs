pub fn d7_1(input: String) {
    let mut sum: i64 = 0;
    for line in input.lines() {
        let splited = line.split(": ").collect::<Vec<&str>>();
        let val: i64 = splited[0].parse().unwrap();
        let v: Vec<i64> = splited[1].split_whitespace()
            .map(|n| n.parse().unwrap())
            .collect();
        if can_make(1, v[0], val, &v) {
            sum += val;
        }
    }
    println!("{}", sum);
}
fn can_make(idx: usize, cur: i64, target: i64, v: &Vec<i64>) -> bool {
    if cur == target && idx == v.len() {
        return true;
    } else if idx == v.len() {
        return false;
    }

    if can_make(idx+1, cur+v[idx], target, v) || can_make(idx+1, cur*v[idx], target, v) {
        return true;
    }

    false
}

pub fn d7_2(input: String) {
    let mut sum: i64 = 0;
    for line in input.lines() {
        let splited = line.split(": ").collect::<Vec<&str>>();
        let val: i64 = splited[0].parse().unwrap();
        let v: Vec<i64> = splited[1].split_whitespace()
            .map(|n| n.parse().unwrap())
            .collect();
        if can_make_2(1, v[0], val, &v) {
            sum += val;
        }
    }
    println!("{}", sum);
}
fn can_make_2(idx: usize, cur: i64, target: i64, v: &Vec<i64>) -> bool {
    if cur == target && idx == v.len() {
        return true;
    } else if idx == v.len() {
        return false;
    }
    let mut value: i64 = v[idx];
    let mut count: i64 = 1;
    while value >= 1 {
        value /= 10;
        count *= 10;
    }
    if can_make_2(idx+1, cur*count + v[idx], target, v) {
        return true;
    }
    if can_make_2(idx+1, cur+v[idx], target, v) || can_make_2(idx+1, cur*v[idx], target, v) {
        return true;
    }

    false
}