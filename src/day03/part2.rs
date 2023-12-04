use std::error::Error;

fn main() {
    let result = solve(include_str!("input")).unwrap();
    println!("{result}");
}

fn solve(input: &str) -> Result<u64, Box<dyn Error>> {
    let mut result = 0;
    let mtx: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    for i in 0..mtx.len() {
        for j in 0..mtx[i].len() {
            if mtx[i][j] == '*' {
                result += gear_ratio(&mtx, i, j);
            }
        }
    }
    Ok(result)
}

fn gear_ratio(mtx: &Vec<Vec<char>>, y: usize, x: usize) -> u64 {
    let upper = triple(mtx, y as isize - 1, x as isize);
    let lower = triple(mtx, y as isize + 1, x as isize);
    let mut result = combine(upper, lower);
    if x > 0 {
        result = combine(result, extract_number(mtx, y as isize, x as isize - 1));
    }
    if x + 1 < mtx[y].len() {
        result = combine(result, extract_number(mtx, y as isize, x as isize + 1));
    }
    if result.0 == 2 {
        result.1
    } else {
        0
    }
}

fn triple(mtx: &Vec<Vec<char>>, y: isize, x: isize) -> (u8, u64) {
    if y < 0 || y >= mtx.len() as isize {
        return (0, 1);
    }
    if mtx[y as usize][x as usize].is_digit(10) {
        extract_number(mtx, y, x)
    } else {
        let left = extract_number(mtx, y, x - 1);
        let right = extract_number(mtx, y, x + 1);
        combine(left, right)
    }
}

fn extract_number(mtx: &Vec<Vec<char>>, y: isize, x: isize) -> (u8, u64) {
    if x < 0 || x >= mtx[y as usize].len() as isize || !mtx[y as usize][x as usize].is_digit(10) {
        (0, 1)
    } else {
        let mut l = x - 1;
        let mut r = x + 1;
        while l >= 0 && mtx[y as usize][l as usize].is_digit(10) {
            l -= 1;
        }
        while r < mtx[y as usize].len() as isize && mtx[y as usize][r as usize].is_digit(10) {
            r += 1;
        }
        let mut number = String::new();
        for i in l + 1..r {
            number.push(mtx[y as usize][i as usize]);
        }
        (1, number.parse().unwrap())
    }
}

fn combine(first: (u8, u64), second: (u8, u64)) -> (u8, u64) {
    (first.0 + second.0, first.1 * second.1)
}
