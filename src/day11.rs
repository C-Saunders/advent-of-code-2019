use crate::grid::Point;
use crate::intcode_computer::{IntcodeComputer, ProgramOutput};
use aoc_runner_derive::{aoc, aoc_generator};
use std::collections::{HashMap, HashSet};
use std::num::ParseIntError;

enum Color {
    Black,
    White,
}

impl Color {
    fn to_computer_input(&self) -> i64 {
        match self {
            Color::Black => 0,
            Color::White => 1,
        }
    }
}

#[derive(PartialEq, Eq, Debug)]
enum FacingDirection {
    Up,
    Down,
    Left,
    Right,
}

enum TurnDirection {
    Left,
    Right,
}

impl From<i64> for TurnDirection {
    fn from(direction: i64) -> Self {
        match direction {
            0 => TurnDirection::Left,
            1 => TurnDirection::Right,
            _ => panic!("Bad turn direction"),
        }
    }
}

struct Robot {
    location: Point,
    facing: FacingDirection,
}

impl Robot {
    fn new() -> Self {
        Robot {
            location: Point::origin(),
            facing: FacingDirection::Up,
        }
    }

    fn turn(&mut self, turn: TurnDirection) {
        self.facing = match self.facing {
            FacingDirection::Up => match turn {
                TurnDirection::Left => FacingDirection::Left,
                TurnDirection::Right => FacingDirection::Right,
            },
            FacingDirection::Down => match turn {
                TurnDirection::Left => FacingDirection::Right,
                TurnDirection::Right => FacingDirection::Left,
            },
            FacingDirection::Left => match turn {
                TurnDirection::Left => FacingDirection::Down,
                TurnDirection::Right => FacingDirection::Up,
            },
            FacingDirection::Right => match turn {
                TurnDirection::Left => FacingDirection::Up,
                TurnDirection::Right => FacingDirection::Down,
            },
        }
    }

    fn walk(&mut self) {
        let current_location = self.location;

        match self.facing {
            FacingDirection::Up => {
                self.location = Point {
                    x: current_location.x,
                    y: current_location.y + 1,
                }
            }
            FacingDirection::Down => {
                self.location = Point {
                    x: current_location.x,
                    y: current_location.y - 1,
                }
            }
            FacingDirection::Left => {
                self.location = Point {
                    x: current_location.x - 1,
                    y: current_location.y,
                }
            }
            FacingDirection::Right => {
                self.location = Point {
                    x: current_location.x + 1,
                    y: current_location.y,
                }
            }
        };
    }
}

#[aoc_generator(day11)]
pub fn get_program(input: &str) -> Result<Vec<i64>, ParseIntError> {
    input.split(',').map(|l| l.parse::<i64>()).collect()
}

#[aoc(day11, part1)]
pub fn part1(program_input: &[i64]) -> usize {
    let mut grid = HashMap::<Point, Color>::new();
    let mut painted_points = HashSet::<Point>::new();
    let mut robot = Robot::new();
    let mut computer = IntcodeComputer::yielding_computer(&program_input);

    loop {
        let robot_location = grid.entry(robot.location).or_insert(Color::Black);
        computer.add_input(robot_location.to_computer_input());

        match computer.run_program() {
            ProgramOutput::Complete(_) => break,
            ProgramOutput::Yielded(val) => {
                painted_points.insert(robot.location);
                match val {
                    0 => *robot_location = Color::Black,
                    1 => *robot_location = Color::White,
                    _ => panic!("Unexpected output value"),
                }
            }
        };

        match computer.run_program() {
            ProgramOutput::Complete(_) => break,
            ProgramOutput::Yielded(val) => {
                robot.turn(TurnDirection::from(val));
                robot.walk();
            }
        }
    }

    painted_points.len()
}
