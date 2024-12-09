use std::fs;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];
    let contents = fs::read_to_string(file_path).expect("Should be able to read the file");
    let input = contents.split("\n").collect::<Vec<&str>>();
    println!("Answer: {}", part1(input));
}

fn part1(input: Vec<&str>) -> i64 {
    let layout = input[0];
    let mut is_file = true;
    let mut file_id = 0;
    let mut diskmap: Vec<i64> = vec![];

    for digit in layout.split("").filter(|x| !x.is_empty()).map(|x| x.parse::<i64>().unwrap()) {
        let to_insert = if is_file {file_id} else {-1};
        for _ in 0..digit {
            diskmap.push(to_insert);
        }
        if is_file {file_id += 1};
        is_file = !is_file;
    }

    let mut left = 0;
    let mut right = diskmap.len() - 1;

    while left < right {
        while diskmap[left] != -1 {
            left += 1;
        }
        while diskmap[right] == -1 {
            right -= 1;
        }
        if right <= left {
            break;
        }
        (diskmap[left], diskmap[right]) = (diskmap[right], diskmap[left]);
        left += 1;
        right -= 1;
    }

    let mut res = 0;
    for i in 0..diskmap.len() {
        let val = diskmap[i];
        if val == -1 { return res };
        res += i as i64 * val;
    }
    return res;
}

fn part2(input: Vec<&str>) -> i32 {
    let mut res = 0;
    return res;
}
