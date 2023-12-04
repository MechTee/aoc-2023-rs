use std::io::{BufRead, BufReader};
use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    let file = std::fs::File::open("input.txt").unwrap();
    let buf = BufReader::new(file);

    let lines = buf.lines();

    let no_word_sum = Arc::new(Mutex::new(0));
    let word_sum = Arc::new(Mutex::new(0));
    let mut handles = vec![];
    for line in lines {
        let no_word_sum = Arc::clone(&no_word_sum);
        let word_sum = Arc::clone(&word_sum);
        let handle = thread::spawn(move || {
            let line = line.unwrap();
            if let Some(no_words) = get_combined_digits(&line, get_first_and_last_digits_only) {
                let mut sum =  no_word_sum.lock().unwrap();
                *sum += no_words;
            }
            if let Some(words) = get_combined_digits(&line, get_first_and_last_with_words) {
                let mut sum =word_sum.lock().unwrap();
                *sum += words;
            }
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }
    println!("sum without is {}", *no_word_sum.lock().unwrap());
    println!("sum with is {}", *word_sum.lock().unwrap());
}

fn get_combined_digits<F>(line: &str, f: F) -> Option<i32>
where
    F: Fn(&str) -> Option<(i32, i32)>,
{
    let mut digits = String::new();
    let extracted_vals = f(line);
    if let Some((first, last)) = extracted_vals {
        digits.push(first.to_string().parse().ok()?);
        digits.push(last.to_string().parse().ok()?);
        return digits.parse::<i32>().ok();
    }
    None
}

fn get_first_and_last_digits_only(line: &str) -> Option<(i32, i32)> {
    Some((
        get_first_digit_and_index(line)?.1.to_digit(10)? as i32,
        get_last_digit_and_index(line)?.1.to_digit(10)? as i32,
    ))
}

fn get_first_and_last_with_words(line: &str) -> Option<(i32, i32)> {
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
    Some((
        get_first_digit_in_chars(line, numbers)?,
        get_last_digit_in_chars(line, numbers)?,
    ))
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

