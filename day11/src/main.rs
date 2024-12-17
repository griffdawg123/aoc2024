use std::collections::hash_map;
use std::collections::HashMap;
use std::fs;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];
    let contents = fs::read_to_string(file_path).expect("Should be able to read the file");
    let input = contents.split("\n").collect::<Vec<&str>>();
    println!("Answer: {}", part2(input));
}

fn blink(value: i64) -> Vec<i64> {
    if value == 0 {
        return vec![1];
    } else if value.to_string().len() % 2 == 0 {
        let as_string: Vec<String> = value.to_string().chars().map(|c| c.to_string()).collect();
        let left = &as_string[0..(as_string.len() / 2)];
        let right = &as_string[(as_string.len() / 2)..];
        return vec![left.join("").parse().unwrap(), right.join("").parse().unwrap()];
    } else {
        return vec![value * 2024];
    }
}

//fn part1(input: Vec<&str>) -> i32 {
//    let mut vals: Vec<i32> = input[0].split(" ").map(|v| v.parse().unwrap()).collect();
//    for _ in 0..75 { 
//        vals = vals.into_iter().flat_map(blink).collect(); 
//    }
//    return vals.len() as i32;
//}

fn recursion(counter: &mut HashMap<i64, i64>, depth: i32) -> i64 {
    //dbg!(&depth);
    if depth == 76 { 
        return counter.values().sum::<i64>();
    }

    let mut new_counter: HashMap<i64, i64> = HashMap::new();
    for (number, count) in counter.iter() {
        let blinked = blink(*number);
        for ret in blinked {
            *new_counter.entry(ret).or_insert(0) += count;
        }
    }
    return recursion(&mut new_counter, depth + 1);
}

fn part2(input: Vec<&str>) -> i64 {
    let vals: Vec<i64> = input[0].split(" ").map(|v| v.parse().unwrap()).collect();
    let mut counter: HashMap<i64, i64> = HashMap::new();
    for val in vals {
        *counter.entry(val).or_insert(0) += 1;
    }
    return recursion(&mut counter, 1) as i64;
}


