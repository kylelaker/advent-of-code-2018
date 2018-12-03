use std::collections::HashMap;
use std::collections::HashSet;
use std::env;
use std::error::Error;
use std::io::Read;
use std::fs::File;

fn main() -> Result<(), Box<Error>> {
    let mut data = String::new();
    let mut f = File::open("input/input")?;
    f.read_to_string(&mut data)?;

    let checksum = part1(&data)?;
    let same_chars = part2(&data).unwrap();
    println!("Checksum = {}", checksum);
    println!("Same chars = {}", same_chars);

    return Ok(());
}

fn part1(data: &str) -> Result<i64, Box<Error>> {
    let mut two: i64 = 0;
    let mut three: i64 = 0;

    for line in data.lines() {
        let mut counter = HashMap::new();
        for letter in line.chars() {
            let count = counter.entry(letter).or_insert(0);
            *count += 1;
        }
        let values: HashSet<i64> = counter.values().cloned().collect();
        if values.contains(&2) {
            two += 1;
        }
        if values.contains(&3) {
            three += 1;
        }
    }

    let checksum = two * three;

    return Ok(checksum);
}

fn part2(data: &str) -> Result<String, ()> {
    for line_1 in data.lines() {
        for line_2 in data.lines() {
            let mut num_differences = 0;
            let mut diff_idx = 0;
            let mut same_chars = String::new();

            for (c1, c2) in line_1.chars().zip(line_2.chars()) {
                if c1 != c2 {
                    num_differences += 1;
                } else {
                    same_chars.push(c1);
                }
            }

            if num_differences == 1 {
                return Ok(same_chars);
            }
        }
    }

    return Err(());
}
