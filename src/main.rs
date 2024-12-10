use std::fs;

fn main() {
    const INPUT_FILE: &str = "input/day9-test.txt";
    let contents = fs::read_to_string(INPUT_FILE).expect("Something went wrong reading the file");
    let mut i: u16 = 1;
    for c in contents.chars() {
        match i % 2 {
            0 => println!("{}"),
        }
        i += 1;
    }

    println!("Hello, world!");
}
