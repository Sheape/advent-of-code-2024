mod day_1;
mod day_2;
mod day_3;
mod solution;

use day_1::Day1;
use day_2::Day2;
use day_3::Day3;

pub use solution::Solution;

fn main() {
    println!("DAY 1 [PART 1]: The total distance is: {}", Day1::part1());
    println!(
        "DAY 1 [PART 2]: The total similarity score is: {}",
        Day1::part2()
    );

    println!(
        "DAY 2 [PART 1]: The number of safe reports is: {}",
        Day2::part1()
    );

    println!(
        "DAY 3 [PART 1]: The result of multiplications is: {}",
        Day3::part1()
    );

    println!(
        "DAY 3 [PART 2]: The result of multiplications is: {}",
        Day3::part2()
    );
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_day_1_part_1() {
        assert_eq!(Day1::part1(), 2166959)
    }

    #[test]
    fn test_day_1_part_2() {
        assert_eq!(Day1::part2(), 23741109)
    }

    #[test]
    fn test_day_2_part_1() {
        assert_eq!(Day2::part1(), 314)
    }

    #[test]
    fn test_day_3_part_1() {
        assert_eq!(Day3::part1(), 188192787)
    }

    #[test]
    fn test_day_3_part_2() {
        assert_eq!(Day3::part2(), 113965544)
    }
}
