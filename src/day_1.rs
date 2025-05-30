use std::{collections::HashMap, fs, iter::zip, path::Path};

use crate::Solution;

pub struct Day1;

impl Solution for Day1 {
    fn part1() -> u32 {
        let input = Path::new("input/day1.txt");
        let contents = fs::read_to_string(input).expect("Something wrong with opening this file");
        let (mut left, mut right): (Vec<u32>, Vec<u32>) = contents
            .lines()
            .map(|s| {
                let (first, second) = s.split_once("   ").unwrap();
                (
                    first.parse::<u32>().unwrap(),
                    second.parse::<u32>().unwrap(),
                )
            })
            .collect::<Vec<(u32, u32)>>()
            .into_iter()
            .unzip();

        left.sort();
        right.sort();

        let total_distance = zip(left, right)
            .map(|(first, second)| (first).abs_diff(second))
            .collect::<Vec<u32>>()
            .iter()
            .sum::<u32>();

        total_distance
    }

    fn part2() -> u32 {
        let input = Path::new("input/day1.txt");
        let contents = fs::read_to_string(input).expect("Something wrong with opening this file");
        let (left, right): (Vec<u32>, Vec<u32>) = contents
            .lines()
            .map(|s| {
                let (first, second) = s.split_once("   ").unwrap();
                (
                    first.parse::<u32>().unwrap(),
                    second.parse::<u32>().unwrap(),
                )
            })
            .collect::<Vec<(u32, u32)>>()
            .into_iter()
            .unzip();

        let mut freq_right: HashMap<u32, u32> = HashMap::new();

        right.iter().for_each(|item| {
            *freq_right.entry(*item).or_insert(0) += 1;
        });

        let similarity_score = left
            .iter()
            .filter(|left| right.contains(left))
            .map(|left| left * freq_right.get(left).unwrap_or(&0))
            .sum::<u32>();

        similarity_score
    }
}
