use std::{fs, path::Path};

use crate::Solution;
pub struct Day4;

enum Position {
    TopEdge,
    LeftEdge,
    BottomEdge,
    RightEdge,
    TopLeftCorner,
    TopRightCorner,
    BottomLeftCorner,
    BottomRightCorner,
    Any,
}

enum Direction {
    Top,
    Left,
    Bottom,
    Right,
    TopLeft,
    TopRight,
    BottomLeft,
    BottomRight,
}

struct Cell<'a> {
    puzzle: &'a [&'a [u8]],
    position: Position,
    abs_position: (usize, usize),
}

impl Cell<'_> {
    fn traverse(&self) -> u32 {
        let directions: &[Direction] = match self.position {
            Position::TopLeftCorner => {
                &[Direction::BottomRight, Direction::Right, Direction::Bottom]
            }
            Position::TopRightCorner => {
                &[Direction::Left, Direction::BottomLeft, Direction::Bottom]
            }
            Position::BottomLeftCorner => &[Direction::Top, Direction::TopRight, Direction::Right],
            Position::BottomRightCorner => &[Direction::Top, Direction::TopLeft, Direction::Left],
            Position::TopEdge => &[
                Direction::Left,
                Direction::Right,
                Direction::Bottom,
                Direction::BottomLeft,
                Direction::BottomRight,
            ],
            Position::RightEdge => &[
                Direction::Top,
                Direction::Bottom,
                Direction::Left,
                Direction::TopLeft,
                Direction::BottomLeft,
            ],
            Position::BottomEdge => &[
                Direction::Top,
                Direction::Left,
                Direction::Right,
                Direction::TopLeft,
                Direction::TopRight,
            ],
            Position::LeftEdge => &[
                Direction::Top,
                Direction::Bottom,
                Direction::Right,
                Direction::TopRight,
                Direction::BottomRight,
            ],
            Position::Any => &[
                Direction::Top,
                Direction::Right,
                Direction::Bottom,
                Direction::Left,
                Direction::TopLeft,
                Direction::TopRight,
                Direction::BottomLeft,
                Direction::BottomRight,
            ],
        };

        directions
            .iter()
            .map(|direction| self.can_traverse(direction) as u32)
            .sum()
    }

    fn can_traverse(&self, direction: &Direction) -> bool {
        let (row, col) = self.abs_position;
        let puzzle = self.puzzle;
        let cur_row = puzzle[row];
        match direction {
            Direction::Right => {
                (cur_row[col + 1] == b'M')
                    && (cur_row[col + 2] == b'A')
                    && (cur_row[col + 3] == b'S')
            }
            Direction::Left => {
                (cur_row[col - 1] == b'M')
                    && (cur_row[col - 2] == b'A')
                    && (cur_row[col - 3] == b'S')
            }
            Direction::Top => {
                (puzzle[row - 1][col] == b'M')
                    && (puzzle[row - 2][col] == b'A')
                    && (puzzle[row - 3][col] == b'S')
            }
            Direction::Bottom => {
                (puzzle[row + 1][col] == b'M')
                    && (puzzle[row + 2][col] == b'A')
                    && (puzzle[row + 3][col] == b'S')
            }
            Direction::TopLeft => {
                (puzzle[row - 1][col - 1] == b'M')
                    && (puzzle[row - 2][col - 2] == b'A')
                    && (puzzle[row - 3][col - 3] == b'S')
            }
            Direction::TopRight => {
                (puzzle[row - 1][col + 1] == b'M')
                    && (puzzle[row - 2][col + 2] == b'A')
                    && (puzzle[row - 3][col + 3] == b'S')
            }
            Direction::BottomLeft => {
                (puzzle[row + 1][col - 1] == b'M')
                    && (puzzle[row + 2][col - 2] == b'A')
                    && (puzzle[row + 3][col - 3] == b'S')
            }
            Direction::BottomRight => {
                (puzzle[row + 1][col + 1] == b'M')
                    && (puzzle[row + 2][col + 2] == b'A')
                    && (puzzle[row + 3][col + 3] == b'S')
            }
        }
    }
}

impl Solution for Day4 {
    fn part1() -> u32 {
        let input = Path::new("input/day4.txt");
        let contents = fs::read_to_string(input).expect("Something wrong with opening this file");

        let mut valid_traversals: u32 = 0;

        let crossword = new_crossword(&contents);
        let (width, height) = (crossword[0].len(), crossword.len());

        for (i, row) in crossword.iter().enumerate() {
            for (j, letter) in row.iter().enumerate() {
                if *letter == b'X' {
                    let is_edge = ((i >= 3) && (height - i > 3)) || ((j >= 3) && (width - j > 3));
                    let is_left = j < 3;
                    let is_top = i < 3;
                    let is_right = width - j <= 3;
                    let is_bottom = height - i <= 3;

                    let is_abs_left = is_left && !is_right;
                    let is_abs_right = is_right && !is_left;
                    let is_abs_top = is_top && !is_bottom;
                    let is_abs_bottom = is_bottom && !is_top;

                    let position = if is_edge {
                        if is_abs_left {
                            Position::LeftEdge
                        } else if is_abs_top {
                            Position::TopEdge
                        } else if is_abs_right {
                            Position::RightEdge
                        } else if is_abs_bottom {
                            Position::BottomEdge
                        } else {
                            Position::Any
                        }
                    } else {
                        if is_abs_left && is_abs_top {
                            Position::TopLeftCorner
                        } else if is_abs_right && is_abs_top {
                            Position::TopRightCorner
                        } else if is_abs_left && is_abs_bottom {
                            Position::BottomLeftCorner
                        } else if is_abs_right && is_abs_bottom {
                            Position::BottomRightCorner
                        } else {
                            Position::Any
                        }
                    };

                    let cell = Cell {
                        puzzle: crossword.as_slice(),
                        position,
                        abs_position: (i, j),
                    };

                    valid_traversals += cell.traverse()
                }
            }
        }

        valid_traversals
    }

    fn part2() -> u32 {
        1u32
    }
}

fn new_crossword(puzzle: &str) -> Vec<&[u8]> {
    puzzle
        .lines()
        .map(|line| line.trim().as_bytes())
        .collect::<Vec<&[u8]>>()
}
