use std::cmp::max;
use std::collections::HashSet;
use std::error::Error;

fn main() {
    let result = solve(include_str!("input")).unwrap();
    println!("{result}");
}

#[derive(Debug)]
enum HandKind {
    FullHouse,
    TwoPair,
    SomeOfKind(u8),
}

impl From<&str> for HandKind {
    fn from(hand: &str) -> Self {
        let mut best_char_count = 1;
        let mut pairs = HashSet::new();
        for first_char in hand.chars() {
            let mut count = 0;
            for second_char in hand.chars() {
                if first_char == second_char {
                    count += 1;
                }
            }
            if count == 2 {
                pairs.insert(first_char);
            }
            best_char_count = max(best_char_count, count);
        }
        match (best_char_count, pairs.len()) {
            (3, 1) => HandKind::FullHouse,
            (2, 2) => HandKind::TwoPair,
            _ => HandKind::SomeOfKind(best_char_count),
        }
    }
}

impl From<HandKind> for u32 {
    fn from(kind: HandKind) -> u32 {
        match kind {
            HandKind::FullHouse => 7,
            HandKind::TwoPair => 5,
            HandKind::SomeOfKind(n) => (n << 1).into(),
        }
    }
}

fn to_decimal(hand: &str) -> u32 {
    const TEMPLATE: &str = "23456789TJQKA";
    let mut result = 0;
    let mut base = 1 << 20;
    for card in hand.chars() {
        let val = 1 + TEMPLATE.chars().position(|c| c == card).unwrap() as u32;
        result += base * val;
        base >>= 4
    }
    result
}

fn solve(input: &str) -> Result<u32, Box<dyn Error>> {
    let mut values = Vec::new();
    for line in input.lines() {
        let (hand, bid) = line.split_once(' ').unwrap();
        let kind = HandKind::from(hand);
        let numeric = (u32::from(kind) << 24) + to_decimal(hand);
        values.push((numeric, bid.parse::<u32>()?));
    }
    values.sort();
    let mut result = 0;
    for (i, val) in values.iter().enumerate() {
        result += val.1 * (i + 1) as u32;
    }
    Ok(result)
}
