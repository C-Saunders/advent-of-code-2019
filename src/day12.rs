use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;
use regex::Regex;
use std::cell::RefCell;
use std::collections::hash_map::DefaultHasher;
use std::collections::HashSet;
use std::hash::{Hash, Hasher};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
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

#[derive(Debug, PartialEq, Eq, Hash)]
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

#[derive(PartialEq, Eq)]
pub struct MoonSystem {
    io: RefCell<Moon>,
    europa: RefCell<Moon>,
    ganymede: RefCell<Moon>,
    callisto: RefCell<Moon>,
}

impl MoonSystem {
    fn len(&self) -> usize {
        4
    }

    fn get_by_index(&self, index: usize) -> &RefCell<Moon> {
        match index {
            0 => &self.io,
            1 => &self.europa,
            2 => &self.ganymede,
            3 => &self.callisto,
            _ => panic!("No moon at index"),
        }
    }

    fn as_vec(&self) -> Vec<&RefCell<Moon>> {
        vec![&self.io, &self.europa, &self.ganymede, &self.callisto]
    }
}

impl Hash for MoonSystem {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.io.borrow().hash(state);
        self.europa.borrow().hash(state);
        self.ganymede.borrow().hash(state);
        self.callisto.borrow().hash(state);
    }
}

#[aoc_generator(day12)]
pub fn get_starting_positions(input: &str) -> MoonSystem {
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

    return MoonSystem {
        io: RefCell::new(Moon {
            position: positions[0],
            velocity: ThreeDimensionalValue::zeros(),
        }),
        europa: RefCell::new(Moon {
            position: positions[1],
            velocity: ThreeDimensionalValue::zeros(),
        }),
        ganymede: RefCell::new(Moon {
            position: positions[2],
            velocity: ThreeDimensionalValue::zeros(),
        }),
        callisto: RefCell::new(Moon {
            position: positions[3],
            velocity: ThreeDimensionalValue::zeros(),
        }),
    };
}

#[aoc(day12, part1)]
pub fn part1(moons: &MoonSystem) -> i32 {
    let ticks = 1000;

    for _ in 0..ticks {
        update_velocities(&moons);
        for moon in moons.as_vec().iter() {
            moon.borrow_mut().apply_velocity();
        }
    }

    moons.as_vec().iter().fold(0, |acc, curr| {
        let moon = curr.borrow();
        acc + moon.potential_energy() * moon.kinetic_energy()
    })
}

fn update_velocities(moons: &MoonSystem) {
    for moon_pair_indexes in (0..(moons.len())).combinations(2) {
        let a_ref = moons.get_by_index(moon_pair_indexes[0]);
        let b_ref = moons.get_by_index(moon_pair_indexes[1]);
        let mut a = a_ref.borrow_mut();
        let mut b = b_ref.borrow_mut();
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

#[aoc(day12, part2)]
pub fn part2(moons: &MoonSystem) -> usize {
    let mut seen = HashSet::<u64>::new();
    seen.insert(calculate_hash(&moons));

    loop {
        update_velocities(&moons);
        for moon in moons.as_vec().iter() {
            moon.borrow_mut().apply_velocity();
        }
        let curr_hash = calculate_hash(&moons);
        if seen.contains(&curr_hash) {
            break;
        }
        seen.insert(curr_hash);
    }

    seen.len()
}

fn calculate_hash<T: Hash>(t: &T) -> u64 {
    let mut s = DefaultHasher::new();
    t.hash(&mut s);
    s.finish()
}
