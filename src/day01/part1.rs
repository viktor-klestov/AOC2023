use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let input = include_str!("input");
    let mut result = 0;
    for line in input.lines() {
        let mut number = String::new();
        let mut last_digit = 'x';
        for symbol in line.chars() {
            let is_digit = symbol >= '0' && symbol <= '9';
            if is_digit {
                last_digit = symbol;
                if number.is_empty() {
                    number.push(symbol);
                }
            }
        }
        number.push(last_digit);
        result += number.parse::<u32>()?;
    }
    println!("{result}");
    Ok(())
}
