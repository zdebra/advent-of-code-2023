use std::env;
use std::fs::File;
use std::io::{self, BufRead};

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let file = File::open(filename).expect("open failed");

    let mut sum_round_one = 0;
    let mut sum_round_two = 0;
    for line in io::BufReader::new(file).lines() {
        let line = line.unwrap();
        //Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
        let line = line.trim_start_matches("Game ");
        let game_id = line.split(":").next().unwrap().parse::<u32>().unwrap();
        let line = line.trim_start_matches(&format!("{}: ", game_id));
        let subsets = line.split("; ").collect::<Vec<&str>>(); // "1 blue, 2 green", "3 green, 4 blue, 1 red"

        let mut max_green = 0;
        let mut max_blue = 0;
        let mut max_red = 0;
        subsets.iter().for_each(|subset| {
            let cubes = subset.split(", ").collect::<Vec<&str>>(); // "1 blue", "2 green"
            cubes.iter().for_each(|cube| {
                let cube = cube.split(" ").collect::<Vec<&str>>(); // "1", "blue"
                let count = cube[0].parse::<u32>().unwrap();
                let color = cube[1];
                match color {
                    "green" => max_green = max_green.max(count),
                    "blue" => max_blue = max_blue.max(count),
                    "red" => max_red = max_red.max(count),
                    _ => panic!("unknown color"),
                }
            })
        });

        let goal_red = 12;
        let goal_green = 13;
        let goal_blue = 14;

        if max_red <= goal_red && max_green <= goal_green && max_blue <= goal_blue {
            sum_round_one += game_id;
        }

        let game_multiplicated = max_red * max_green * max_blue;
        sum_round_two += game_multiplicated;
    }
    println!("sum_round_one: {}", sum_round_one);
    println!("sum_round_two: {}", sum_round_two);
}
