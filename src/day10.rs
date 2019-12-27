use crate::grid::Point;
use aoc_runner_derive::{aoc, aoc_generator};
use ordered_float::OrderedFloat;
use std::collections::HashSet;

#[aoc_generator(day10)]
pub fn get_occupied_points(input: &str) -> HashSet<Point> {
    let mut result = HashSet::new();
    for (y_index, line) in input.lines().enumerate() {
        for (x_index, curr) in line.chars().enumerate() {
            match curr {
                '#' => {
                    result.insert(Point {
                        x: x_index as i32,
                        y: y_index as i32,
                    });
                }
                '.' => {}
                _ => panic!("Bad input"),
            }
        }
    }

    result
}

#[aoc(day10, part1)]
pub fn part1(asteroids: &HashSet<Point>) -> usize {
    let mut max_point = Point::origin();
    let mut max_detectable = std::usize::MIN;

    for curr in asteroids.iter() {
        let unique_angles = asteroids
            .iter()
            .filter(|asteroid| curr != *asteroid)
            .map(|asteroid| Point::angle_between(&curr, &asteroid))
            .collect::<HashSet<OrderedFloat<f64>>>();

        let num_detectable = unique_angles.len();

        if max_detectable < num_detectable {
            max_detectable = num_detectable;
            max_point = *curr;
        }
    }

    max_detectable
}

// #[aoc(day10, part2)]
// pub fn part2(program: &[i64]) -> i64 {
//     let outputs = run_program(&program, &[2]);
//     *outputs.last().unwrap()
// }

#[cfg(test)]
mod asteroids_part1 {
    use super::*;
    use indoc::indoc;

    #[test]
    fn day10_example1() {
        assert_eq!(
            part1(&get_occupied_points(&indoc!(
                ".#..#
                .....
                #####
                ....#
                ...##"
            ))),
            8
        );
    }

    #[test]
    fn day10_example_2() {
        assert_eq!(
            part1(&get_occupied_points(&indoc!(
                "......#.#.
                #..#.#....
                ..#######.
                .#.#.###..
                .#..#.....
                ..#....#.#
                #..#....#.
                .##.#..###
                ##...#..#.
                .#....####"
            ))),
            33
        );
    }

    #[test]
    fn day10_example_3() {
        assert_eq!(
            part1(&get_occupied_points(&indoc!(
                "#.#...#.#.
                .###....#.
                .#....#...
                ##.#.#.#.#
                ....#.#.#.
                .##..###.#
                ..#...##..
                ..##....##
                ......#...
                .####.###."
            ))),
            35
        );
    }

    #[test]
    fn day10_example_4() {
        assert_eq!(
            part1(&get_occupied_points(&indoc!(
                ".#..#..###
                ####.###.#
                ....###.#.
                ..###.##.#
                ##.##.#.#.
                ....###..#
                ..#.#..#.#
                #..#.#.###
                .##...##.#
                .....#.#.."
            ))),
            41
        );
    }

    #[test]
    fn day10_example_5() {
        assert_eq!(
            part1(&get_occupied_points(&indoc!(
                ".#..##.###...#######
                ##.############..##.
                .#.######.########.#
                .###.#######.####.#.
                #####.##.#.##.###.##
                ..#####..#.#########
                ####################
                #.####....###.#.#.##
                ##.#################
                #####.##.###..####..
                ..######..##.#######
                ####.##.####...##..#
                .#####..#.######.###
                ##...#.##########...
                #.##########.#######
                .####.#.###.###.#.##
                ....##.##.###..#####
                .#.#.###########.###
                #.#.#.#####.####.###
                ###.##.####.##.#..##"
            ))),
            210
        );
    }
}
