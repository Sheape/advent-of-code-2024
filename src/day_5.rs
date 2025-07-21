use std::{
    collections::{HashMap, hash_map::Entry::Vacant},
    fs::read_to_string,
    path::Path,
};

use crate::Solution;

pub struct Day5;

impl Solution for Day5 {
    fn part1() -> u32 {
        let input = Path::new("input/day5.txt");
        let contents = read_to_string(input).expect("Something wrong with opening this file");
        let Some((first_section, second_section)) = contents.split_once("\n\n") else {
            panic!("Invalid file input. Requires the exact format for section 1 and 2");
        };

        let mut map: HashMap<u8, Vec<u8>> = HashMap::new();

        first_section.split_whitespace().for_each(|line| {
            let Some((before, after)) = line.split_once('|') else {
                panic!("Invalid line found.");
            };

            let key = before.parse::<u8>().unwrap();
            let value = after.parse::<u8>().unwrap();

            if let Vacant(e) = map.entry(key) {
                e.insert(vec![value]);
            } else {
                map.get_mut(&key).unwrap().push(value);
            }
        });

        second_section
            .split_whitespace()
            .map(|line| {
                let update = line
                    .split(',')
                    .map(|number| number.parse::<u8>().unwrap())
                    .collect::<Vec<u8>>();
                let mut valid_update = true;

                for (i, page) in update.iter().enumerate() {
                    let pages = map.get_mut(page);
                    if let Some(page_exist) = pages {
                        for next_upd_idx in i + 1..update.len() {
                            if !page_exist.contains(&update[next_upd_idx]) {
                                valid_update = false;
                                break;
                            }
                        }
                        for prev_upd_idx in 0..i {
                            if page_exist.contains(&update[prev_upd_idx]) {
                                valid_update = false;
                                break;
                            }
                        }
                    }
                }

                if valid_update {
                    update[update.len().div_ceil(2) - 1] as u32
                } else {
                    0
                }
            })
            .sum::<u32>()
    }

    fn part2() -> u32 {
        1u32
    }
}
