use core::num;
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

/**
*   Iterate through the grid and maintain seen coords
*   if we get to a coordinate we have not seen, perform bfs 
*   area is number of squares in the area
*   Perimeter we add the number of different adjacent non-same cells
*/

fn bfs(start: (usize, usize), seen: &mut HashSet<(usize, usize)>, map: &Vec<Vec<char>>) -> (i32, i32) {
    if (*seen).contains(&start) { 
        return (0, 0);
    }
    
    let region_char = map[start.0][start.1];
    let mut perimeter = 0;
    let mut area = 0;
    (*seen).insert(start);
    area += 1;
    perimeter += if start.0 < 1 || map[start.0 - 1][start.1] != region_char { 1 } else { 0 };
    perimeter += if start.0 + 1 >= map.len() || map[start.0 + 1][start.1] != region_char { 1 } else { 0 };
    perimeter += if start.1 < 1 || map[start.0][start.1 - 1] != region_char { 1 } else { 0 };
    perimeter += if start.1 + 1 >= map[0].len() || map[start.0][start.1 + 1] != region_char { 1 } else { 0 };


    if start.0 >= 1 && map[start.0 - 1][start.1] == region_char {
        let bfs_res = bfs((start.0-1, start.1), seen, map);
        perimeter += bfs_res.0;
        area += bfs_res.1;
    }
    if start.0 < map.len() - 1 && map[start.0 + 1][start.1] == region_char {
        let bfs_res = bfs((start.0+1, start.1), seen, map);
        perimeter += bfs_res.0;
        area += bfs_res.1;
    }
    if start.1 >= 1 && map[start.0][start.1 - 1] == region_char {
        let bfs_res = bfs((start.0, start.1-1), seen, map);
        perimeter += bfs_res.0;
        area += bfs_res.1;
    }
    if start.1 < map[0].len() - 1 && map[start.0][start.1 + 1] == region_char {
        let bfs_res = bfs((start.0, start.1+1), seen, map);
        perimeter += bfs_res.0;
        area += bfs_res.1;
    }
    return (perimeter, area);
}

fn part1(input: Vec<&str>) -> i32 {
    let mut seen: HashSet<(usize, usize)> = HashSet::new();
    let map: Vec<Vec<char>> = input.into_iter()
        .filter(|x| !x.is_empty())
        .map(|line| line.chars().collect())
        .collect();

    let mut res = 0;
    for (row_idx, row) in map.clone().into_iter().enumerate() {
        for (col_idx, _) in row.into_iter().enumerate() {
            if !seen.contains(&(row_idx, col_idx)) {
                let bfs_res = bfs((row_idx, col_idx), &mut seen, &map);
                //println!("Perimeter: {}, Area: {}", bfs_res.0, bfs_res.1);
                res += bfs_res.0 * bfs_res.1;
            }
        }
    }
    return res;
}


/**
+       +
 A A A A
+   + + +
 B B C D
      + +
 B B C C
+   + + |
 E E E C
*/


fn bfs_p2(start: (usize, usize), seen: &mut HashSet<(usize, usize)>, map: &Vec<Vec<char>>) -> (i32, i32) {
    if (*seen).contains(&start) { 
        return (0, 0);
    }
    
    let region_char = map[start.0][start.1];
    let mut num_edges = 0;
    let mut area = 0;
    (*seen).insert(start);
    area += 1;

    // check if diagonal cell is different or off the map
    if (start.0 == 0 || start.1 == 0) || map[start.0 - 1][start.1 - 1] != region_char {
        // if other corners are different or off the map
        // convex corner
        if (start.1 == 0 || map[start.0][start.1 - 1] != region_char) && // W
        (start.0 == 0 || map[start.0 - 1][start.1] != region_char) // N
        { // NW
            //println!("Convex corner NW at {:?}", start);
            num_edges += 1;
        }
        // need both corners to be on the map with them being the same.
        // concave corner
        if (start.1 > 0 && map[start.0][start.1 - 1] == region_char) && // W
        (start.0 > 0 && map[start.0 - 1][start.1] == region_char ) {  // N
            //println!("Concave corner NW at {:?}", start);
            num_edges += 1;
        }
    } else if map[start.0 - 1][start.1 - 1] == region_char {
        if ( map[start.0][start.1 - 1] != region_char) && // W
        ( map[start.0 - 1][start.1] != region_char) // N
        { // NW
            //println!("Convex corner NW at {:?} diagonal", start);
            num_edges += 1;
        }
    }

    // NE - first check we have a potential candidate
    if (start.0 == 0 || start.1 == map[0].len()-1) || map[start.0 - 1][start.1 + 1] != region_char {
        // convex corner
        if (start.1 == map[0].len() - 1 || map[start.0][start.1 + 1] != region_char) && // W
        (start.0 == 0 || map[start.0 - 1][start.1] != region_char) // N
        { // NW
            //println!("Convex corner NE at {:?}", start);
            num_edges += 1;
        }
        // concave corner
        if (start.1 < map[0].len() - 1 && map[start.0][start.1 + 1] == region_char) && // W
        (start.0 > 0 && map[start.0 - 1][start.1] == region_char ) {  // N
            //println!("Concave corner NE at {:?}", start);
            num_edges += 1;
        }
    } else if map[start.0 - 1][start.1 + 1] == region_char {
        if ( map[start.0][start.1 + 1] != region_char) && // W
        ( map[start.0 - 1][start.1] != region_char) // N
        { // NW
            //println!("Convex corner NE at {:?} diagonal", start);
            num_edges += 1;
        }
    }


    // SE
    //
    if (start.0 == map.len()-1 || start.1 == map[0].len()-1) || map[start.0 + 1][start.1 + 1] != region_char {
        // convex corner
        if (start.1 == map[0].len() - 1 || map[start.0][start.1 + 1] != region_char) && // W
        (start.0 == map.len() - 1 || map[start.0 + 1][start.1] != region_char) // N
        { // NW
            //println!("Convex corner SE at {:?}", start);
            num_edges += 1;
        }
        // concave corner
        if (start.1 < map[0].len() - 1 && map[start.0][start.1 + 1] == region_char) && // W
        (start.0 < map.len() - 1 && map[start.0 + 1][start.1] == region_char ) {  // N
            //println!("Concave corner SE at {:?}", start);
            num_edges += 1;
        }
    } else if map[start.0 + 1][start.1 + 1] == region_char {
        if (map[start.0][start.1 + 1] != region_char) && // W
        (map[start.0 + 1][start.1] != region_char) // N
        { // NW
            //println!("Convex corner SE at {:?} diagonal", start);
            num_edges += 1;
        }
    }
    
    
    // SW
    if (start.0 == map.len() - 1 || start.1 == 0) || map[start.0 + 1][start.1 - 1] != region_char {
        // convex corner
        if (start.1 == 0 || map[start.0][start.1 - 1] != region_char) && // W
        (start.0 == map.len() - 1 || map[start.0 + 1][start.1] != region_char) // N
        { // NW
            //println!("Convex corner SW at {:?}", start);
            num_edges += 1;
        }
        // concave corner
        if (start.1 > 0 && map[start.0][start.1 - 1] == region_char) && // W
        (start.0 < map.len() - 1 && map[start.0 + 1][start.1] == region_char ) {  // N
            //println!("Concave corner SW at {:?}", start);
            num_edges += 1;
        }
    } else if map[start.0 + 1][start.1 - 1] == region_char {
        if ( map[start.0][start.1 - 1] != region_char) && // W
        ( map[start.0 + 1][start.1] != region_char) // N
        { // NW
            //println!("Convex corner SW at {:?} diagonal", start);
            num_edges += 1;
        }
    }

    if start.0 >= 1 && map[start.0 - 1][start.1] == region_char {
        //println!("up");
        let bfs_res = bfs_p2((start.0-1, start.1), seen, map);
        num_edges += bfs_res.0;
        area += bfs_res.1;
    }
    if start.0 < map.len() - 1 && map[start.0 + 1][start.1] == region_char {
        //println!("down");
        let bfs_res = bfs_p2((start.0+1, start.1), seen, map);
        num_edges += bfs_res.0;
        area += bfs_res.1;
    }
    if start.1 >= 1 && map[start.0][start.1 - 1] == region_char {
        //println!("left");
        let bfs_res = bfs_p2((start.0, start.1-1), seen, map);
        num_edges += bfs_res.0;
        area += bfs_res.1;
    }
    if start.1 < map[0].len() - 1 && map[start.0][start.1 + 1] == region_char {
        //println!("right");
        let bfs_res = bfs_p2((start.0, start.1+1), seen, map);
        num_edges += bfs_res.0;
        area += bfs_res.1;
    }
    return (num_edges, area);
}

fn part2(input: Vec<&str>) -> i32 {
    let mut seen: HashSet<(usize, usize)> = HashSet::new();
    let map: Vec<Vec<char>> = input.into_iter()
        .filter(|x| !x.is_empty())
        .map(|line| line.chars().collect())
        .collect();

    let mut res = 0;
    for (row_idx, row) in map.clone().into_iter().enumerate() {
        for (col_idx, c) in row.into_iter().enumerate() {
            if !seen.contains(&(row_idx, col_idx)) {
                let bfs_res = bfs_p2((row_idx, col_idx), &mut seen, &map);
                //println!("Num_edges: {}, Area: {}, Cost: {}, Region: {}", bfs_res.0, bfs_res.1, bfs_res.0 * bfs_res.1, c);
                res += bfs_res.0 * bfs_res.1;
            }
        }
    }
    return res;
}
