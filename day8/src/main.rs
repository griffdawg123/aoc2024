use std::collections::HashMap;
use std::collections::HashSet;
use std::fs;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];
    let contents = fs::read_to_string(file_path).expect("Should be able to read the file");
    let input = contents.split("\n").collect::<Vec<&str>>();
    println!("Answer: {}", part2(input));
}

fn part1(input: Vec<&str>) -> i32 {
    // maps chars to a vector of their associated positions
    let mut positions: HashMap<char, Vec<(i32, i32)>> = HashMap::new();

    let rows: i32 = input.len() as i32 - 1;
    let cols: i32 = input[0].len() as i32;
    println!("{} x {}", rows, cols);
    for (row, line) in input.iter().enumerate() {
        for (col, c) in line.chars().enumerate() {
            if c != '.' {
                let mut coord_list = match positions.get(&c) {
                    Some(x) => x.clone(),
                    None => vec![],
                };
                coord_list.push((row as i32, col as i32));
                positions.insert(c, coord_list);
            }
        }
    }

    let mut antinodes: HashSet<(i32, i32)> = HashSet::new();
    for antenna in positions.keys() {
        for a1 in 0..positions[antenna].len() {
            for a2 in a1+1..positions[antenna].len() {
                let (a1x, a1y) = positions[antenna][a1]; 
                let (a2x, a2y) = positions[antenna][a2];

                let offset_x = a2x - a1x; // a2x - offset_x = a1x
                let offset_y = a2y - a1y; // a2y - offset_y = a1y
                
                let (anti1x, anti1y) = (a1x - offset_x, a1y - offset_y);
                let (anti2x, anti2y) = (a2x + offset_x, a2y + offset_y);

                if anti1x >= 0 && anti1x < cols && anti1y >= 0 && anti1y < rows{
                    antinodes.insert((anti1x, anti1y));
                }
                if anti2x >= 0 && anti2x < cols && anti2y >= 0 && anti2y < rows{
                    antinodes.insert((anti2x, anti2y));
                }
            }
        }
    }

    return antinodes.len() as i32;
}

fn part2(input: Vec<&str>) -> i32 {
    // maps chars to a vector of their associated positions
    let mut positions: HashMap<char, Vec<(i32, i32)>> = HashMap::new();

    let rows: i32 = input.len() as i32 - 1;
    let cols: i32 = input[0].len() as i32;
    println!("{} x {}", rows, cols);
    for (row, line) in input.iter().enumerate() {
        for (col, c) in line.chars().enumerate() {
            if c != '.' {
                let mut coord_list = match positions.get(&c) {
                    Some(x) => x.clone(),
                    None => vec![],
                };
                coord_list.push((row as i32, col as i32));
                positions.insert(c, coord_list);
            }
        }
    }

    let mut antinodes: HashSet<(i32, i32)> = HashSet::new();
    for antenna in positions.keys() {
        for a1 in 0..positions[antenna].len() {
            for a2 in a1+1..positions[antenna].len() {
                let (a1x, a1y) = positions[antenna][a1]; 
                let (a2x, a2y) = positions[antenna][a2];

                let offset_x = a2x - a1x; // a2x - offset_x = a1x
                let offset_y = a2y - a1y; // a2y - offset_y = a1y
                
                let (mut anti1x, mut anti1y) = (a1x, a1y);
                let (mut anti2x, mut anti2y) = (a2x, a2y);

                while anti1x >= 0 && anti1x < cols && anti1y >= 0 && anti1y < rows{
                    antinodes.insert((anti1x, anti1y));
                    anti1x -= offset_x;
                    anti1y -= offset_y;
                }
                while anti2x >= 0 && anti2x < cols && anti2y >= 0 && anti2y < rows{
                    antinodes.insert((anti2x, anti2y));
                    anti2x += offset_x;
                    anti2y += offset_y;
                }
            }
        }
    }

    return antinodes.len() as i32;
}
