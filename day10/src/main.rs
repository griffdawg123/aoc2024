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

#[derive (Hash, Eq, PartialEq, Debug, Clone)]
struct Coords {
    row: usize,
    col: usize,
}

fn get_trailheads(map: &Vec<Vec<i32>>) -> HashSet<Coords> {
    let mut res = HashSet::new();
    for (row_idx, row) in map.into_iter().enumerate() {
        for (col_idx, value) in row.into_iter().enumerate() {
            if *value == 0 {
                res.insert(Coords { row: row_idx, col: col_idx});
            }
        }
    }
    res
}

fn do_dfs_part1(map: &Vec<Vec<i32>>, position: &Coords, seen: &mut HashSet<Coords>, level: i32) -> () {
    let mut to_visit = HashSet::new();
    if position.row > 0 && map[position.row - 1][position.col] == level + 1 { to_visit.insert(Coords { row: position.row - 1, col: position.col });}
    if position.row < map.len() - 1 && map[position.row + 1][position.col] == level + 1 { to_visit.insert(Coords { row: position.row + 1, col: position.col });}
    if position.col > 0 && map[position.row][position.col - 1] == level + 1 { to_visit.insert(Coords { row: position.row, col: position.col - 1 });}
    if position.col < map[0].len() - 1 && map[position.row][position.col + 1] == level + 1 { to_visit.insert(Coords { row: position.row, col: position.col + 1 });}

    seen.insert(position.clone());

    for new_location in to_visit {
        do_dfs_part1(map, &new_location, seen, level + 1);
    }
}

fn part1(input: Vec<&str>) -> i32 {
    let map = input.clone().into_iter()
        .filter(|line| !line.is_empty())
        .map(|line| line.chars().map(|c| c.to_string().parse().unwrap()).collect())
        .collect::<Vec<Vec<i32>>>();
    dbg!(&map);
    let trail_heads = get_trailheads(&map);
    let mut res = 0;
    for trail_head in trail_heads {
        let mut reachable = HashSet::new();
        do_dfs_part1(&map, &trail_head, &mut reachable, 0);
        res += &reachable.into_iter()
            .map(|coords| map[coords.row][coords.col])
            .filter(|height| *height == 9)
            .collect::<Vec<i32>>().len();
    }
    return res as i32;
}

fn do_dfs_part2(map: &Vec<Vec<i32>>, position: &Coords, seen: &mut HashSet<Coords>, level: i32) -> i32 {
    if level == 9 { return 1 };
    let mut to_visit = HashSet::new();
    if position.row > 0 && map[position.row - 1][position.col] == level + 1 { to_visit.insert(Coords { row: position.row - 1, col: position.col });}
    if position.row < map.len() - 1 && map[position.row + 1][position.col] == level + 1 { to_visit.insert(Coords { row: position.row + 1, col: position.col });}
    if position.col > 0 && map[position.row][position.col - 1] == level + 1 { to_visit.insert(Coords { row: position.row, col: position.col - 1 });}
    if position.col < map[0].len() - 1 && map[position.row][position.col + 1] == level + 1 { to_visit.insert(Coords { row: position.row, col: position.col + 1 });}

    seen.insert(position.clone());

    let mut res = 0;
    for new_location in to_visit {
        res += do_dfs_part2(map, &new_location, seen, level + 1);
    }
    return res;
}
fn part2(input: Vec<&str>) -> i32 {
    let map = input.clone().into_iter()
        .filter(|line| !line.is_empty())
        .map(|line| line.chars().map(|c| c.to_string().parse().unwrap()).collect())
        .collect::<Vec<Vec<i32>>>();
    let trail_heads = get_trailheads(&map);
    let mut res = 0;
    let mut reachable = HashSet::new();
    for trail_head in trail_heads {
        res += do_dfs_part2(&map, &trail_head, &mut reachable, 0);
    }
    return res as i32;
}
