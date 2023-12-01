use std::io::{BufRead, BufReader};

fn main() {
    let file = std::fs::File::open("input.txt").unwrap();
    let buf = BufReader::new(file);
    let sum = buf
        .lines()
        .filter_map(|l| {
            let line = l.unwrap();
            let val = get_combined_digits(line.chars());
            println!("value for line {} is {}", line, val.unwrap_or(0));
            val
        })
        .sum::<isize>();
    println!("sum is {}", sum);
}

fn get_combined_digits(chars: std::str::Chars) -> Option<isize> {
    let mut digits = String::new();
    digits.push(get_first_digit_in_chars(chars.clone())?);
    digits.push(get_last_digit_in_chars(chars.clone())?);
    if !digits.is_empty() {
        Some(digits.parse().ok()?)
    } else {
        None
    }
}

fn get_first_digit_in_chars(chars: std::str::Chars) -> Option<char> {
    chars.into_iter().find(|&c| c.is_ascii_digit())
}

fn get_last_digit_in_chars(chars: std::str::Chars) -> Option<char> {
    chars.into_iter().rev().find(|&c| c.is_ascii_digit())
}
