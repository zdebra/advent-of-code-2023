use std::collections::HashMap;
use std::collections::LinkedList;
use std::env;
use std::fs::File;
use std::io::{self, BufRead};

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let file = File::open(filename).expect("open failed");

    let mut number_of_rows = 0;

    let mut numbers = HashMap::new();
    let mut symbols = Vec::new();

    for line in io::BufReader::new(file).lines() {
        let line = line.unwrap().chars().collect::<Vec<char>>();

        let mut i = 0;
        while i < line.len() {
            let c = line[i];
            if c == '.' {
                i += 1;
                continue;
            }
            if c.is_numeric() {
                if let Some(num) = number_at((number_of_rows, i), &line) {
                    let new_i = num.end_at;
                    for j in num.start_at..num.end_at + 1 {
                        numbers.insert((number_of_rows, j), num.clone());
                    }
                    i = new_i;
                }
            } else {
                symbols.push(Symbol {
                    row: number_of_rows,
                    col: i,
                    value: c,
                });
            }
            i += 1;
        }

        number_of_rows += 1;
    }

    let mut final_sum = 0;
    let mut round_two_sum = 0;
    symbols.iter().for_each(|s| {
        let mut adjacent_numbers = Vec::new();

        if let Some(top_left_number) = numbers.get(&(s.row - 1, s.col - 1)) {
            adjacent_numbers.push(top_left_number);
            final_sum += top_left_number.value;
        }
        if let Some(top_number) = numbers.get(&(s.row - 1, s.col)) {
            if top_number.start_at == s.col {
                adjacent_numbers.push(top_number);
                final_sum += top_number.value;
            }
        }
        if let Some(top_right_number) = numbers.get(&(s.row - 1, s.col + 1)) {
            if top_right_number.start_at == s.col + 1 {
                adjacent_numbers.push(top_right_number);
                final_sum += top_right_number.value;
            }
        }
        if let Some(left_number) = numbers.get(&(s.row, s.col - 1)) {
            adjacent_numbers.push(left_number);
            final_sum += left_number.value;
        }
        if let Some(right_number) = numbers.get(&(s.row, s.col + 1)) {
            adjacent_numbers.push(right_number);
            final_sum += right_number.value;
        }
        if let Some(bottom_left_number) = numbers.get(&(s.row + 1, s.col - 1)) {
            adjacent_numbers.push(bottom_left_number);
            final_sum += bottom_left_number.value;
        }
        if let Some(bottom_number) = numbers.get(&(s.row + 1, s.col)) {
            if bottom_number.start_at == s.col {
                adjacent_numbers.push(bottom_number);
                final_sum += bottom_number.value;
            }
        }
        if let Some(bottom_right_number) = numbers.get(&(s.row + 1, s.col + 1)) {
            if bottom_right_number.start_at == s.col + 1 {
                adjacent_numbers.push(bottom_right_number);
                final_sum += bottom_right_number.value;
            }
        }

        if adjacent_numbers.len() == 2 && s.value == '*' {
            let gear_ratio = adjacent_numbers[0].value * adjacent_numbers[1].value;
            round_two_sum += gear_ratio;
        }
    });
    println!("final sum: {}", final_sum);
    println!("round two sum: {}", round_two_sum);
}

fn number_at((x, y): (usize, usize), row: &Vec<char>) -> Option<Number> {
    if !row[y].is_numeric() {
        return None;
    }

    let mut number_chars = LinkedList::new();
    let mut start_at = y;
    let mut end_at = y;

    for i in y..row.len() {
        end_at = i;
        if row[i].is_numeric() {
            number_chars.push_back(row[i]);
        } else {
            end_at -= 1;
            break;
        }
    }
    for i in (0..y).rev() {
        start_at = i;
        if row[i].is_numeric() {
            number_chars.push_front(row[i]);
        } else {
            start_at += 1;
            break;
        }
    }

    let value = number_chars
        .iter()
        .collect::<String>()
        .parse::<u32>()
        .unwrap();

    Some(Number {
        row: x,
        start_at,
        end_at,
        value,
    })
}

#[derive(Clone, PartialEq, Eq, Debug)]
struct Number {
    row: usize,
    start_at: usize,
    end_at: usize,
    value: u32,
}

#[derive(Clone)]
struct Symbol {
    row: usize,
    col: usize,
    value: char,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_number_at_numeric() {
        let row = vec!['1', '2', '3', '4', '5'];
        let result = number_at((0, 0), &row);
        assert_eq!(
            result,
            Some(Number {
                row: 0,
                start_at: 0,
                end_at: 4,
                value: 12345
            })
        );
    }

    #[test]
    fn test_number_at_non_numeric() {
        let row = vec!['.', '*', '#', '+', '$'];
        let result = number_at((0, 2), &row);
        assert_eq!(result, None);
    }

    #[test]
    fn test_number_at_part_of_number() {
        let row = vec!['1', '2', '3', '.', '4', '5'];
        assert_eq!(
            number_at((0, 1), &row),
            Some(Number {
                row: 0,
                start_at: 0,
                end_at: 2,
                value: 123
            })
        );
        assert_eq!(
            number_at((0, 2), &row),
            Some(Number {
                row: 0,
                start_at: 0,
                end_at: 2,
                value: 123
            })
        );
        assert_eq!(
            number_at((0, 5), &row),
            Some(Number {
                row: 0,
                start_at: 4,
                end_at: 5,
                value: 45
            })
        );
    }
}
