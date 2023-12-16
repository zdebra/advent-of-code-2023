use std::env;
use std::fs::File;
use std::io::{self, BufRead};

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let file = File::open(filename).expect("open failed");

    let mut sum = 0;
    for line in io::BufReader::new(file).lines() {
        let line = line.unwrap();

        let mut first_digit = char::default();
        for (i, ch) in line.chars().enumerate() {
            if ch.is_numeric() {
                first_digit = ch;
                break;
            }
            let to_start = line[..i + 1].to_string();
            let digit = line_to_digit(&to_start);
            if digit.is_some() {
                first_digit = digit.unwrap();
                break;
            }
        }

        let mut last_digit = char::default();
        for (i, ch) in line.chars().rev().enumerate() {
            if ch.is_numeric() {
                last_digit = ch;
                break;
            }

            let to_end = line[line.len() - i - 1..].to_string();
            let digit = line_to_digit(&to_end);
            if digit.is_some() {
                last_digit = digit.unwrap();
                break;
            }
        }

        let concantenated = format!("{}{}", first_digit, last_digit);
        let number: u32 = concantenated.parse().unwrap();
        sum += number;
    }

    println!("{}", sum);
}

fn line_to_digit(line: &str) -> Option<char> {
    if line.contains("one") {
        Some('1')
    } else if line.contains("two") {
        Some('2')
    } else if line.contains("three") {
        Some('3')
    } else if line.contains("four") {
        Some('4')
    } else if line.contains("five") {
        Some('5')
    } else if line.contains("six") {
        Some('6')
    } else if line.contains("seven") {
        Some('7')
    } else if line.contains("eight") {
        Some('8')
    } else if line.contains("nine") {
        Some('9')
    } else if line.contains("zero") {
        Some('0')
    } else {
        None
    }
}
