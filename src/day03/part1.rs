use std::error::Error;

fn main() {
    let result = solve(include_str!("input")).unwrap();
    println!("{result}");
}

fn solve(input: &str) -> Result<u32, Box<dyn Error>> {
    let mut result = 0;
    let mtx: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    for i in 0..mtx.len() {
        let mut buffer = String::new();
        let mut has_adjacent_symbol = false;
        for j in 0..mtx[i].len() {
            let symbol = mtx[i][j];
            if symbol.is_digit(10) {
                has_adjacent_symbol |= has_adjacent_symbol_above_or_below(&mtx, j, i);
                buffer.push(symbol);
            } else {
                flush(
                    has_adjacent_symbol || symbol != '.',
                    &mut buffer,
                    &mut result,
                );
                has_adjacent_symbol = symbol != '.';
            }
        }
        flush(has_adjacent_symbol, &mut buffer, &mut result);
    }
    Ok(result)
}

fn has_adjacent_symbol_above_or_below(mtx: &Vec<Vec<char>>, x: usize, y: usize) -> bool {
    let dxs = [-1, 0, 1];
    let dys = [-1, 1];
    let mut has = false;
    for dx in dxs {
        for dy in dys {
            let xx = x as i32 + dx;
            let yy = y as i32 + dy;
            if xx < 0 || yy < 0 {
                continue;
            }
            let xx = xx as usize;
            let yy = yy as usize;
            if yy >= mtx.len() || xx >= mtx[0].len() {
                continue;
            }
            has |= mtx[yy][xx] != '.';
        }
    }
    has
}

fn flush(has_adjacent_symbol: bool, buffer: &mut String, result: &mut u32) {
    if has_adjacent_symbol && !buffer.is_empty() {
        *result += buffer.parse::<u32>().unwrap();
    }
    buffer.clear();
}
