use std::{env, fs};

pub mod answers;
use answers::*;
fn main() {
    let args = env::args()
        .skip(1)
        .collect::<Vec<String>>();
    let filename: String;
    if args.len() == 2 {
        filename = format!("d{}t.txt", args[0]);
    } else if args.len() == 3 {
        filename = format!("d{}tt.txt", args[0]);
    } else {
        filename = format!("d{}.txt", args[0]);
    }
    let path = format!("inputs/{}", filename);
    let input = fs::read_to_string(path)
        .expect("no file");


    // d2::d2_1(input);
    // d2::d2_2(input);
    // d3::d3_1(input);
    // d3::d3_2(input);
    d4::d4_1(input);
    // d4::d4_2(input);
    // d5_1(input);
    // d5_2(input);
}
