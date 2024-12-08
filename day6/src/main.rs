use std::collections::HashSet;
use std::fs;
use std::env;
use std::hash::Hash;

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];
    let contents = fs::read_to_string(file_path).expect("Should be able to read the file");
    let input = contents.split("\n").collect::<Vec<&str>>();
    println!("Answer: {}", part2(input));
}

#[derive(Eq, Hash, PartialEq, Debug, Clone)]
struct Coords {
    x: i32,
    y: i32,
}

enum Direction {
    Up,
    Left,
    Down,
    Right,
}

fn move_player(current_pos: &Coords, direction: &Direction) -> Coords {
    let offset_vec = match direction {
        Direction::Up => Coords { x: 0, y: -1 },
        Direction::Left => Coords { x: -1, y: 0 },
        Direction::Down => Coords { x: 0, y: 1 },
        Direction::Right => Coords { x: 1, y: 0 },
    };
    return Coords { x: current_pos.x + offset_vec.x, y: current_pos.y + offset_vec.y };
}

fn part1(input: Vec<&str>) -> i32 {
    let mut current_coords = Coords{ x: 0, y: 0};   
    let mut blockages: HashSet<Coords> = HashSet::new();
    let rows = input.len() as i32 - 1; // accounts for last 
    let cols = input[0].len() as i32;
    let mut current_direction = 0;
    for (i, row) in input.iter().enumerate() {
        for (j, c) in row.chars().enumerate() {
            if c == '^' {
                current_coords = Coords { x: j as i32, y: i as i32 };
            } else if c == '#' {
                blockages.insert(Coords { x: j as i32, y: i as i32 });
            }
        }
    }
    let directions = [Direction::Up, Direction::Right, Direction::Down, Direction::Left];
    let mut next_position;
    let mut seen: HashSet<Coords> = HashSet::new();
    seen.insert(current_coords.clone());
    while current_coords.x < cols && current_coords.x >= 0 && current_coords.y < rows && current_coords.y >= 0 {
        next_position = move_player(&current_coords, &directions[current_direction]);
        if blockages.contains(&next_position) {
            current_direction = (current_direction + 1) % directions.len();
        } else {
            current_coords = next_position.clone();
            seen.insert(next_position);
        }
    }
    return seen.into_iter().filter(|coord| coord.x >= 0 && coord.x < cols && coord.y >= 0 && coord.y < rows).collect::<Vec<Coords>>().len() as i32;
}

fn part2(input: Vec<&str>) -> i32 {
    // For each empty space, place a blockage
    // As we travel through, add a coordinate and a direction
    // If we find already have a coordinate x direction pairing in the seen set, increase by one.
    let mut starting_coords = Coords{ x: 0, y: 0};   
    let mut initial_blockages: HashSet<Coords> = HashSet::new();
    let rows = input.len() as i32 - 1; // accounts for last 
    let cols = input[0].len() as i32;
    for (i, row) in input.iter().enumerate() {
        for (j, c) in row.chars().enumerate() {
            if c == '^' {
                starting_coords = Coords { x: j as i32, y: i as i32 };
            } else if c == '#' {
                initial_blockages.insert(Coords { x: j as i32, y: i as i32 });
            }
        }
    }
    let directions = [Direction::Up, Direction::Right, Direction::Down, Direction::Left];
    // loop over each position
    let mut res = 0;
    for i in 0..rows {
        for j in 0..cols {
            let test = Coords { x: j as i32, y: i as i32 };
            dbg!(&test);
            let mut current_coords = starting_coords.clone();
            let mut seen: HashSet<(i32, Coords)> = HashSet::new();
            let mut current_direction = 0;
            seen.insert((0, starting_coords.clone()));
            if test != starting_coords && !initial_blockages.contains(&test) {
                let mut blockages = initial_blockages.clone();
                blockages.insert(test.clone());
                let mut next_position;
                while current_coords.x < cols && current_coords.x >= 0 && current_coords.y < rows && current_coords.y >= 0 {
                    next_position = move_player(&current_coords, &directions[current_direction as usize]);
                    if blockages.contains(&next_position) {
                        current_direction = (current_direction + 1) % directions.len() as i32;
                    } else if seen.contains(&(current_direction, next_position.clone())) {
                        res += 1;
                        break;
                    } else {
                        current_coords = next_position.clone();
                        //let before = seen.len();
                        seen.insert((current_direction, next_position ));
                        //if seen.len() != before {
                        //    dbg!(seen.len());
                        //}
                    }
                
            }
        }
    }
    }
    return res;
}
