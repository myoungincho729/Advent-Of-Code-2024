use std::{env, fs};

pub mod answers;
use answers::*;

fn main() {
    let args = env::args()
        .skip(1)
        .collect::<Vec<String>>();

    let filename: String;
    let problem_name: Vec<&str> = args[0].split('-').collect::<Vec<&str>>();
    if args.len() == 1 {
        filename = format!("d{}.txt", problem_name[0]);
    } else {
        filename = format!("d{}{}.txt", problem_name[0], args[1]);
    }
    let path = format!("inputs/{}", filename);
    let input = fs::read_to_string(path)
        .expect("no file");

    match args[0].as_str() {
        "1-1" => d1::d1_1(input),
        "1-2" => d1::d1_2(input),
        "2-1" => d2::d2_1(input),
        "2-2" => d2::d2_2(input),
        "3-1" => d3::d3_1(input),
        "3-2" => d3::d3_2(input),
        "4-1" => d4::d4_1(input),
        "4-2" => d4::d4_2(input),
        "5-1" => d5::d5_1(input),
        "5-2" => d5::d5_2(input),
        "6-1" => d6::d6_1(input),
        "6-2" => d6::d6_2(input),
        "7-1" => d7::d7_1(input),
        "7-2" => d7::d7_2(input),
        "8-1" => d8::d8_1(input),
        "8-2" => d8::d8_2(input),
        "9-1" => d9::d9_1(input),
        "9-2" => d9::d9_2(input),
        "10-1" => d10::d10_1(input),
        "10-2" => d10::d10_2(input),
        "11-1" => d11::d11_1(input),
        "11-2" => d11::d11_2(input),
        _ => println!("Unknown command: {}", args[1]),
    }
}
