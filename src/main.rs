use std::io::{BufRead, BufReader};

fn main() {
    let file = std::fs::File::open("input.txt").unwrap();
    let buf = BufReader::new(file);
    let (sum_without, sum_with) = buf
        .lines()
        .filter_map(|l| {
            let line = l.unwrap();
            let no_words = get_combined_digits(&line, false);
            let words = get_combined_digits(&line, true);
            println!(
                "NOWORDS value for line {} is {}",
                line,
                no_words.unwrap_or(0)
            );
            println!("value for line {} is {}", line, words.unwrap_or(0));
            Some((no_words?, words?))
        })
        .fold((0, 0), |(sum_without, sum_with), (no_words, words)| {
            (sum_without + no_words, sum_with + words)
        });
    println!("sum without is {}", sum_without);
    println!("sum with is {}", sum_with);
}

fn get_combined_digits(
    line: &str,
    respect_words: bool
) -> Option<i32> {
    let mut digits = String::new();

    if respect_words {
        let numbers = [
            ("one", 1),
            ("two", 2),
            ("three", 3),
            ("four", 4),
            ("five", 5),
            ("six", 6),
            ("seven", 7),
            ("eight", 8),
            ("nine", 9),
        ];
        digits.push(
            get_first_digit_in_chars(line, numbers)?
                .to_string()
                .parse()
                .ok()?,
        );
        digits.push(
            get_last_digit_in_chars(line, numbers)?
                .to_string()
                .parse()
                .ok()?,
        );
    } else {
        digits.push(
            get_first_digit_and_index(line)?
                .1
                .to_string()
                .parse()
                .ok()?,
        );
        digits.push(get_last_digit_and_index(line)?.1.to_string().parse().ok()?);
    }

    if !digits.is_empty() {
        Some(digits.parse().ok()?)
    } else {
        None
    }
}

fn get_first_digit_in_chars(line: &str, numbers: [(&str, i32); 9]) -> Option<i32> {
    let index_and_digit = get_first_digit_and_index(line)?;
    let lowest_index = numbers
        .iter()
        .filter_map(|(n, _)| line.match_indices(*n).min_by_key(|(i, _)| *i))
        .min_by_key(|(i, _)| *i);
    if let Some(lowest_index) = lowest_index {
        if index_and_digit.0 < lowest_index.0 {
            return Some(index_and_digit.1.to_digit(10)? as i32);
        }
        return Some(numbers.iter().find(|(word, _)| lowest_index.1 == *word)?.1);
    }
    Some(index_and_digit.1.to_digit(10)? as i32)
}

fn get_first_digit_and_index(line: &str) -> Option<(usize, char)> {
    line.chars().enumerate().find(|(_, c)| c.is_ascii_digit())
}

fn get_last_digit_and_index(line: &str) -> Option<(usize, char)> {
    let digit = line.chars().rev().find(|c| c.is_ascii_digit())?;
    let index_of_digit = line.chars().position(|c| c == digit)?;
    Some((index_of_digit, digit))
}

fn get_last_digit_in_chars(line: &str, numbers: [(&str, i32); 9]) -> Option<i32> {
    let (index_of_digit, digit) = get_last_digit_and_index(line)?;
    let highest_index = numbers
        .iter()
        .filter_map(|(n, _)| line.match_indices(*n).max_by_key(|(i, _)| *i))
        .max_by_key(|(i, _)| *i);
    if let Some(highest_index) = highest_index {
        if index_of_digit > highest_index.0 {
            return Some(digit.to_digit(10)? as i32);
        }
        return Some(numbers.iter().find(|(word, _)| highest_index.1 == *word)?.1);
    }
    Some(digit.to_digit(10)? as i32)
}
