use regex::Regex;

pub fn d3_1(input: String) {
    let mut sum = 0;
    let re = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();
    for capture in re.captures_iter(&input) {
        let left = &capture[1].parse::<i32>().unwrap();
        let right = &capture[2].parse::<i32>().unwrap();
        sum += left * right;
    }
    println!("{}", sum);
}

pub fn d3_2(input: String) {
    let mut sum = 0;
    let re = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)|do\(\)|don't\(\)").unwrap();

    let mut addable = true;
    for capture in re.captures_iter(&input) {
        let inst = capture.get(0).unwrap().as_str();
        match inst {
            "do()" => addable = true,
            "don't()" => addable = false,
            _ => {
                if addable == true {
                    let left: i32 = capture.get(1).unwrap().as_str().parse().unwrap();
                    let right: i32 = capture.get(2).unwrap().as_str().parse().unwrap();
                    sum += left * right;
                }
            }
        }
    }
    println!("{}", sum);
}