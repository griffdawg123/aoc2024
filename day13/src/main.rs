use std::fs;
use std::env;

use regex::Regex;

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];
    let contents = fs::read_to_string(file_path).expect("Should be able to read the file");
    //let input = contents.split("\n").collect::<Vec<&str>>();
    println!("Answer: {}", part2(contents));
}

fn find_intercept(a_x: i32, a_y: i32, b_x: i32, b_y: i32, p_x: i32, p_y: i32) -> (i32, i32) {
    let a = (b_y * p_x - b_x * p_y)/(a_x * b_y - a_y * b_x);
    let b = (a_x * p_y - a_y * p_x)/(a_x * b_y - a_y * b_x);
    //dbg!(&(a,b));
    (a, b)
}

#[allow(dead_code)]
fn part1(input: String) -> i32 {

    let re = Regex::new(r"Button A: X\+(?<XA>\d+), Y\+(?<YA>\d+)\nButton B: X\+(?<XB>\d+), Y\+(?<YB>\d+)\nPrize: X=(?<XP>\d+), Y=(?<YP>\d+)").unwrap();
    let mut res = 0;
    for (_, [xa, ya, xb, yb, xp, yp]) in re.captures_iter(&input).map(|c| c.extract()) {
        let (a, b) = find_intercept(xa.parse().unwrap(), ya.parse().unwrap(), xb.parse().unwrap(), yb.parse().unwrap(), xp.parse().unwrap(), yp.parse().unwrap());
        if a >= 0 && a <= 100 
           && b >= 0 && b <= 100 
           && xa.parse::<i32>().unwrap() * a + xb.parse::<i32>().unwrap() * b == xp.parse().unwrap() 
           && ya.parse::<i32>().unwrap() * a + yb.parse::<i32>().unwrap() * b == yp.parse().unwrap() {
            res += 3*a + b;
            //println!("a: {}, b: {}", a, b);
        };
    };
    //println!("A: {:?}, B: {:?}, Prizes: {:?}", a_s, b_s, prize_s);
    return res;
}

// for some a and b we have ax + by = gcd
fn find_extended_gcd(a: i64, b: i64) -> (i64, i64, i64) {
    if b == 0 { return (a, 1, 0) } 
    let (gcd, x1, y1) = find_extended_gcd(b, a % b); 
    let x = x1;
    let y = x1 - ( a / b ) * y1;
    (gcd, x, y)
}

fn calculate_button_presses(a_x: i64, a_y: i64, b_x: i64, b_y: i64, p_x: i64, p_y: i64) -> (i64, i64) {
    let det = a_x * b_y - b_x * a_y;
    if det == 0 { return (-1, -1) };

    let (prize_x, prize_y) = ((p_x * b_y - p_y * b_x), (p_y * a_x - p_x * a_y));
    //dbg!(&(prize_x, prize_y));
    if prize_x % det != 0 || prize_y % det != 0 { return (-1, -1) }
    return (prize_x / det, prize_y / det);
}

// find a combination of button presses that equate to x=y since they can 
// then do that n times to get the 1000000
#[allow(dead_code)]
fn part2(input: String) -> i64 {
    let re = Regex::new(r"Button A: X\+(?<XA>\d+), Y\+(?<YA>\d+)\nButton B: X\+(?<XB>\d+), Y\+(?<YB>\d+)\nPrize: X=(?<XP>\d+), Y=(?<YP>\d+)").unwrap();
    let mut res = 0;
    for (_, [xa, ya, xb, yb, xp, yp]) in re.captures_iter(&input).map(|c| c.extract()) {
        let (a, b) = calculate_button_presses(xa.parse().unwrap(), ya.parse().unwrap(), xb.parse().unwrap(), yb.parse().unwrap(), xp.parse::<i64>().unwrap() + 10_i64.pow(13), yp.parse::<i64>().unwrap() + 10_i64.pow(13));
        if a >= 0 && b >= 0 {
            res += 3*a + b;
            //println!("a: {}, b: {}", a, b);
        };
    };
    //println!("A: {:?}, B: {:?}, Prizes: {:?}", a_s, b_s, prize_s);
    return res;
}

#[cfg(test)]
mod tests {

    use std::fs;
    use super::*;

    #[test]
    fn sanity() {
        let res = 2 + 2;
        assert_eq!(res, 4);
    }

    #[test]
    fn test_part1() {
        let contents = fs::read_to_string("src/test.txt").expect("Should be able to read the file");
        assert_eq!(part1(contents), 480);
    }

    #[test]
    fn test_part2() {
        let contents = fs::read_to_string("src/test.txt").expect("Should be able to read the file");
        assert_eq!(part2(contents), 480);
    }
}
