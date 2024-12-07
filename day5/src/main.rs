use std::cmp::Ordering;
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
    println!("Answer: {}", part2(input));
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

    //dbg!(&rule_map);

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
                        //dbg!(&seen);
                        //dbg!(&rule_map.get(&page_number));
                        okay = false;
                    }
                }
                seen.insert(page_number);
            }
            if okay == true {
                let num_pages = pages.len();
                assert_eq!(num_pages % 2, 1);
                let middle_idx = (num_pages - 1)/2;
                res += &pages[middle_idx].parse().unwrap();
            }
        }
    }
    return res;
}

#[derive(Debug)]
struct Page {
    number: i32,
    post_pages: Option<Vec<i32>>,
}

impl Ord for Page {
    fn cmp(&self, other: &Self) -> Ordering {
        if let Some(x) = self.post_pages.clone() {
            if x.contains(&other.number) {
                return Ordering::Less;
            }
        } else if let Some(x) = other.post_pages.clone(){
            if x.contains(&self.number)  {
                return Ordering::Greater;
            }
        } 
        return Ordering::Equal;
    }
}

impl PartialOrd for Page {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Page {
    fn eq(&self, other: &Self) -> bool {
        return self.number == other.number || (!self.post_pages.clone().expect("something").contains(&other.number) && !other.post_pages.clone().expect("something").contains(&self.number))
    }
}

impl Eq for Page {}

fn part2(input: Vec<&str>) -> i32 {
    
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

    let mut out_of_order: HashSet<&str> = HashSet::new();
    for update in updates {
        let pages: Vec<&str> = update.split(",").collect();
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
                        out_of_order.insert(update);
                    }
                }
                seen.insert(page_number);
            }
        }
    }

    let mut res = 0;
    for update in out_of_order {
        let page_nums: Vec<i32> = update.split(",").map(|x| x.parse::<i32>().unwrap()).collect();
        let mut as_pages: Vec<Page> = page_nums.into_iter().map(|x| Page { number: x, post_pages: rule_map.get(&x).cloned()}).collect();
        as_pages.sort_by(|a,b| a.cmp(b));
        assert_eq!(as_pages.len() % 2, 1);
        res += match as_pages.get((as_pages.len() - 1 )/ 2) {
            Some(x) => x.number,
            None => 0,
        }
    }

    return res;
}
