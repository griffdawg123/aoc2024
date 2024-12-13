use std::fs;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];
    let contents = fs::read_to_string(file_path).expect("Should be able to read the file");
    let input = contents.split("\n").collect::<Vec<&str>>();
    println!("Answer: {}", part2(input));
}

fn part1(input: Vec<&str>) -> i64 {
    let layout = input[0];
    let mut is_file = true;
    let mut file_id = 0;
    let mut diskmap: Vec<i64> = vec![];

    for digit in layout.split("").filter(|x| !x.is_empty()).map(|x| x.parse::<i64>().unwrap()) {
        let to_insert = if is_file {file_id} else {-1};
        for _ in 0..digit {
            diskmap.push(to_insert);
        }
        if is_file {file_id += 1};
        is_file = !is_file;
    }

    let mut left = 0;
    let mut right = diskmap.len() - 1;

    while left < right {
        while diskmap[left] != -1 {
            left += 1;
        }
        while diskmap[right] == -1 {
            right -= 1;
        }
        if right <= left {
            break;
        }
        (diskmap[left], diskmap[right]) = (diskmap[right], diskmap[left]);
        left += 1;
        right -= 1;
    }

    let mut res = 0;
    for i in 0..diskmap.len() {
        let val = diskmap[i];
        if val == -1 { return res };
        res += i as i64 * val;
    }
    return res;
}

fn part2(input: Vec<&str>) -> i64 {

    let layout = input[0];
    let mut is_file = true;
    let mut file_id = 0;
    let mut diskmap: Vec<i64> = vec![];

    for digit in layout.split("").filter(|x| !x.is_empty()).map(|x| x.parse::<i64>().unwrap()) {
        let to_insert = if is_file {file_id} else {-1};
        for _ in 0..digit {
            diskmap.push(to_insert);
        }
        if is_file {file_id += 1};
        is_file = !is_file;
    }

    file_id -= 1;
    //dbg!(file_id);
    //dbg!(&diskmap);
    while file_id >= 0 {
        let mut empty_start = 0;
        while empty_start < diskmap.len() {
            while empty_start < diskmap.len() && diskmap[empty_start] != -1 {
                empty_start += 1;
            }
            if empty_start >= diskmap.len() {break;}
            let mut empty_end = empty_start.clone();
            while empty_end < diskmap.len() && diskmap[empty_end] == -1 {
                empty_end += 1;
            }
            // diskmap[empty_start..empty_end] is a contigious space of empty diskspace
            
            let mut block_end = diskmap.len() - 1;
            // find left hand side of the empty block, then find the right hand side
            // we can find the lenght of th
            while diskmap[block_end] != file_id {
                block_end -= 1;
            }
            // if we cant move it left - finish
            if block_end <= empty_start {
                break;
            }
            let mut block_start = block_end.clone();
            while diskmap[block_start] == file_id {
                block_start -= 1;
            }
            // we stop when we find the file_id, so pop it up one to have normal bounds
            block_end += 1;
            // we stop when we no longer have the file_id, so increase it so it is the start
            block_start += 1;
            // if the empty space is too small, move past it and try and find another
            println!("Empty is {}..{}: {:?}", empty_start, empty_end, (empty_start..empty_end).map(|i| diskmap[i]).collect::<Vec<i64>>());
            println!("Block is {}..{}: {:?}", block_start, block_end, (block_start..block_end).map(|i| diskmap[i]).collect::<Vec<i64>>());
            if block_end - block_start > empty_end - empty_start {
                empty_start = empty_end;
            } else {
                // else move the block and break to the next file_id
                for block_idx in 0..block_end-block_start {
                    (diskmap[block_start + block_idx], diskmap[empty_start + block_idx]) = (diskmap[empty_start + block_idx], diskmap[block_start + block_idx]);
                }
                println!("New Diskmap: {:?}", &diskmap);
                break;
            }
        }
        file_id -= 1;
    }
    let mut res = 0;
    for i in 0..diskmap.len() {
        let val = diskmap[i];
        if val != -1 { 
            res += i as i64 * val;
        }
    }
    return res;
}
