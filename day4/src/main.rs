use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];
    let contents = fs::read_to_string(file_path).expect("Should be able to read the file");
    let lines = contents.split("\n").collect::<Vec<&str>>();
    dbg!(&lines);
    println!("Answer: {}", part2(lines));
}

fn part1(lines: Vec<&str>) -> i32 {
    let mut res = 0;
    let cols = lines[0].len();
    let rows = lines.len();
    for ( i, line ) in lines.clone().into_iter().enumerate() {
        for ( j, c ) in line.chars().enumerate() {
            if c == 'X' {
                //print!("{}", c);
                //println!("X at {} {}", i, j);
                // horizontal and East diagonal
                if j + 3 < cols { 
                    if line.chars().nth(j+1) == Some('M') &&
                        line.chars().nth(j+2) == Some('A') &&
                        line.chars().nth(j+3) == Some('S') { // XMAS
                        //println!("{} {} h", i, j);
                        res += 1;
                    }
                    if i >= 3 { // X^M^A^S
                        if lines[i-1].chars().nth(j+1) == Some('M') &&
                        lines[i-2].chars().nth(j+2) == Some('A') &&
                        lines[i-3].chars().nth(j+3) == Some('S') {
                        //println!("{} {} NE", i, j);
                            res += 1;
                        }
                    }
                    if i + 3 < rows-1 { //XvMvAvS
                        if lines [i+1].chars().nth(j+1) == Some('M') &&
                        lines[i+2].chars().nth(j+2) == Some('A') &&
                        lines[i+3].chars().nth(j+3) == Some('S') {
                        //println!("{} {} SE", i, j);
                            res += 1;
                        }
                    }
                    // backwards horizontal and west diagonal
                } 
                if j >= 3 {
                    if line.chars().nth(j-1) == Some('M') &&
                    line.chars().nth(j-2) == Some('A') &&
                    line.chars().nth(j-3) == Some('S') { // SMAX
                        //println!("{} {} hb", i, j);
                        res += 1;
                    }
                    if i >= 3 { // SvMvAvX
                        if lines[i-1].chars().nth(j-1) == Some('M') &&
                        lines[i-2].chars().nth(j-2) == Some('A') &&
                        lines[i-3].chars().nth(j-3) == Some('S') {
                        //println!("{} {} NW", i, j);
                            res += 1;
                        }
                    }
                    if i + 3 < rows - 1 { // S^M^A^X
                        if lines [i+1].chars().nth(j-1) == Some('M') &&
                        lines[i+2].chars().nth(j-2) == Some('A') &&
                        lines[i+3].chars().nth(j-3) == Some('S') {
                        //println!("{} {} SW", i, j);
                            res += 1;
                        }
                    }
                }                 
                if i + 3 < rows - 1 { // X/M/A/S
                    if lines[i+1].chars().nth(j) == Some('M') &&
                    lines[i+2].chars().nth(j) == Some('A') &&
                    lines[i+3].chars().nth(j) == Some('S') {
                        //println!("{} {} v", i, j);
                        res += 1;
                    }
                }
                if i >= 3 { // S/M/A/X
                    if lines[i-1].chars().nth(j) == Some('M') &&
                    lines[i-2].chars().nth(j) == Some('A') &&
                    lines[i-3].chars().nth(j) == Some('S') {
                        //println!("{} {} vb", i, j);
                        res += 1;
                    }
                }
            } 
        }
        //print!("\n");
    }
    return res;
}

fn part2(lines: Vec<&str>) -> i32 {
    let mut res = 0;
    let cols = lines[0].len();
    let rows = lines.len();
    for ( i, line ) in lines.clone().into_iter().enumerate() {
        for ( j, c ) in line.chars().enumerate() {
            if c == 'A' {
                if i > 0 && i + 1 < rows - 1 && j > 0 && j + 1 < cols {
                    if lines[i-1].chars().nth(j-1) == Some('M') && 
                    lines[i-1].chars().nth(j+1) == Some('M') &&
                    lines[i+1].chars().nth(j-1) == Some('S') &&
                    lines[i+1].chars().nth(j+1) == Some('S') { // MM
                        res += 1;
                    }
                    if lines[i-1].chars().nth(j-1) == Some('M') && 
                    lines[i-1].chars().nth(j+1) == Some('S') &&
                    lines[i+1].chars().nth(j-1) == Some('M') &&
                    lines[i+1].chars().nth(j+1) == Some('S') { // MS
                        res += 1;
                    }
                    if lines[i-1].chars().nth(j-1) == Some('S') && 
                    lines[i-1].chars().nth(j+1) == Some('M') &&
                    lines[i+1].chars().nth(j-1) == Some('S') &&
                    lines[i+1].chars().nth(j+1) == Some('M') { // SM
                        res += 1;
                    }
                    if lines[i-1].chars().nth(j-1) == Some('S') && 
                    lines[i-1].chars().nth(j+1) == Some('S') &&
                    lines[i+1].chars().nth(j-1) == Some('M') &&
                    lines[i+1].chars().nth(j+1) == Some('M') { // MM
                        res += 1;
                    }
                }
            }
        }
    }
    return res;
}
