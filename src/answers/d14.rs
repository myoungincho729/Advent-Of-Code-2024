use std::{fs::{self, OpenOptions}, io::Write, thread::sleep, time::Duration, vec};

use regex::Regex;

pub fn d14_1(input: String) {
    let mut v: Vec<Robot> = Vec::new();
    let re = Regex::new(r"p=(-*\d*),(-*\d*) v=(-*\d*),(-*\d*)").unwrap();
    for capture in re.captures_iter(input.as_str()) {
        println!("{} {} {} {} {}", &capture[0], &capture[1], &capture[2], &capture[3], &capture[4]);
        let px = capture[1].parse::<i32>().unwrap();
        let py = capture[2].parse::<i32>().unwrap();
        let vx = capture[3].parse::<i32>().unwrap();
        let vy = capture[4].parse::<i32>().unwrap();
        let (rx, ry) = (101, 103);
        v.push(Robot::new(px, py, vx, vy, rx, ry));
    }
    let mut q = vec![0,0,0,0,0];
    for robot in v.iter_mut() {
        println!("{:?}", robot.get_curr());
        robot.move100();
        println!("{:?}", robot.get_curr());
        q[robot.what_quadrant()] += 1;
    }
    let ret = q[1] * q[2] * q[3] * q[4];
    println!("{}", ret);

}

struct Robot {
    curr_position: (i32, i32),
    velocity: (i32, i32),
    room_size: (i32, i32)
}

impl Robot {
    pub fn new(px: i32, py: i32, vx: i32, vy: i32, rx: i32, ry: i32) -> Robot {
        let mut real_vx = vx;
        let mut real_vy = vy;
        if vx < 0 {
            real_vx = vx + rx;
        }
        if vy < 0 {
            real_vy = vy + ry;
        }

        Robot {
            curr_position: (px, py),
            velocity: (real_vx, real_vy),
            room_size: (rx, ry)
        }
    }

    pub fn move1(&mut self) {
        let pos_x = (self.curr_position.0 + self.velocity.0) % self.room_size.0;
        let pos_y = (self.curr_position.1 + self.velocity.1) % self.room_size.1;
        self.curr_position = (pos_x, pos_y);
    }

    pub fn move100(&mut self) {
        let pos_x = (self.curr_position.0 + 100 * self.velocity.0) % self.room_size.0;
        let pos_y = (self.curr_position.1 + 100 * self.velocity.1) % self.room_size.1;
        self.curr_position = (pos_x, pos_y);
    }

    pub fn get_curr(&self) -> (i32, i32) {
        self.curr_position
    }

    pub fn what_quadrant(&self) -> usize {
        if self.curr_position.0 < self.room_size.0 / 2 
            && self.curr_position.1 < self.room_size.1 / 2 {
                1
        } else if self.curr_position.0 < self.room_size.0 / 2 
            && self.curr_position.1 > self.room_size.1 / 2 {
                2
        } else if self.curr_position.0 > self.room_size.0 / 2 
            && self.curr_position.1 < self.room_size.1 / 2 {
                3
        } else if self.curr_position.0 > self.room_size.0 / 2 
            && self.curr_position.1 > self.room_size.1 / 2 {
                4
        } else {
                0
        }
    }
}

pub fn d14_2(input: String) {
    let mut file = OpenOptions::new()
        .append(true)
        .create(true) // Create the file if it doesn't exist
        .open("example.txt")
        .unwrap();
    let mut v: Vec<Robot> = Vec::new();
    let re = Regex::new(r"p=(-*\d*),(-*\d*) v=(-*\d*),(-*\d*)").unwrap();
    for capture in re.captures_iter(input.as_str()) {
        println!("{} {} {} {} {}", &capture[0], &capture[1], &capture[2], &capture[3], &capture[4]);
        let px = capture[1].parse::<i32>().unwrap();
        let py = capture[2].parse::<i32>().unwrap();
        let vx = capture[3].parse::<i32>().unwrap();
        let vy = capture[4].parse::<i32>().unwrap();
        let (rx, ry) = (101, 103);
        v.push(Robot::new(px, py, vx, vy, rx, ry));
    }
    let mut q = vec![0,0,0,0,0];
    for m in 0..10000 {
        if m % 100 == 0 {
            println!("{}", m);
        }
        let mut map: Vec<Vec<char>> = vec![vec!['.'; 101]; 103];
        // map.fill(0);
        for robot in v.iter_mut() {
            // println!("{:?}", robot.get_curr());
            robot.move1();
            // println!("{:?}", robot.get_curr());
            // q[robot.what_quadrant()] += 1;
            let (x, y) = robot.get_curr();
            map[y as usize][x as usize] = '@';
        }
        for i in 0..103 {
            for j in 0..101 {
                file.write(map[i][j].to_string().as_bytes()).unwrap();
                // print!("{}", map[i][j]);
            }
            file.write("\n".as_bytes()).unwrap();
            // println!();
        }
        // println!();
        file.write(m.to_string().as_bytes()).unwrap();
        file.write("\n".as_bytes()).unwrap();
        // sleep(Duration::from_millis(10));
    }
    // let ret = q[1] * q[2] * q[3] * q[4];
    // println!("{}", ret);
}