use std::fs;
use std::env;
use regex::Regex;

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];
    let contents = fs::read_to_string(file_path).expect("Should be able to read the file");
    let memory = contents.split("\n").collect::<Vec<&str>>();
    let mut res = 0;
    let mut active = true;
    for section in memory {
        let re = Regex::new(r"(mul\(\d{1,3},\d{1,3}\)|do\(\)|don't\(\))").unwrap();
        let mut results = vec![];
        for (_, [yuh]) in re.captures_iter(section).map(|c| c.extract()) {
            results.push(yuh);
        }
        let mul_re = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();
        for result in results {
            if result == "do()" {
                active = true;
            } else if result == "don't()" {
                active = false;
            } else if active {
                let num_captures = mul_re.captures(result).unwrap();
                let a: i32 = num_captures.get(1).unwrap().as_str().parse().unwrap();
                let b: i32 = num_captures.get(2).unwrap().as_str().parse().unwrap();
                res += a * b;
            }
        }
    }
    println!("Answer: {res}");
}

fn part1(section: &str) -> i32 {
    // input is memory
    // multiply some numbers
    // mul(x,y) --> x and y are 1-3 diit numbers
    // Some dodgy stuff, ignore everything else
    // Tokenizer
    
    let re = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();
    let mut results = vec![];
    for (_, [a, b]) in re.captures_iter(section).map(|c| c.extract()) {
        results.push((a, b));
    }
    let mut res: i32 = 0;
    for (a, b) in results {
        let a_val: i32 = a.parse().unwrap();
        let b_val: i32 = b.parse().unwrap();

        res += a_val * b_val;
    }
    return res;
}

fn part2(section: &str) -> i32 {
    let re = Regex::new(r"(mul\(\d{1,3},\d{1,3}\)|do\(\)|don't\(\))").unwrap();
    let mut res: i32 = 0;
    let mut results = vec![];
    for (_, [yuh]) in re.captures_iter(section).map(|c| c.extract()) {
        results.push(yuh);
    }
    dbg!(&results);
    let mul_re = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();
    let mut active = true;
    for result in results {
        if result == "do()" {
            active = true;
        } else if result == "don't()" {
            dbg!("Don't!");
            active = false;
        } else if active {
            let num_captures = mul_re.captures(result).unwrap();
            let a: i32 = num_captures.get(1).unwrap().as_str().parse().unwrap();
            let b: i32 = num_captures.get(2).unwrap().as_str().parse().unwrap();
            println!("{} {}", a, b);
            res += a * b;
        }
    }
    return res;
}
