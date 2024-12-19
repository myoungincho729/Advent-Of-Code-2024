use std::collections::HashSet;
static D: [(i32, i32); 4] = [(-1,0), (1,0), (0,1), (0,-1)];
pub fn d10_1(input: String) {
    let v: Vec<Vec<u8>> = input
        .lines()
        .map(|line| 
            line
                .chars()
                .map(|c| c.to_digit(10).unwrap() as u8)
                .collect()
        ).collect();
    let r = v.len();
    let c = v[0].len();
    let mut sum = 0;
    let mut vis: HashSet<(usize, usize)> = HashSet::new();

    for i in 0..r {
        for j in 0..c {
            if v[i][j] == 0 {
                // println!("@{} {}", i, j);
                dfs(i, j, &v, &mut vis, &mut sum);
                // println!("@@{sum}");            
                vis.clear();
            }
        }
    }
    println!("{sum}");

}

fn dfs(i: usize, j: usize, v: &Vec<Vec<u8>>, vis: &mut HashSet<(usize, usize)>, sum: &mut i32) {
    // println!("{} {} {}", i, j, v[i][j]);
    if v[i][j] == 9 {
        *sum += 1;
        vis.insert((i, j));
        return;
    }
    for (dy, dx) in D {
        let ny = i as i32 + dy;
        let nx = j as i32 + dx;
        if ny < 0 || ny >= v.len() as i32 || nx < 0 || nx >= v[0].len() as i32 {
            continue;
        }
        if v[ny as usize][nx as usize] != v[i][j] + 1 {
            continue;
        }
        if vis.contains(&(ny as usize, nx as usize)) {
            continue;
        }
        // vis.insert((ny as usize, nx as usize));
        dfs(ny as usize, nx as usize, v, vis, sum);
    }
}
pub fn d10_2(input: String) {
    
}