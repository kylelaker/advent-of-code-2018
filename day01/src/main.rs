use std::collections::HashSet;
use std::env;
use std::error::Error;
use std::io::Read;
use std::fs::File;

fn main() -> Result<(), Box<Error>> {
    let mut data = String::new();
    let mut f = File::open("input/input")?;
    f.read_to_string(&mut data)?;

    let part1_result = part1(&data)?;
    let part2_result = part2(&data)?;
    println!("Part 1 = {}", part1_result);
    println!("Part 2 = {}", part2_result);

    return Ok(());
}

fn part1(data: &str) -> Result<i64, Box<Error>> {
    let mut frequency: i64 = 0;
    for line in data.lines() {
        let change: i64 = line.parse()?;
        frequency += change;
    }
    return Ok(frequency);
}

fn part2(data: &str) -> Result<i64, Box<Error>> {
    let mut frequency: i64 = 0;
    let mut seen_frequencies = HashSet::new();
    seen_frequencies.insert(0);

    loop {
        for line in data.lines() {
            let change: i64 = line.parse()?;
            frequency += change;
            if seen_frequencies.contains(&frequency) {
                return Ok(frequency);
            }
            seen_frequencies.insert(frequency);
        }
    }
}
