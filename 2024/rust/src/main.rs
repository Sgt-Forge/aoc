mod day01;
mod day02;
mod day03;
mod day04;
mod day05;

use std::fs::File;
use std::io::{self, prelude::*, BufReader};

use day01::day01::day01_both;
use day02::day02::day02_both;
use day03::day03::{day03_part1, day03_part2};
use day04::day04::{day04_part1, day04_part2};
use day05::day05::{day05_part1, day05_part2};

fn get_days_input(day: &str) -> io::Result<io::Lines<BufReader<File>>> {
    let filepath = format!("../../../problems/day{}/input.txt", day);
    let file = File::open(filepath)?;
    Ok(BufReader::new(file).lines())
}

fn parse_text_to_numbers(text: io::Lines<BufReader<File>>) -> Vec<Vec<i32>> {
    let numbers: Vec<Vec<i32>> = text
        .map(|line| {
            line.unwrap()
                .split_whitespace()
                .map(|num_str| num_str.parse().unwrap())
                .collect()
        })
        .collect();

    numbers
}

fn main() {
    _ = day01_both();
    _ = day02_both();
    day03_part1();
    day03_part2();
    day04_part1();
    day04_part2();
    day05_part1();
    day05_part2();
}
