use std::cmp::{max, min};
use std::error::Error;

fn main() {
    let result = solve(include_str!("input.example")).unwrap();
    println!("{result}");
}

fn solve(input: &str) -> Result<i32, Box<dyn Error>> {
    let mut result = 0;
    let mut mtx: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    let mut i = 0;
    for line in input.lines() {
        for j in 0..mtx[i].len() {
            match explore(&mut mtx, (i, j)) {
                ExplorationResult::NumberPart(range, has_adjacent_symbol) => {
                    if has_adjacent_symbol {
                        let number = &line[range.0..range.1 + 1];
                        println!("{:?}", mtx);
                        println!("{number}");
                        result += number.parse::<i32>()?;
                    }
                }
                _ => (),
            }
        }
        i += 1;
    }
    Ok(result)
}

fn explore(schema: &mut Vec<Vec<char>>, coorinates: (usize, usize)) -> ExplorationResult {
    println!("{:?}", coorinates);
    let symbol = schema[coorinates.0][coorinates.1];
    if symbol == '.' {
        return ExplorationResult::Empty;
    }
    if !symbol.is_digit(10) {
        return ExplorationResult::Symbol;
    }
    schema[coorinates.0][coorinates.1] = '$';
    let mut range = (coorinates.1, coorinates.1);
    let mut has_adjacent_symbol = false;
    let dxs = [-1, 0, 1];
    let dys = dxs;
    for dx in dxs {
        for dy in dys {
            let new_coordinates = (coorinates.0 as i32 + dy, coorinates.1 as i32 + dx);
            let mut invalid = dx <= 0 && dy == 0;
            invalid |= new_coordinates.0 < 0 || new_coordinates.1 < 0;
            if invalid {
                continue;
            }
            let new_coordinates = (new_coordinates.0 as usize, new_coordinates.1 as usize);
            if schema.len() <= new_coordinates.0
                || schema[new_coordinates.0].len() <= new_coordinates.1
            {
                continue;
            }
            let subresult = explore(schema, new_coordinates);
            println!("{:?} {:?}", new_coordinates, subresult);
            if dy == 0 {
                match subresult {
                    ExplorationResult::NumberPart(subrange, has_transitional_adjacent_symbol) => {
                        range.0 = min(range.0, subrange.0);
                        range.1 = max(range.1, subrange.1);
                        has_adjacent_symbol |= has_transitional_adjacent_symbol;
                    }
                    _ => (),
                }
            }
            match subresult {
                ExplorationResult::Symbol => {
                    has_adjacent_symbol = true;
                }
                _ => (),
            }
        }
    }
    ExplorationResult::NumberPart(range, has_adjacent_symbol)
}

#[derive(Debug)]
enum ExplorationResult {
    Empty,
    Symbol,
    NumberPart((usize, usize), bool),
}
