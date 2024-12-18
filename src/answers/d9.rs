
pub fn d9_1(input: String) {
    let mut v: Vec<Option<i32>> = Vec::new();
    let mut id = 0;
    let mut is_num = 1;

    for c in input.chars() {
        let n = c as i32 - '0' as i32;
        for _ in 0..n {
            if is_num == 1 {
                v.push(Some(id));
            } else {
                v.push(None);
            }
        }
        is_num = 1 - is_num;
        if is_num == 1 {
            id += 1;
        }
    }

    let mut start = 0;
    let mut end = v.len() - 1;
    loop {
        while v[end] == None {
            end -= 1;
        }
        while v[start] != None {
            start += 1;
        }
        if start >= end {
            break;
        }
        v[start] = v[end];
        v[end] = None;
    }

    let mut sum: i64 = 0;
    for (idx, val) in v.iter().enumerate() {
        if let Some(value) = val {
            sum += idx as i64 * *value as i64;
        }
    }
    println!("{}", sum);
}

pub fn d9_2(input: String) {
}