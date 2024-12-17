pub fn d4_1(input: String) {
    let arr: Vec<Vec<char>> = input.lines().map(|s| s.chars().collect()).collect();
    let r = arr.len();
    let c = arr[0].len();
    let dy: Vec<i32> = vec![-1,-1,-1,0,0,1,1,1];
    let dx: Vec<i32> = vec![-1,0,1,-1,1,-1,0,1];
    let xmas = ['X', 'M', 'A', 'S'];
    let mut count = 0;
    for i in 0..r {
        for j in 0..c {
            if arr[i][j] == 'X' {
                for k in 0..8 {
                    let ny: i32 = i as i32 + 3 * dy[k];
                    let nx: i32 = j as i32 + 3 * dx[k];
                    if ny < 0 || ny >= r as i32 || nx < 0 || nx >= c as i32 {
                        continue;
                    }
                    let mut ok = true;
                    for z in 1..4 {
                        let ny: i32 = i as i32 + z * dy[k];
                        let nx: i32 = j as i32 + z * dx[k];
                        if arr[ny as usize][nx as usize] != xmas[z as usize] {
                            ok = false;
                            break;
                        }
                    }
                    if ok == true {
                        count += 1;
                    }
                }
            }
        }
    }
    println!("{}", count);
}

pub fn d4_2(input: String) {
    let arr: Vec<Vec<char>> = input.lines().map(|s| s.chars().collect()).collect();
    let r = arr.len();
    let c = arr[0].len();
    let filter: Vec<Vec<char>> = ["MMSS", "MSMS", "SMSM", "SSMM"].iter()
        .map(|s| s.chars().collect::<Vec<char>>()).collect();
    
    let mut count = 0;
    for i in 0..r-2 {
        for j in 0..c-2 {
            for k in 0..filter.len() {
                if arr[i][j]==filter[k][0] 
                    && arr[i][j+2]==filter[k][1] 
                    && arr[i+2][j]==filter[k][2] 
                    && arr[i+2][j+2]==filter[k][3] 
                    && arr[i+1][j+1]=='A' {
                    count += 1;
                    // println!("{} {}", i, j);
                    break;
                }
            }
        }
    }
    println!("{}", count);
}