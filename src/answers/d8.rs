use std::collections::{HashMap, HashSet};

pub fn d8_1(input: String) {
    let map: Vec<Vec<char>> = input.lines()
        .map(|line| line.chars().collect())
        .collect();
    let r = map.len();
    let c = map[0].len();
    let mut positions_map: HashMap<char, Vec<(usize, usize)>> = HashMap::new();
    let mut positions_set: HashSet<(i32, i32)> = HashSet::new();

    for i in 0..r {
        for j in 0..c {
            if map[i][j] == '.' {
                continue;
            }
            if let Some(v) = positions_map.get_mut(&map[i][j]) {
                v.push((i, j));
            } else {
                positions_map.insert(map[i][j], vec![(i, j)]);
            }
        }
    }

    for key in positions_map.keys() {
        let v = positions_map.get(key).unwrap();
        for a in 0..v.len()-1 {
            for b in a+1..v.len() {
                let (ay, ax) = v[a];
                let (by, bx) = v[b];
                let dy = by as i32 - ay as i32;
                let dx = bx as i32 - ax as i32;
                let (sy, sx) = (by as i32 + dy, bx as i32 + dx);
                let (ey, ex) = (ay as i32 - dy, ax as i32 - dx);
                if !(sy < 0 || sy >= r as i32 || sx < 0 || sx >= c as i32) {
                    positions_set.insert((sy, sx));
                } 
                if !(ey < 0 || ey >= r as i32 || ex < 0 || ex >= c as i32) {
                    positions_set.insert((ey, ex));
                } 
            }
        }
    }
    println!("{}", positions_set.len());
}

pub fn d8_2(input: String) {
    let map: Vec<Vec<char>> = input.lines()
        .map(|line| line.chars().collect())
        .collect();
    let r = map.len();
    let c = map[0].len();
    let mut positions_map: HashMap<char, Vec<(usize, usize)>> = HashMap::new();
    let mut positions_set: HashSet<(i32, i32)> = HashSet::new();

    for i in 0..r {
        for j in 0..c {
            if map[i][j] == '.' {
                continue;
            }
            if let Some(v) = positions_map.get_mut(&map[i][j]) {
                v.push((i, j));
            } else {
                positions_map.insert(map[i][j], vec![(i, j)]);
            }
        }
    }

    for key in positions_map.keys() {
        let v = positions_map.get(key).unwrap();
        for a in 0..v.len()-1 {
            for b in a+1..v.len() {
                let (ay, ax) = v[a];
                let (by, bx) = v[b];
                let dy = by as i32 - ay as i32;
                let dx = bx as i32 - ax as i32;
                let mut scale = 0;
                loop {
                    let (sy, sx) = (by as i32 + dy * scale, bx as i32 + dx * scale);
                    if !(sy < 0 || sy >= r as i32 || sx < 0 || sx >= c as i32) {
                        positions_set.insert((sy, sx));
                        scale += 1;
                    } else {
                        break;
                    }
                }
                scale = 0;
                loop {
                    let (ey, ex) = (ay as i32 - dy * scale, ax as i32 - dx * scale);
                    if !(ey < 0 || ey >= r as i32 || ex < 0 || ex >= c as i32) {
                        positions_set.insert((ey, ex));
                        scale += 1;
                    } else {
                        break;
                    }
                }
            }
        }
    }
    println!("{}", positions_set.len());
}