use aoc_runner_derive::{aoc, aoc_generator};
use std::collections::HashMap;
use std::num::ParseIntError;
use std::str::FromStr;

#[derive(PartialEq, Eq, Debug)]
enum Direction {
    UP,
    DOWN,
    LEFT,
    RIGHT,
}

impl FromStr for Direction {
    type Err = ();
    fn from_str(letter: &str) -> Result<Self, Self::Err> {
        match letter {
            "U" => Ok(Direction::UP),
            "D" => Ok(Direction::DOWN),
            "L" => Ok(Direction::LEFT),
            "R" => Ok(Direction::RIGHT),
            _ => Err(()),
        }
    }
}

#[derive(Eq, PartialEq, Debug)]
pub struct PathPart {
    direction: Direction,
    distance: i32,
}

pub struct WirePaths {
    a: Vec<PathPart>,
    b: Vec<PathPart>,
}

#[derive(Eq, PartialEq, Hash, Clone, Copy, Debug)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn origin() -> Self {
        Point { x: 0, y: 0 }
    }
}

#[derive(Debug)]
struct CrossedBy {
    a: bool,
    b: bool,
}

impl CrossedBy {
    fn new() -> Self {
        CrossedBy { a: false, b: false }
    }
}

#[aoc_generator(day3)]
pub fn get_wire_paths(input: &str) -> Result<WirePaths, ParseIntError> {
    let mut lines = input.lines();

    Ok(WirePaths {
        a: parse_line(lines.next().unwrap()),
        b: parse_line(lines.next().unwrap()),
    })
}

fn parse_line(line: &str) -> Vec<PathPart> {
    line.split(',')
        .map(|raw_part| {
            let (letter, number) = raw_part.split_at(1);
            PathPart {
                direction: Direction::from_str(&letter).unwrap(),
                distance: number.parse::<i32>().unwrap(),
            }
        })
        .collect()
}

#[aoc(day3, part1)]
pub fn part1(wire_paths: &WirePaths) -> i32 {
    let mut grid: HashMap<Point, CrossedBy> = HashMap::new();
    let mut intersections: Vec<Point> = Vec::new();

    for (index, path) in (vec![&wire_paths.a, &wire_paths.b]).iter().enumerate() {
        add_wire_path_to_grid(path, &Point::origin(), |&current_point| {
            let cross = grid.entry(current_point).or_insert(CrossedBy::new());
            if index == 0 {
                cross.a = true;
            } else {
                cross.b = true;
            }

            if cross.a && cross.b {
                intersections.push(current_point.clone());
            }
        });
    }

    find_closest_intersection_distance(&intersections)
}

fn add_wire_path_to_grid<F>(wire_path: &Vec<PathPart>, start_point: &Point, mut point_callback: F)
where
    F: FnMut(&Point) -> (),
{
    let mut last_point = start_point.clone();

    for part in wire_path {
        let new_point = match part.direction {
            Direction::UP => {
                for y_val in last_point.y..=last_point.y + part.distance {
                    let current_point = Point {
                        x: last_point.x,
                        y: y_val,
                    };

                    point_callback(&current_point);
                }

                Point {
                    x: last_point.x,
                    y: last_point.y + part.distance,
                }
            }
            Direction::DOWN => {
                for y_val in last_point.y - part.distance..=last_point.y {
                    let current_point = Point {
                        x: last_point.x,
                        y: y_val,
                    };

                    point_callback(&current_point);
                }

                Point {
                    x: last_point.x,
                    y: last_point.y - part.distance,
                }
            }
            Direction::LEFT => {
                for x_val in last_point.x - part.distance..=last_point.x {
                    let current_point = Point {
                        x: x_val,
                        y: last_point.y,
                    };

                    point_callback(&current_point);
                }

                Point {
                    x: last_point.x - part.distance,
                    y: last_point.y,
                }
            }
            Direction::RIGHT => {
                for x_val in last_point.x..=last_point.x + part.distance {
                    let current_point = Point {
                        x: x_val,
                        y: last_point.y,
                    };

                    point_callback(&current_point);
                }

                Point {
                    x: last_point.x + part.distance,
                    y: last_point.y,
                }
            }
        };

        last_point = new_point.clone();
    }
}

fn find_closest_intersection_distance(intersections: &Vec<Point>) -> i32 {
    let mut minimum = std::i32::MAX;

    for current in intersections {
        let current_cross_distance = current.x.abs() + current.y.abs();

        // exclude the origin
        if current.x == 0 && current.y == 0 {
            continue;
        }

        if current_cross_distance < minimum {
            minimum = current_cross_distance;
        }
    }

    minimum
}

// #[aoc(day3, part2)]
// pub fn part2(program_input: &([PathPart], [PathPart])) -> usize {
//     0
// }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_line_test() {
        assert_eq!(
            parse_line(&"R83,U83,L12,D49,R71"),
            vec![
                PathPart {
                    direction: Direction::RIGHT,
                    distance: 83
                },
                PathPart {
                    direction: Direction::UP,
                    distance: 83
                },
                PathPart {
                    direction: Direction::LEFT,
                    distance: 12
                },
                PathPart {
                    direction: Direction::DOWN,
                    distance: 49
                },
                PathPart {
                    direction: Direction::RIGHT,
                    distance: 71
                },
            ]
        )
    }

    #[test]
    fn closest_intersection_distance_test() {
        assert_eq!(
            find_closest_intersection_distance(&vec![Point { x: 1, y: 1 }]),
            2
        );
        assert_eq!(
            find_closest_intersection_distance(&vec![Point { x: 123, y: 1 }, Point { x: 1, y: 1 }]),
            2
        );
        assert_eq!(
            find_closest_intersection_distance(&vec![
                Point { x: 123, y: 1 },
                Point { x: -1, y: -1 }
            ]),
            2
        );
        assert_eq!(
            find_closest_intersection_distance(&vec![
                Point { x: 123, y: 1 },
                Point { x: 1, y: -1 }
            ]),
            2
        );
    }

    #[test]
    fn part1_example1() {
        assert_eq!(
            part1(&WirePaths {
                a: parse_line(&"R75,D30,R83,U83,L12,D49,R71,U7,L72"),
                b: parse_line(&"U62,R66,U55,R34,D71,R55,D58,R83"),
            }),
            159
        );
    }

    #[test]
    fn part1_example2() {
        assert_eq!(
            part1(&WirePaths {
                a: parse_line(&"R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51"),
                b: parse_line(&"U98,R91,D20,R16,D67,R40,U7,R15,U6,R7"),
            }),
            135
        );
    }
}
