mod day_1;
mod solution;

use day_1::Day1;
pub use solution::Solution;

fn main() {
    println!("DAY 1 [PART 1]: The total distance is: {}", Day1::part1());
    println!(
        "DAY 1 [PART 2]: The total similarity score is: {}",
        Day1::part2()
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
}
