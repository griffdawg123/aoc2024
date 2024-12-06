use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];
    let contents = fs::read_to_string(file_path).expect("Should be able to read the file");
    let lines = contents.split("\n").collect::<Vec<&str>>();

    let mut a: Vec<i32> = vec!();
    let mut b: Vec<i32> = vec!();

    for line in lines {
        dbg!(line);
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

    assert_eq!(a.len(), b.len());

    let mut res: Vec<i32> = vec!();
    for i in 0..a.len() {
        res.push((a[i] - b[i]).abs());
    }

    let sum: i32 = res.into_iter().sum();
    println!("Answer: {sum}");
}
