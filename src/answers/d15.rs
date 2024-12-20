use std::collections::{HashMap, HashSet};

pub fn d15_1(input: String) {
    let v: Vec<&str> = input.split("\r\n\r\n").collect::<Vec<&str>>();
    let mut  map: Vec<Vec<char>> = v[0].split("\r\n")
        .map(|row| row.chars().collect())
        .collect();
    let (r, c) = (map.len(), map[0].len());
    let (mut y, mut x) = (0, 0);
    println!("{} {}", r, c);
    for i in 0..r {
        for j in 0..c {
            print!("{}",map[i][j]);
        }
        println!();
    }
    for i in 0..r {
        for j in 0..c {
            if map[i][j] == '@' {
                y = i;
                x = j;
            }
        }
    }
    for chr in v[1].chars() {
        if chr == '\n' {
            continue;
        }
        if chr == '^' {
            if map[y-1][x] == '.' {
                map[y][x] = '.';
                y -= 1;
                continue;
            } else if map[y-1][x] == '#' {
                continue;
            }
            let mut count = 0;
            while map[y - 1 - count][x] == 'O' {
                count += 1;
            }
            if map[y - 1 - count][x] == '#' {
                continue;
            } else {
                map[y][x] = '.';
                while count > 0 {
                    map[y - 1 - count][x] = 'O';
                    count -= 1;
                }
                map[y - 1][x] = '.';
                y -= 1;
            }
        } else if chr == 'v' {
            if map[y+1][x] == '.' {
                map[y][x] = '.';
                y += 1;
                continue;
            } else if map[y+1][x] == '#' {
                continue;
            }
            let mut count = 0;
            while map[y + 1 + count][x] == 'O' {
                count += 1;
            }
            if map[y + 1 + count][x] == '#' {
                continue;
            } else {
                map[y][x] = '.';
                while count > 0 {
                    map[y + 1 + count][x] = 'O';
                    count -= 1;
                }
                map[y + 1][x] = '.';
                y += 1;
            }
        } else if chr == '<' {
            if map[y][x-1] == '.' {
                map[y][x] = '.';
                x -= 1;
                continue;
            } else if map[y][x-1] == '#' {
                continue;
            }
            let mut count = 0;
            while map[y][x - 1 - count] == 'O' {
                count += 1;
            }
            if map[y][x - 1 - count] == '#' {
                continue;
            } else {
                map[y][x] = '.';
                while count > 0 {
                    map[y][x - 1 - count] = 'O';
                    count -= 1;
                }
                map[y][x-1] = '.';
                x -= 1;
            }
        } else if chr == '>' {
            if map[y][x+1] == '.' {
                map[y][x] = '.';
                x += 1;
                continue;
            } else if map[y][x+1] == '#' {
                continue;
            }
            let mut count = 0;
            while map[y][x + 1 + count] == 'O' {
                count += 1;
            }
            if map[y][x + 1 + count] == '#' {
                continue;
            } else {
                map[y][x] = '.';
                while count > 0 {
                    map[y][x + 1 + count] = 'O';
                    count -= 1;
                }
                map[y][x+1] = '.';
                x += 1;
            }
        }
    }
    // println!("{} {} {}", y, x); 
    let mut sum = 0;
    for i in 0..r {
        for j in 0..c {
            if map[i][j] == 'O' {
                sum += 100 * i + j;
            }
            print!("{}",map[i][j]);
        }
        println!();
    }
    println!("{}", sum);
}

pub fn d15_2(input: String) {
    let v: Vec<&str> = input.split("\r\n\r\n").collect::<Vec<&str>>();
    let mut map: Vec<Vec<char>> = v[0].split("\r\n")
        .map(|row| row.chars().collect())
        .collect();
    let (r, c) = (map.len(), map[0].len());
    let mut map2: Vec<Vec<char>> = vec![vec!['.'; 2*c]; r];
    let (mut y, mut x) = (0, 0);
    for i in 0..r {
        for j in 0..c {
            if map[i][j] == '@' {
                map2[i][j*2] = '@';
                map2[i][j*2+1] = '.';
                y = i;
                x = j*2+1;
            } else if map[i][j] == '.' {
                map2[i][j*2] = '.';
                map2[i][j*2+1] = '.';
            } else if map[i][j] == '#' {
                map2[i][j*2] = '#';
                map2[i][j*2+1] = '#';
            } else if map[i][j] == 'O' {
                map2[i][j*2] = '[';
                map2[i][j*2+1] = ']';
            }
        }
    }
    let (r2, c2) = (map2.len(), map2[0].len());
    for i in 0..r2 {
        for j in 0..c2 {
            print!("{}", map2[i][j]);
        }
        println!();
    }

    for chr in v[1].chars() {
        if chr == '\n' {
            continue;
        }
        if chr == '^' {
            if map2[y-1][x] == '.' {
                map2[y][x] = '.';
                y -= 1;
                continue;
            } else if map2[y-1][x] == '#' {
                continue;
            }
            let mut hmap: HashSet<(usize, usize)> = HashSet::new();
            let mut count = 0;
            let mut boxes: Vec<(usize, usize)> = Vec::new();
            if map2[y-1][x] == '[' {
                boxes.push((y-1, x));
            } else if map[y-1][x] == ']' {
                boxes.push((y-1, x+1));
            }
            loop {
                //get boxes
                let mut tmp: HashSet<(usize, usize)> = HashSet::new();
                let mut can_move = true;
                while boxes.len() > 0 {
                    let (by, bx) = boxes.pop().unwrap();
                    if map2[by-1][bx] == '#' || map2[by-1][bx+1] == '#' {
                        can_move = false;
                        break;
                    }
                    if map2[by-1][bx] == ']' {
                        tmp.insert((by-1, bx-1));
                    } 
                    if map2[by-1][bx] == '[' {
                        tmp.insert((by-1, bx));
                    }
                    if map2[by-1][bx+1] == '[' {
                        tmp.insert((by-1, bx+1));
                    }
                }
            }
            while map[y - 1 - count][x] == 'O' {
                count += 1;
            }
            if map[y - 1 - count][x] == '#' {
                continue;
            } else {
                map[y][x] = '.';
                while count > 0 {
                    map[y - 1 - count][x] = 'O';
                    count -= 1;
                }
                map[y - 1][x] = '.';
                y -= 1;
            }
        } else if chr == 'v' {
            if map[y+1][x] == '.' {
                map[y][x] = '.';
                y += 1;
                continue;
            } else if map[y+1][x] == '#' {
                continue;
            }
            let mut count = 0;
            while map[y + 1 + count][x] == 'O' {
                count += 1;
            }
            if map[y + 1 + count][x] == '#' {
                continue;
            } else {
                map[y][x] = '.';
                while count > 0 {
                    map[y + 1 + count][x] = 'O';
                    count -= 1;
                }
                map[y + 1][x] = '.';
                y += 1;
            }
        } else if chr == '<' {
            if map[y][x-1] == '.' {
                map[y][x] = '.';
                x -= 1;
                continue;
            } else if map[y][x-1] == '#' {
                continue;
            }
            let mut count = 0;
            while map[y][x - 1 - 2*count] == ']' {
                count += 1;
            }
            if map[y][x - 1 - 2*count] == '#' {
                continue;
            } else {
                map[y][x] = '.';
                while count > 0 {
                    map[y][x - 1 - 2*count] = ']';
                    map[y][x - 2 - 2*count] = '[';
                    count -= 1;
                }
                map[y][x-1] = '.';
                x -= 1;
            }
        } else if chr == '>' {
            if map[y][x+1] == '.' {
                map[y][x] = '.';
                x += 1;
                continue;
            } else if map[y][x+1] == '#' {
                continue;
            }
            let mut count = 0;
            while map[y][x + 1 + 2*count] == '[' {
                count += 1;
            }
            if map[y][x + 1 + 2*count] == '#' {
                continue;
            } else {
                map[y][x] = '.';
                while count > 0 {
                    map[y][x + 1 + 2*count] = '[';
                    map[y][x + 2 + 2*count] = ']';
                    count -= 1;
                }
                map[y][x+1] = '.';
                x += 1;
            }
        }
    }
}