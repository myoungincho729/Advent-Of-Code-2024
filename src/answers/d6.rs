use std::collections::{HashMap, HashSet};

pub fn d6_1(input: String) { 
    let mut map: Vec<Vec<char>> = input
        .lines()
        .map(|line| line.chars().collect())
        .collect();
    let r = map.len();
    let c = map[0].len();
    let mut py = 0;
    let mut px = 0;
    let mut d = 0;
    let dir: [[i32; 2]; 4] = [[-1, 0], [0, 1], [1, 0], [0, -1]];
    for i in 0..r {
        for j in 0..c {
            if map[i][j]=='.' || map[i][j]=='#' {
                continue;
            }
            py = i;
            px = j;
            if map[i][j]=='^' {
                d = 0;
            } else if map[i][j] == '>' {
                d = 1;
            } else if map[i][j] == 'v' {
                d = 2;
            } else if map[i][j] == '<' {
                d = 3;
            }
        }
    }
    let mut visited: HashSet<(usize, usize)> = HashSet::new();
    loop {
        visited.insert((py, px));
        map[py][px] = 'X';
        let ny = py as i32 + dir[d][0];
        let nx = px as i32 + dir[d][1];
        if ny < 0 || ny >= r as i32 || nx < 0 || nx >= c as i32 {
            break;
        }
        if map[ny as usize][nx as usize] == '#' {
            d = (d + 1) % 4;
            continue;
        }
        py = ny as usize;
        px = nx as usize;
    }
    for i in 0..r {
        for j in 0..c {
            print!("{}", map[i][j]);
        }
        print!("\n");
    }
    println!("{}", visited.len());
}

#[allow(dead_code)]
pub fn d6_2(input: String) { 
    let mut map: Vec<Vec<char>> = input
        .lines()
        .map(|line| line.chars().collect())
        .collect();
    let r = map.len();
    let c = map[0].len();
    let mut py = 0;
    let mut px = 0;
    let mut d = 0;
    let dir: [[i32; 2]; 4] = [[-1, 0], [0, 1], [1, 0], [0, -1]];
    for i in 0..r {
        for j in 0..c {
            if map[i][j]=='.' || map[i][j]=='#' {
                continue;
            }
            py = i;
            px = j;
            if map[i][j]=='^' {
                d = 0;
            } else if map[i][j] == '>' {
                d = 1;
            } else if map[i][j] == 'v' {
                d = 2;
            } else if map[i][j] == '<' {
                d = 3;
            }
        }
    }
    let mut sum = 0;
    for i in 0..r {
        for j in 0..c {
            if map[i][j] != '.' {
                continue;
            }
            map[i][j] = '#';
            let mut y = py;
            let mut x = px;
            let mut dd = d;
            // simulation
            let mut visited: HashMap<(usize, usize), i32> = HashMap::new();
            let mut can_block = false;
            println!("start {} {} {} {} {:?}", i, j, y, x, visited);
            loop {
                let ny = y as i32 + dir[dd][0];
                let nx = x as i32 + dir[dd][1];
                if ny < 0 || ny >= r as i32 || nx < 0 || nx >= c as i32 {
                    // println!("cannot block");
                    break;
                }
                if map[ny as usize][nx as usize] == '#' {
                    if let Some(num) = visited.get_mut(&(y, x)) {
                        if *num > 2 {
                            can_block = true;
                            break;
                        } else {
                            *num += 1;
                        }
                    } else {
                        visited.insert((y, x), 1);
                    }
                    // if visited.contains(&(y, x)) {
                    //     can_block = true;
                    //     println!("can block {} {}", y, x);
                    //     break;
                    // }
                    // visited.insert((y, x));
                    dd = (dd + 1) % 4;
                    // println!("{} {} {:?}", y, x, visited);
                    continue;
                }
                y = ny as usize;
                x = nx as usize;
            }
            // println!("{} {} {:?}", i, j, visited);
            if can_block == true {
                sum += 1;
            }
            map[i][j] = '.';
        }
    }
    println!("{}", sum);

}