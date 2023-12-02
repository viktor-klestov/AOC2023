use std::cmp::max;
use std::error::Error;

fn main() {
    let result = solve(include_str!("input")).unwrap();
    println!("{result}");
}

fn index(color: &str) -> usize {
    match color {
        "red" => 0,
        "green" => 1,
        _ => 2,
    }
}

fn solve(input: &str) -> Result<u32, Box<dyn Error + 'static>> {
    let mut result = 0;
    for line in input.lines() {
        let mut mx = [0, 0, 0];
        let line = line.split(':').nth(1).unwrap();
        for set in line.split(';') {
            for group in set.split(',') {
                let mut parts = group.trim_start().split(' ');
                let count = parts.next().unwrap().parse()?;
                let color_idx = index(parts.next().unwrap());
                mx[color_idx] = max(mx[color_idx], count);
            }
        }
        result += mx.iter().product::<u32>();
    }
    Ok(result)
}
