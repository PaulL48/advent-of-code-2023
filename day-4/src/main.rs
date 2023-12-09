use core::num;
use std::collections::HashSet;

use itertools::Itertools;

const INPUT: &str = include_str!("./input.txt");
const LEADING_NUMBERS: usize = 1;
const WINNING_NUMBERS_COUNT: usize = 10;
const WINNING_NUMBERS_PADDING: usize = 1;
const YOUR_NUMBERS_COUNT: usize = 25;


fn process_line(line: &str) -> u32 {
    let mut groups = line
        .chars()
        .group_by(|v| v.is_digit(10));

    let mut number_iter = groups
        .into_iter()
        .filter_map(|(k, g)| k.then(|| g.collect::<String>().parse::<u32>().unwrap()))
        .skip(LEADING_NUMBERS);

    // parse
    let mut matches = 0;
    let winning_numbers = number_iter.by_ref().take(WINNING_NUMBERS_COUNT).collect::<HashSet<_>>();
    for number in number_iter {
        if winning_numbers.contains(&number) {
            matches += 1;
        }
    }

    if matches == 0 {
        0
    } else {
        2_u32.pow(matches - 1)
    }
}

fn main() {
    let total_points: u32 = INPUT.lines().map(process_line).sum();
    println!("Total points: {}", total_points);
}
