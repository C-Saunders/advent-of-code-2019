use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;
use regex::Regex;
use std::cell::RefCell;

#[derive(Debug, Clone, Copy)]
pub struct ThreeDimensionalValue {
    x: i32,
    y: i32,
    z: i32,
}

impl ThreeDimensionalValue {
    fn zeros() -> Self {
        ThreeDimensionalValue { x: 0, y: 0, z: 0 }
    }
}

#[derive(Debug)]
pub struct Moon {
    position: ThreeDimensionalValue,
    velocity: ThreeDimensionalValue,
}

impl Moon {
    fn apply_velocity(&mut self) {
        self.position.x += self.velocity.x;
        self.position.y += self.velocity.y;
        self.position.z += self.velocity.z;
    }

    fn potential_energy(&self) -> i32 {
        self.position.x.abs() + self.position.y.abs() + self.position.z.abs()
    }

    fn kinetic_energy(&self) -> i32 {
        self.velocity.x.abs() + self.velocity.y.abs() + self.velocity.z.abs()
    }
}

#[aoc_generator(day12)]
pub fn get_starting_positions(input: &str) -> Vec<RefCell<Moon>> {
    lazy_static! {
        static ref PARSING_EXPR: Regex =
            Regex::new(r"^<x=(?P<x>-?\d+), y=(?P<y>-?\d+), z=(?P<z>-?\d+)>$").unwrap();
    }
    let positions: Vec<ThreeDimensionalValue> = input
        .lines()
        .map(|line| {
            let caps = PARSING_EXPR.captures(line).unwrap();
            ThreeDimensionalValue {
                x: caps["x"].parse::<i32>().unwrap(),
                y: caps["y"].parse::<i32>().unwrap(),
                z: caps["z"].parse::<i32>().unwrap(),
            }
        })
        .collect();

    return vec![
        RefCell::new(Moon {
            position: positions[0],
            velocity: ThreeDimensionalValue::zeros(),
        }),
        RefCell::new(Moon {
            position: positions[1],
            velocity: ThreeDimensionalValue::zeros(),
        }),
        RefCell::new(Moon {
            position: positions[2],
            velocity: ThreeDimensionalValue::zeros(),
        }),
        RefCell::new(Moon {
            position: positions[3],
            velocity: ThreeDimensionalValue::zeros(),
        }),
    ];
}

#[aoc(day12, part1)]
pub fn part1(moons: &[RefCell<Moon>]) -> i32 {
    let ticks = 1000;

    for _ in 0..ticks {
        update_velocities(&moons);
        for moon in moons {
            moon.borrow_mut().apply_velocity();
        }
    }

    moons.iter().fold(0, |acc, curr| {
        let moon = curr.borrow();
        acc + moon.potential_energy() * moon.kinetic_energy()
    })
}

fn update_velocities(moons: &[RefCell<Moon>]) {
    for moon_pair_indexes in (0..(moons.len())).combinations(2) {
        let mut a = moons[moon_pair_indexes[0]].borrow_mut();
        let mut b = moons[moon_pair_indexes[1]].borrow_mut();
        if a.position.x < b.position.x {
            a.velocity.x += 1;
            b.velocity.x -= 1;
        } else if b.position.x < a.position.x {
            a.velocity.x -= 1;
            b.velocity.x += 1;
        }

        if a.position.y < b.position.y {
            a.velocity.y += 1;
            b.velocity.y -= 1;
        } else if b.position.y < a.position.y {
            a.velocity.y -= 1;
            b.velocity.y += 1;
        }

        if a.position.z < b.position.z {
            a.velocity.z += 1;
            b.velocity.z -= 1;
        } else if b.position.z < a.position.z {
            a.velocity.z -= 1;
            b.velocity.z += 1;
        }
    }
}

// #[aoc(day12, part2)]
// pub fn part2(program_input: &[ThreeDimensionalPoint]) -> usize {
//     0
// }
