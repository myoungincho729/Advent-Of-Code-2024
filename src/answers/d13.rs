pub fn d13_1(input: String) {
    let mut sum = 0;
    input.split("\r\n\r\n")
        .for_each(|chunk| {
            let lines: Vec<&str> = chunk.split('\n')
                .map(|line| line.trim())
                .collect();
            // println!("{:?}", lines);
        
            let (pax, pay) = (lines[0].find("X+").unwrap(), lines[0].find("Y+").unwrap());
            let (pbx, pby) = (lines[1].find("X+").unwrap(), lines[1].find("Y+").unwrap());
            let (psx, psy) = (lines[2].find("X=").unwrap(), lines[2].find("Y=").unwrap());  
            let (ax, ay) = (lines[0][pax+2..pax+4].parse::<i32>().unwrap(), lines[0][pay+2..pay+4].parse::<i32>().unwrap());
            let (bx, by) = (lines[1][pbx+2..pbx+4].parse::<i32>().unwrap(), lines[1][pby+2..pby+4].parse::<i32>().unwrap());
            let (sx, sy) = (
                lines[2][psx+2..lines[2].find(", ").unwrap()].parse::<i32>().unwrap(),
                lines[2][psy+2..lines[2].len()].parse::<i32>().unwrap()
            );
            // println!("{} {} {} {} {} {}", ax,ay,bx,by,sx,sy);

            let (a_numerator, a_denominator) = ((bx*sy-by*sx).abs(), (ay*bx-ax*by).abs());
            let (b_numerator, b_denominator) = ((ax*sy-ay*sx).abs(), (ax*by-ay*bx).abs());
            if a_numerator % a_denominator == 0 && b_numerator % a_denominator == 0 {
                let a = a_numerator / a_denominator;
                let b = b_numerator / b_denominator;
                if a <= 100 && b <= 100 {
                    sum += 3*a + b;
                }
            }
        });
    println!("{}", sum);
}

pub fn d13_2(input: String) {
    let mut sum = 0;
    input.split("\r\n\r\n")
        .for_each(|chunk| {
            let lines: Vec<&str> = chunk.split('\n')
                .map(|line| line.trim())
                .collect();
            // println!("{:?}", lines);
        
            let (pax, pay) = (lines[0].find("X+").unwrap(), lines[0].find("Y+").unwrap());
            let (pbx, pby) = (lines[1].find("X+").unwrap(), lines[1].find("Y+").unwrap());
            let (psx, psy) = (lines[2].find("X=").unwrap(), lines[2].find("Y=").unwrap());  
            let (ax, ay) = (lines[0][pax+2..pax+4].parse::<i64>().unwrap(), lines[0][pay+2..pay+4].parse::<i64>().unwrap());
            let (bx, by) = (lines[1][pbx+2..pbx+4].parse::<i64>().unwrap(), lines[1][pby+2..pby+4].parse::<i64>().unwrap());
            let (sx, sy) = (
                lines[2][psx+2..lines[2].find(", ").unwrap()].parse::<i64>().unwrap() + 10000000000000i64,
                lines[2][psy+2..lines[2].len()].parse::<i64>().unwrap() + 10000000000000i64
            );
            // println!("{} {} {} {} {} {}", ax,ay,bx,by,sx,sy);

            let (a_numerator, a_denominator) = ((bx*sy-by*sx).abs(), (ay*bx-ax*by).abs());
            let (b_numerator, b_denominator) = ((ax*sy-ay*sx).abs(), (ax*by-ay*bx).abs());
            if a_numerator % a_denominator == 0 && b_numerator % a_denominator == 0 {
                // println!("{} {} {} {}", ax,ay,bx,by);
                let a = a_numerator / a_denominator;
                let b = b_numerator / b_denominator;
                sum += 3*a + b;
            }
        });
    println!("{}", sum);
}