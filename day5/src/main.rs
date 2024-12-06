use std::collections::HashMap;
use std::collections::HashSet;
use std::fs;
use std::env;

/**
* Create a lookup table for a page and its postcessors
* iterate through list, and check each potential 
* 
*
* add seen pages to a set
* for each page in list, see if ? is in set
*
* problem when a page appears and its meant to come before something we've already seen.
* ie its post-page is in the seen set. So we need to check all of its post pagges and 
* see if they are in the set.
*
* Map Page --> post page
* add page to set
* Check post pages with set
* return if no good.
*/

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];
    let contents = fs::read_to_string(file_path).expect("Should be able to read the file");
    let input = contents.split("\n").collect::<Vec<&str>>();
    println!("Answer: {}", part1(input));
}

fn part1(input: Vec<&str>) -> i32 {
    let blank_idx = input.iter().position(|&s| s == "").unwrap();
    let sections: Vec<&[&str]> = input.chunks(blank_idx).collect();
    let rules = sections[0];
    let updates = sections[1];

    // page to its pages that must follow it
    let mut rule_map = HashMap::<i32, Vec<i32>>::new(); 
    for rule in rules {
        let page_numbers: Vec<&str> = rule.split("|").collect();
        let page: i32 = page_numbers[0].parse().unwrap();
        let post: i32 = page_numbers[1].parse().unwrap();
        if let Some(x) = rule_map.get_mut(&page) {
            let mut array = x.clone();
            array.push(post);
            *x = array;
        } else {
            rule_map.insert(page, vec![post]);
        }
    }

    let mut res = 0;
    for update in updates {
        let pages: Vec<&str> = update.split(",").collect();
        let mut okay = true;
        if pages != vec![""] {
            let mut seen = HashSet::<i32>::new();
            for page in &pages {
                let page_number: i32 = page.parse().unwrap();
                let post_pages  = match rule_map.get(&page_number) {
                    Some(x) => &x,
                    None => &vec![],
                };
                for post_page in post_pages {
                    if seen.contains(post_page) {
                        okay = false;
                    }
                }
                seen.insert(page_number);
            }
        }
        if okay {
            res += 1;
        }
    }
    return res;
}

