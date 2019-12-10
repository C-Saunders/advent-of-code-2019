use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;
use std::collections::HashMap;

#[derive(Debug)]
pub struct IntRange {
    bottom: u32,
    top: u32,
}

#[aoc_generator(day4)]
pub fn parse_data(input: &str) -> IntRange {
    let split: Vec<u32> = input
        .split('-')
        .map(|item| item.parse::<u32>().unwrap())
        .collect();
    IntRange {
        bottom: split[0],
        top: split[1],
    }
}

#[aoc(day4, part1)]
pub fn part1(range: &IntRange) -> u32 {
    let mut counter = 0;
    for number in range.bottom..=range.top {
        if is_valid_solution_for_part1(number) {
            counter = counter + 1;
        }
    }

    counter
}

#[aoc(day4, part2)]
pub fn part2(range: &IntRange) -> u32 {
    let mut counter = 0;
    for number in range.bottom..=range.top {
        if is_valid_solution_for_part2(number) {
            counter = counter + 1;
        }
    }

    counter
}

fn is_valid_solution_for_part1(number: u32) -> bool {
    let as_str = number.to_string();

    let mut double_found = false;

    for (left, right) in as_str.chars().tuple_windows() {
        let left_num = left.to_digit(10).unwrap();
        let right_num = right.to_digit(10).unwrap();

        if left_num > right_num {
            return false;
        }

        if left_num == right_num {
            double_found = true;
        }
    }

    double_found
}

fn is_valid_solution_for_part2(number: u32) -> bool {
    if !is_valid_solution_for_part1(number) {
        return false;
    }
    let as_str = number.to_string();
    let mut occurrences: HashMap<char, u32> = HashMap::new();

    for char in as_str.chars() {
        let instance = occurrences.entry(char).or_insert(0);
        *instance = *instance + 1;
    }

    for count in occurrences.values() {
        if *count == 2 {
            return true;
        }
    }

    false
}

#[cfg(test)]
mod part1_tests {
    use super::is_valid_solution_for_part1;

    #[test]
    fn invalid_solution_due_to_decreases() {
        assert_eq!(is_valid_solution_for_part1(12343), false);
    }

    #[test]
    fn invalid_solution_due_no_double() {
        assert_eq!(is_valid_solution_for_part1(12345), false);
    }

    #[test]
    fn valid_solutions() {
        assert_eq!(is_valid_solution_for_part1(11), true);
        assert_eq!(is_valid_solution_for_part1(11234), true);
        assert_eq!(is_valid_solution_for_part1(12334), true);
        assert_eq!(is_valid_solution_for_part1(12344), true);
        assert_eq!(is_valid_solution_for_part1(123444), true);
    }
}

#[cfg(test)]
mod part2_tests {
    use super::is_valid_solution_for_part2;

    #[test]
    fn invalid_solution_due_to_decreases() {
        assert_eq!(is_valid_solution_for_part2(12343), false);
    }

    #[test]
    fn invalid_solution_due_no_double() {
        assert_eq!(is_valid_solution_for_part2(12345), false);
    }

    #[test]
    fn invalid_solution_due_to_invalid_double() {
        assert_eq!(is_valid_solution_for_part2(11123), false);
    }

    #[test]
    fn valid_solution_with_invalid_double() {
        assert_eq!(is_valid_solution_for_part2(111122), true);
    }

    #[test]
    fn valid_solutions() {
        assert_eq!(is_valid_solution_for_part2(11), true);
        assert_eq!(is_valid_solution_for_part2(11234), true);
        assert_eq!(is_valid_solution_for_part2(12334), true);
        assert_eq!(is_valid_solution_for_part2(12344), true);
    }
}
