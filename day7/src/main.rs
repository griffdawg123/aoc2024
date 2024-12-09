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
        let max_bitmask = 2_i64.pow(2 * (input_vals.len() as u32 - 1));
        for combination in 0..max_bitmask {
            let mut value: i64 = input_vals[0].into();
            let mut test = 3;
            for number in 1..input_vals.len() {
                if value > result {
                    break;
                }
                if combination & test == 0 {
                    value += input_vals[number];
                } else if (combination & test) >> ((number - 1)*2) == 1 {
                    value *= input_vals[number];
                } else if (combination & test) >> ((number - 1)*2) == 2 {
                    value *= 10;
                    value += input_vals[number];
                }

                test = test << 2;
            }
            if value == result {
                res += result;
                //print!("{}",input_vals[0]);
                //for number in 1..input_vals.len() {
                //    let test = 3 << ((number - 1) * 2);
                //    if combination & test == 0 {
                //        print!("+");
                //    } else if (combination & test) >> (number - 1)*2 == 1 {
                //        print!("*");
                //    } else if (combination & test) >> (number - 1)*2 == 2 {
                //        print!("||");
                //    }
                //    print!("{}", input_vals[number]);
                //}
                //println!(" = {}", result);
                break;
            }
        }
    }
    return res;
}
