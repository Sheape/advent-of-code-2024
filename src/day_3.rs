use std::{fs, path::Path};

use crate::Solution;
use fancy_regex::Regex;

pub struct Day3;

impl Solution for Day3 {
    fn part1() -> u32 {
        let input = Path::new("input/day3.txt");
        let contents = fs::read_to_string(input).expect("Something wrong with opening this file");

        find_mul(contents.as_str())
    }

    fn part2() -> u32 {
        let input = Path::new("input/day3.txt");
        let contents = fs::read_to_string(input).expect("Something wrong with opening this file");

        let dont_indices: Vec<usize> = contents
            .match_indices("don't()")
            .map(|result| result.0)
            .collect();

        let do_indices: Vec<usize> = contents
            .match_indices("do()")
            .map(|result| result.0)
            .collect();

        find_valid_indices(&do_indices, &dont_indices, contents.len())
            .iter()
            .map(|(do_index, dont_index)| find_mul(contents.get(*do_index..*dont_index).unwrap()))
            .sum()
    }
}

fn find_valid_indices(
    do_indices: &[usize],
    dont_indices: &[usize],
    length: usize,
) -> Vec<(usize, usize)> {
    let mut valid_indices: Vec<(usize, usize)> =
        vec![(0, dont_indices[0]), (*do_indices.last().unwrap(), length)];
    let mut last_dont_index: usize = 0;

    for i in do_indices {
        for j in dont_indices {
            if *i < *j && *i > last_dont_index {
                valid_indices.push((*i, *j));
                last_dont_index = *j;
                continue;
            }
        }
    }

    valid_indices
}

fn find_mul(input: &str) -> u32 {
    let re = Regex::new(r"(?<=mul\()\d{1,3},\d{1,3}(?=\))").unwrap();
    let mut result: u32 = 0;

    for regex_result in re.find_iter(input) {
        result += regex_result
            .unwrap()
            .as_str()
            .split(',')
            .map(|number| number.parse::<u32>().unwrap())
            .product::<u32>();
    }

    result
}
