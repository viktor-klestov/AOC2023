use std::collections::HashSet;
use std::error::Error;

fn main() {
    let input = include_str!("input");
    let result = solve(input).unwrap();
    println!("{result}");
}

fn solve(input: &str) -> Result<u64, Box<dyn Error>> {
    let mut result = 0;
    let mut increments: Vec<(u8, u32)> = Vec::new();
    for line in input.lines() {
        let line = line.split_once(':').unwrap().1;
        let (winning, actual) = line.split_once('|').unwrap();
        let mut lucky_numbers = HashSet::new();
        for number in winning.trim().split(' ') {
            if !number.is_empty() {
                lucky_numbers.insert(number.parse::<u8>()?);
            }
        }
        let mut points = 0;
        for number in actual.trim().split(' ') {
            if !number.is_empty() && lucky_numbers.contains(&number.parse::<u8>()?) {
                points += 1;
            }
        }
        let mut new_increments = Vec::new();
        let mut copies = 1;
        for (length, count) in &increments {
            if *length > 1 {
                new_increments.push((*length - 1, *count));
            }
            copies += *count;
        }
        if points > 0 {
            new_increments.push((points, copies));
        }
        increments = new_increments;
        result += copies as u64;
    }
    Ok(result)
}
