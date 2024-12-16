pub fn d2_1(input: String) {
    let mut sum = 0;
    for line in input.lines() {
        let v: Vec<i32> = line
            .split_whitespace()
            .map(|n| n.parse::<i32>().unwrap())
            .collect::<Vec<i32>>();

        let mut flag = true;
        for i in 2..v.len() {
            let diff1 = v[i-1] - v[i-2];
            let diff2 = v[i] - v[i-1];
            if diff1 * diff2 <= 0 {
                flag = false;
                break;
            }
            if diff1.abs() > 3 || diff2.abs() > 3 {
                flag = false;
                break;
            }
        }
        sum += flag as i32;
    }
    println!("{}", sum);
}

pub fn d2_2(input: String) {
    let mut sum = 0;
    for line in input.lines() {
        let mut v: Vec<i32> = line
            .split_whitespace()
            .map(|n| n.parse::<i32>().unwrap())
            .collect::<Vec<i32>>();

        let mut flag_total = true;
        for j in 0..v.len() {
            let removed_index = j;
            let removed = v.remove(removed_index);
            let mut flag = true;

            for i in 2..v.len() {
                let diff1 = v[i-1] - v[i-2];
                let diff2 = v[i] - v[i-1];
                if diff1 * diff2 <= 0 {
                    flag = false;
                    break;
                }
                if diff1.abs() > 3 || diff2.abs() > 3 {
                    flag = false;
                    break;
                }
            }
            if flag == true {
                sum += 1;
                break;
            } else {
                v.insert(j, removed);
            }
        }
    }
    println!("{}", sum);
}