use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];
    let contents = fs::read_to_string(file_path).expect("Should be able to read the file");
    let reports = contents.split("\n").collect::<Vec<&str>>();


    let res = part2(reports);
    println!("Answer: {res}");
}

fn part1(reports: Vec<&str>) -> i32 {
    let mut res = 0;
    for report in reports {
        if report.len() == 0 { break };
        let levels = report.split(" ").collect::<Vec<&str>>();
        dbg!(&levels);
        let level_vals: Vec<i32> = levels.clone().into_iter().map(|x| x.parse().unwrap()).collect();
        let direction = if level_vals[0] < level_vals[1] { 1 } else { -1 };
        let mut i = 1;
        while i < levels.len() {
            if (level_vals[i] - level_vals[i-1]) * direction <= 0 || // if its not going the right way or is equal
            (level_vals[i] - level_vals[i-1]) * direction > 3 {break}  // if its more than 3
            i += 1;
        }
        if i == levels.len() {
            res += 1;
        }
    }
    
    return res;
}

fn part2(reports: Vec<&str>) -> i32 {
    let mut res = 0;
    for report in reports {
        if report.len() == 0 { break };
        let levels = report.split(" ").collect::<Vec<&str>>();
        let level_vals: Vec<i32> = levels.clone().into_iter().map(|x| x.parse().unwrap()).collect();
        let mut removed_idx = 0;
        while removed_idx <= levels.len() {
            let mut i = 1; 
            // create new list with one level removed (or not);
            let mut new_levels = level_vals.clone();
            if removed_idx < levels.len() {
                new_levels.remove(removed_idx);
            }
            // update direction if needed
            let direction = if new_levels[0] < new_levels[1] { 1 } else { -1 };
            // loop through the list, if its no good break
            while i < new_levels.len() {
                if (new_levels[i] - new_levels[i-1]) * direction <= 0 || // if its not going the right way or is equal
                (new_levels[i] - new_levels[i-1]) * direction > 3 {break}  // if its more than 3
                i += 1;
            }
            // if we have reached the end of the new list - its good
            if i == new_levels.len() {
                res += 1;
                break;
            } else {
                // otherwise continue
                removed_idx += 1;
            }
        }
    }
    return res;
}
