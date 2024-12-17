use std::fs;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];
    let contents = fs::read_to_string(file_path).expect("Should be able to read the file");
    let input = contents.split("\n").collect::<Vec<&str>>();
    println!("Answer: {}", part1(input));
}

fn part1(input: Vec<&str>) -> i32 {
    let mut res = 0;
    return res;
}

fn part2(input: Vec<&str>) -> i32 {
    let mut res = 0;
    return res;
}
