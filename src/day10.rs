use crate::grid::{Point, Vector};
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
    find_max_station(&asteroids).1
}

fn find_max_station(asteroids: &HashSet<Point>) -> (Point, usize) {
    let mut max_point = Point::origin();
    let mut max_detectable = std::usize::MIN;

    for curr in asteroids.iter() {
        let unique_angles = get_vectors(&asteroids, &curr)
            .iter()
            .map(|vector| vector.angle_deg)
            .collect::<HashSet<OrderedFloat<f64>>>();

        let num_detectable = unique_angles.len();

        if max_detectable < num_detectable {
            max_detectable = num_detectable;
            max_point = *curr;
        }
    }

    (max_point, max_detectable)
}

fn get_vectors(asteroids: &HashSet<Point>, current: &Point) -> Vec<Vector> {
    asteroids
        .iter()
        .filter(|asteroid| current != *asteroid)
        .map(|asteroid| Vector::from_points(&current, &asteroid))
        .collect::<Vec<Vector>>()
}

#[aoc(day10, part2)]
pub fn part2(asteroids: &HashSet<Point>) -> i32 {
    let (station, num_asteroids_detected) = find_max_station(&asteroids);
    let mut vectors = get_vectors(&asteroids, &station);
    vectors.sort_by(|a, b| {
        if a.angle_deg == b.angle_deg {
            // close to far
            return a.distance.cmp(&b.distance);
        }
        // high to low b/c we want to sweep clockwise
        return b.angle_deg.cmp(&a.angle_deg);
    });

    let (first_quadrant, mut others): (Vec<Vector>, Vec<Vector>) = vectors
        .iter()
        .partition(|&v| v.angle_deg <= OrderedFloat(90f64));

    let mut all_points = first_quadrant;
    all_points.append(&mut others);

    let mut destroyed_points: Vec<Point> = Vec::with_capacity(num_asteroids_detected);
    let mut angles_covered_this_loop = Vec::<OrderedFloat<f64>>::new();

    loop {
        angles_covered_this_loop.clear();
        for asteroid_vector in all_points.iter() {
            if !angles_covered_this_loop.contains(&asteroid_vector.angle_deg)
                && !destroyed_points.contains(&asteroid_vector.end_point)
            {
                destroyed_points.push(asteroid_vector.end_point);
                angles_covered_this_loop.push(asteroid_vector.angle_deg);
            }
        }

        if destroyed_points.len() >= 200 {
            break;
        }
    }

    let target_point = destroyed_points[199];

    target_point.x * 100 + target_point.y
}

#[cfg(test)]
mod asteroids_find_max_station {
    use super::*;
    use indoc::indoc;

    #[test]
    fn day10_example1() {
        assert_eq!(
            find_max_station(&get_occupied_points(&indoc!(
                ".#..#
                .....
                #####
                ....#
                ...##"
            ))),
            (Point { x: 3, y: 4 }, 8)
        );
    }

    #[test]
    fn day10_example_2() {
        assert_eq!(
            find_max_station(&get_occupied_points(&indoc!(
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
            (Point { x: 5, y: 8 }, 33)
        );
    }

    #[test]
    fn day10_example_3() {
        assert_eq!(
            find_max_station(&get_occupied_points(&indoc!(
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
            (Point { x: 1, y: 2 }, 35)
        );
    }

    #[test]
    fn day10_example_4() {
        assert_eq!(
            find_max_station(&get_occupied_points(&indoc!(
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
            (Point { x: 6, y: 3 }, 41)
        );
    }

    #[test]
    fn day10_example_5() {
        assert_eq!(
            find_max_station(&get_occupied_points(&indoc!(
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
            (Point { x: 11, y: 13 }, 210)
        );
    }
}
