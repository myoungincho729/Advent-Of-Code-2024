use std::collections::HashSet;

static D: [(i32, i32); 4] = [(-1,0), (1,0), (0,1), (0,-1)];
static D2: [(i32, i32); 5] = [(-1,0), (0,1), (1,0), (0,-1), (-1,0)];
pub fn d12_1(input: String) {
    let v: Vec<Vec<char>> = input.lines()
        .map(|line| line.chars().collect())
        .collect();
    let (r, c) = (v.len(), v[0].len());
    let mut vis: HashSet<(usize, usize)> = HashSet::new();
    let mut sum = 0;
    for i in 0..r {
        for j in 0..c {
            if vis.contains(&(i, j)) {
                continue;
            }
            let (area, peri) = dfs(i, j, &mut vis, &v);
            // println!("{} {} {}", v[i][j], area, peri);
            sum += area * peri;
        }
    }
    println!("{}", sum);
}

fn dfs(i: usize, j: usize, vis: &mut HashSet<(usize, usize)>,  v: &Vec<Vec<char>>) -> (i32, i32) {
    vis.insert((i.clone(), j.clone()));
    let mut area = 1;
    let mut peri = 0;

    for (dy, dx) in D {
        let ny = i as i32 + dy;
        let nx = j as i32 + dx;
        if ny < 0 || ny >= v.len() as i32 || nx < 0 || nx >= v[0].len()  as i32 {
            continue;
        }
        if vis.contains(&(ny as usize, nx as usize)) {
            continue;
        }
        if v[ny as usize][nx as usize] != v[i][j] {
            continue;
        }
        let (inner_area, inner_peri) = dfs(ny as usize, nx as usize, vis, v);
        area += inner_area;
        peri += inner_peri;
    }
    for (dy, dx) in D {
        let ny = i as i32 + dy;
        let nx = j as i32 + dx;
        if ny < 0 || ny >= v.len() as i32 || nx < 0 || nx >= v[0].len()  as i32 {
            peri += 1;
        } else if v[ny as usize][nx as usize] != v[i][j] {
            peri += 1;
        }
    }
    (area, peri)
}

pub fn d12_2(input: String) {
    let v: Vec<Vec<char>> = input.lines()
        .map(|line| line.chars().collect())
        .collect();
    let (r, c) = (v.len(), v[0].len());
    let mut vis: HashSet<(usize, usize)> = HashSet::new();
    let mut sum = 0;
    for i in 0..r {
        for j in 0..c {
            if vis.contains(&(i, j)) {
                continue;
            }
            let (area, side) = dfs2(i, j, &mut vis, &v);
            // println!("{} {} {}", v[i][j], area, side);
            sum += area * side;
        }
    }
    println!("{}", sum);

}

fn dfs2(i: usize, j: usize, vis: &mut HashSet<(usize, usize)>,  v: &Vec<Vec<char>>) -> (i32, i32) {
    // println!("{}-{}", i,j);
    vis.insert((i.clone(), j.clone()));
    let mut area = 1;
    let mut side = 0;

    for d in 0..4 {
        let (dy1, dx1) = D2[d];
        let ny = i as i32 + dy1;
        let nx = j as i32 + dx1;

        
        if ny < 0 || ny >= v.len() as i32 || nx < 0 || nx >= v[0].len()  as i32 {
            continue;
        }
        if vis.contains(&(ny as usize, nx as usize)) {
            continue;
        }
        if v[ny as usize][nx as usize] != v[i][j] {
            continue;
        }
        let (inner_area, inner_side) = dfs2(ny as usize, nx as usize, vis, v);
        area += inner_area;
        side += inner_side;
    }
    for d in 0..4 {
        let (dy1, dx1) = D2[d];
        let (dy2, dx2) = D2[d+1];
        let (dy3, dx3) = (dy1+dy2, dx1+dx2);
        let ny = i as i32 + dy1;
        let nx = j as i32 + dx1;
        let ny2 = i as i32 + dy2;
        let nx2 = j as i32 + dx2;
        let ny3 = i as i32 + dy3;
        let nx3 = j as i32 + dx3;
        let mut p1_exist = true;
        let mut p2_exist = true;
        let mut p3_exist = true;
        if (ny < 0 || ny >= v.len() as i32 || nx < 0 || nx >= v[0].len() as i32)
        || (v[ny as usize][nx as usize] != v[i][j]) {
            p1_exist = false;
        }
        if (ny2 < 0 || ny2 >= v.len() as i32 || nx2 < 0 || nx2 >= v[0].len() as i32)
        || (v[ny2 as usize][nx2 as usize] != v[i][j]) {
            p2_exist = false;
        }
        if (ny3 < 0 || ny3 >= v.len() as i32 || nx3 < 0 || nx3 >= v[0].len() as i32)
        || (v[ny3 as usize][nx3 as usize] != v[i][j]) {
            p3_exist = false;
        }
        if !p1_exist && !p2_exist {
            side += 1;
        }
        if p1_exist && p2_exist && !p3_exist {
            side += 1;
        }
    }
    // println!("{}-{} {} {}", i,j,area, side);
    (area, side)
}