use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

use regex::Regex;

fn handle_aoc_file(aoc_file_content: String) {
    let mut total = 0;

    for file_line in aoc_file_content.lines() {     
        let mut first_num = "";
        let mut last_num = "";

        for cap in Regex::new(r"\d|\s").unwrap().find_iter(file_line) {
            let cap_str = cap.as_str();

            if first_num == "" {
                first_num = cap_str;
                last_num = cap_str;
            } else {
                last_num = cap_str;
            }
        }

        let found_num:i32 = (first_num.to_owned() + last_num).parse().unwrap();
        total += found_num;

        println!("{}->{}", file_line, found_num);
    }

    println!("Total is {}", total);
}

fn main() {
    println!("Advent of Code - Day 01");

    let input_path = Path::new("input_aoc_d01.txt");

    let mut file = match File::open(&input_path) {
        Ok(file) => file,
        Err(_exc) => panic!("Can't open the input file"),
    };

    let mut file_content = String::new();
    match file.read_to_string(&mut file_content) {
        Ok(_) => handle_aoc_file(file_content),
        Err(_exc) => panic!("Can't read the file content"),
    }
}
