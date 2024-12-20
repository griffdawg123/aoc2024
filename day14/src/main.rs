use std::fs;
use std::env;

use regex::Regex;

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];
    let contents = fs::read_to_string(file_path).expect("Should be able to read the file");
    let input = contents.split("\n").collect::<Vec<&str>>();
    println!("Answer: {}", part2(input, 101, 103));
    //println!("Answer: {}", part1(input, 11, 7));
}

#[derive(Debug, Clone)]
struct Robot {
    position: (i32, i32),
    velocity: (i32, i32),
}

impl Robot {
    fn move_robot(&mut self, width: i32, height: i32) {
        let (old_x, old_y) = self.position;
        let new_x = old_x + self.velocity.0;
        let new_y = old_y + self.velocity.1;
        self.position = (((new_x % width) + width) % width, ((new_y % height) + height) % height);
    }
}

fn print_board(position_table: &Vec<Vec<i32>>) {
    for y in 0..position_table[0].len() {
        for x in 0..position_table.len() {
            if position_table[x as usize][y as usize] > 0 {print!("{}", position_table[x as usize][y as usize])} else {print!(".")};
        }
        println!();
    }
}

fn get_robots(input: Vec<&str>) -> Vec<Robot> {

    let re = Regex::new(r"p=(\d+),(\d+) v=([-]?\d+),([-]?\d+)").unwrap();
    let mut robots: Vec<Robot> = Vec::new();
    for robot in input {
        for (_, [px, py, vx, vy]) in re.captures_iter(&robot).map(|c| c.extract()) {
            robots.push(Robot { position: (px.parse().unwrap(), py.parse().unwrap()), velocity: (vx.parse().unwrap(), vy.parse().unwrap()) });
        }
    }
    robots
}

fn part1(input: Vec<&str>, width: i32, height: i32) -> i32 {

    let mut robots = get_robots(input);
    let mut final_board: Vec<Vec<i32>> = vec![vec![0; height as usize]; width as usize];
    for _ in 0..100 {
        for robot in robots.iter_mut() {
            robot.move_robot(width, height);
        }
        final_board = vec![vec![0; height as usize]; width as usize];
        for i in 0..robots.len() {
            final_board[robots[i].position.0 as usize][robots[i].position.1 as usize] += 1;
        }
        //print_board(&final_board);
    }


    let mut res = 1;
    for q_col in 0..2 {
        for q_row in 0..2 {
            // 0 0 --> 0..5, 0..5
            // 0 1 --> 0..5, 6..11
            //println!("In quadrant (x, y): {:?}", (q_col, q_row));
            let mut safety_value = 0;
            for x in q_col * (width + 1)/2..(width - 1)/2 + q_col * (width + 1)/2 {
                for y in q_row * (height + 1)/2..(height - 1)/2 + q_row * (height + 1)/2 {
                    //println!("{:?}", (x, y));
                    safety_value += final_board[x as usize][y as usize];
                }
            }
            //dbg!(safety_value);
            res *= safety_value;
        }
    }
    return res;
}

fn count_max_rows(robots: &Vec<Robot>, height: i32) -> i32 {
    let mut row_counts = vec![0; height as usize];
    for robot in robots {
        row_counts[robot.position.1 as usize] += 1;
    }
    return row_counts.into_iter().reduce(|a, b| a.max(b)).unwrap();
}

fn part2(input: Vec<&str>, width: i32, height: i32) -> i32 {
    let mut robots = get_robots(input);
    
    let mut final_board: Vec<Vec<i32>> = vec![vec![0; height as usize]; width as usize];
    for iteration in 0..10000 {
        for robot in robots.iter_mut() {
            robot.move_robot(width, height);
        }
        if count_max_rows(&robots, height) > 20 {
        
            final_board = vec![vec![0; height as usize]; width as usize];
            for i in 0..robots.len() {
                final_board[robots[i].position.0 as usize][robots[i].position.1 as usize] += 1;
            }
            println!("Seconds: {}", iteration);
            print_board(&final_board);
        }
    }

    return -1;
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
    fn part1_example() {
        let contents = fs::read_to_string("src/test.txt").expect("Should be able to read the file");
        let input = contents.split("\n").collect::<Vec<&str>>();
        assert_eq!(part1(input, 11, 7), 12);
    }
}
