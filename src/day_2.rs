use std::{fs, path::Path};

use crate::Solution;

pub struct Day2;

fn check_line(line: &[u8]) -> bool {
    if line.len() < 2 {
        return true;
    }

    if line[0] == line[1] {
        return false;
    }

    let is_increasing = line[0] < line[1];

    for window in line.windows(2) {
        let [first, second] = *window else {
            return false; // This should never happen, but just in case
        };

        if (first < second) != is_increasing || first == second || first.abs_diff(second) > 3 {
            return false;
        }
    }

    true
}

impl Solution for Day2 {
    fn part1() -> u32 {
        let input = Path::new("input/day2.txt");
        let contents = fs::read_to_string(input).expect("Something wrong with opening this file");
        let safe_reports = contents
            .lines()
            .map(|line| {
                line.split_ascii_whitespace()
                    .map(|number| number.parse::<u8>().unwrap())
                    .collect::<Vec<u8>>()
            })
            .filter(|line| check_line(line))
            .count();

        safe_reports.try_into().unwrap()
    }

    fn part2() -> u32 {
        let input = Path::new("input/day2.txt");
        let contents = fs::read_to_string(input).expect("Something wrong with opening this file");
        let safe_reports = contents
            .lines()
            .map(|line| {
                line.split_ascii_whitespace()
                    .map(|number| number.parse::<u8>().unwrap())
                    .collect::<Vec<u8>>()
            })
            .filter(|line| check_line(line))
            .count();

        safe_reports.try_into().unwrap()
    }
}
