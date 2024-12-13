use std::default;
use std::fs;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];
    let contents = fs::read_to_string(file_path).expect("Should be able to read the file");
    let input = contents.split("\n").filter(|x| !x.is_empty()).collect::<Vec<&str>>();
    println!("Answer: {}", part2(input));
}

fn part1(input: Vec<&str>) -> i64 {

    let mut res = 0;
    for line in input { 
        let sections: Vec<&str> = line.split(":").collect();
        let result: i64 = sections[0].parse().unwrap();
        let input_vals: Vec<i64> = sections[1].split(" ").filter(|x| !x.is_empty()).map(|x| x.parse().unwrap()).collect();
        let max_bitmask = 2_i64.pow(input_vals.len().try_into().unwrap()) / 2;
        for combination in 0..max_bitmask {
            let mut value: i64 = input_vals[0].into();
            let mut test = 1;
            for number in 1..input_vals.len() {
                if value > result {
                    break;
                }
                if combination & test == 0 {
                    value += input_vals[number];
                } else {
                    value *= input_vals[number];
                }
                test = test << 1;
            }
            if value == result {
                res += result;
                break;
            }
        }
    }

    return res;
}

fn part2(input: Vec<&str>) -> i64 {
    let mut res = 0;
    for line in input {
        let sections: Vec<&str> = line.split(":").collect();
        let result: i64 = sections[0].parse().unwrap();
        let input_vals: Vec<i64> = sections[1].split(" ").filter(|x| !x.is_empty()).map(|x| x.parse().unwrap()).collect();
        if let (true, _) = find_combination(result, input_vals[0], input_vals[1..].to_vec()) { 
            res += result;
        }
    }
    return res;
}

fn find_combination(result: i64, value: i64, rest: Vec<i64>) -> (bool, i64) {
    if rest.is_empty() {
        return (result == value, value);
    }
    if value > result {
        return (false, -1);
    }
    if let (true, x) = find_combination(result, value + rest[0], rest[1..].to_vec()) {
        return (true, x);
    }
    if let (true, x) = find_combination(result, value * rest[0], rest[1..].to_vec()) {
        return (true, x);
    }
    let v_string = value.to_string();
    let rest_string = rest[0].to_string();
    let concat_string = v_string + &rest_string;
    let concat_val: i64 = concat_string.parse().unwrap();
    if let (true, x) = find_combination(result, concat_val, rest[1..].to_vec()) {
        return (true, x);
    }
    return (false, -1);
}
