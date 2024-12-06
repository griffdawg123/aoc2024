use std::fs;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];
    let contents = fs::read_to_string(file_path).expect("Should be able to read the file");
    let lines = contents.split("\n").collect::<Vec<&str>>();

    let mut a: Vec<i32> = vec!();
    let mut b: Vec<i32> = vec!();

    for line in lines {
        if line.is_empty() {
            continue;
        }
        let values = line.split("   ").collect::<Vec<&str>>();
        if !values.is_empty() {
            a.push(values[0].parse().unwrap());
            b.push(values[1].parse().unwrap());
        }
    }

    a.sort();
    b.sort();


    // loop through a_vals and increase b pointer while that value is less than 
    // the a value.
    let mut res = 0;
    let mut i = 0;
    let mut j = 0;
    while i < a.len() {
        // reset multiplier
        let mut multiplier = 0;
        // while we don't have the value of a, increment j
        while j < b.len() && b[j] < a[i] {
            j += 1;
        }
        // while we have dupes of a[i] in b
        while j < b.len() && b[j] == a[i] {
            multiplier += 1;
            j += 1;
        }
        res += multiplier * a[i];
        i += 1;
        while i < a.len() && a[i] == a[i-1] {
            res += multiplier * a[i];
            i += 1;
        }
    }
    println!("Answer: {res}");
}
