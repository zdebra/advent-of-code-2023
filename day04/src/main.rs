use std::env;
use std::fs::File;
use std::io::{self, BufRead};

fn main() {
    let args: Vec<String> = env::args().collect();
    let file = File::open(&args[1]).expect("open failed");

    let mut counter = Vec::new();
    let mut cur_line = 0;
    let mut sum = 0;
    for line in io::BufReader::new(file).lines() {
        let line = line.unwrap().chars().collect::<String>();
        let winning_numbers = line.split(":").collect::<Vec<&str>>()[1]
            .split("|")
            .collect::<Vec<&str>>()[0]
            .split_whitespace()
            .map(|n| n.parse::<u32>().unwrap())
            .collect::<Vec<u32>>();

        let my_numbers = line.split(":").collect::<Vec<&str>>()[1]
            .split("|")
            .collect::<Vec<&str>>()[1]
            .split_whitespace()
            .map(|n| n.parse::<u32>().unwrap())
            .collect::<Vec<u32>>();

        let matching_numbers = my_numbers
            .iter()
            .filter(|n| winning_numbers.contains(n))
            .collect::<Vec<&u32>>();
        increase_counter(&mut counter, cur_line, cur_line, 1);

        if matching_numbers.len() > 0 {
            let number_of_copies = counter[cur_line];
            if number_of_copies > 0 {
                increase_counter(
                    &mut counter,
                    cur_line + 1,
                    cur_line + matching_numbers.len(),
                    number_of_copies,
                )
            }
        }

        if matching_numbers.len() > 0 {
            // 2^(n-1)
            let round_points = 2u32.pow((matching_numbers.len() - 1) as u32);
            sum += round_points;
        }
        cur_line += 1;
    }
    println!("sum: {}", sum);

    let total: u32 = counter.iter().take(cur_line).sum();
    println!("total: {}", total);
}

fn increase_counter(counter: &mut Vec<u32>, from: usize, to: usize, amount: u32) {
    if counter.len() < to + 1 {
        counter.resize(to + 1, 0);
    }
    for i in from..to + 1 {
        counter[i] += amount;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_increase_counter_within_bounds() {
        let mut counter = vec![1, 2, 3, 4, 5];
        increase_counter(&mut counter, 1, 3, 2);
        assert_eq!(counter, vec![1, 4, 5, 6, 5]);
    }

    #[test]
    fn test_increase_counter_out_of_bounds() {
        let mut counter = vec![1, 2, 3];
        increase_counter(&mut counter, 2, 4, 2);
        assert_eq!(counter, vec![1, 2, 5, 2, 2]);
    }

    #[test]
    fn test_increase_counter_with_zero_amount() {
        let mut counter = vec![1, 2, 3, 4, 5];
        increase_counter(&mut counter, 1, 3, 0);
        assert_eq!(counter, vec![1, 2, 3, 4, 5]);
    }

    #[test]
    fn test_increase_counter_empty() {
        let mut counter = vec![];
        increase_counter(&mut counter, 0, 2, 1);
        assert_eq!(counter, vec![1, 1, 1]);
    }

    #[test]
    fn test_increase_single() {
        let mut counter = vec![1, 2, 3];
        increase_counter(&mut counter, 0, 0, 1);
        assert_eq!(counter, vec![2, 2, 3]);
    }
}
