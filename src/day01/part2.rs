use std::collections::hash_map::HashMap;
use std::error::Error;

fn build_digits() -> HashMap<String, u8> {
    let mut digits = HashMap::new();
    for i in 0..10 {
        digits.insert(i.to_string(), i);
    }
    digits.insert("one".to_string(), 1);
    digits.insert("two".to_string(), 2);
    digits.insert("three".to_string(), 3);
    digits.insert("four".to_string(), 4);
    digits.insert("five".to_string(), 5);
    digits.insert("six".to_string(), 6);
    digits.insert("seven".to_string(), 7);
    digits.insert("eight".to_string(), 8);
    digits.insert("nine".to_string(), 9);
    digits
}

fn main() -> Result<(), Box<dyn Error>> {
    let input = include_str!("input");
    let mut result: u32 = 0;
    let digits = build_digits();
    for line in input.lines() {
        let mut first_digit = 0;
        let mut last_digit = 0;
        let line_size = line.len();
        for i in 0..line_size {
            for word in digits.keys() {
                if i + word.len() <= line_size && &line[i..i + word.len()] == word {
                    last_digit = digits[word];
                    if first_digit == 0 {
                        first_digit = last_digit;
                    }
                }
            }
        }
        result += (10 * first_digit + last_digit) as u32;
    }
    println!("{result}");
    Ok(())
}
