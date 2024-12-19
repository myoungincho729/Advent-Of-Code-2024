
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
    let mut v: Vec<(i32, i32)> = Vec::new();
    let mut is_num = 1;
    let mut id = 0;
    for c in input.chars() {
        let n = c as i32 - '0' as i32;
        if is_num == 1 {
            v.push((id, n));
            id += 1;
        } else {
            v.push((-1, n));
        }
        is_num = 1 - is_num;
    }
    // println!("{:?}", v);
    id -= 1;
    let mut end = v.len() - 1;
    loop {
        if id == 0 {
            break;
        }
        while end > 0 && v[end].0 != id {
            end -= 1;
        }
        let mut start = 0;
        while start < v.len() && (v[start].0 != -1 || v[start].1 < v[end].1) {
            start += 1;
        }
        // println!("{} {} {} {}", id, start, end, v.len());
        if start >= end {
            id -= 1;
            continue;
        }
        let left = v[start].1 - v[end].1;
        if left != 0 {
            v[start] = (-1, left);
            v.insert(start, v[end]);
            v[end+1] = (-1, v[end+1].1);
        } else {
            v[start] = v[end];
            v[end] = (-1, v[end].1);
        }
        id -= 1;
        if id == 0 {
            break;
        }
    }
    // println!("{:?}", v);
    let mut sum: i64 = 0;
    let mut idx = 0;
    for (id, num) in v {
        if id == -1 {
            idx += num;
            continue;
        }
        for i in 0..num {
            // println!("{} {}", idx+i, id);
            sum += (idx as i64 + i as i64) * id as i64; 
        }
        idx += num;
    }
    println!("{}", sum);
}