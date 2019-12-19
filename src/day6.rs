use aoc_runner_derive::{aoc, aoc_generator};
use std::collections::HashMap;

#[derive(Debug)]
pub struct OrbitPair {
    parent: String,
    satellite: String,
}

#[derive(Debug)]
pub struct OrbitGraphEdges {
    satellites: Vec<String>,
}

impl OrbitGraphEdges {
    fn new() -> Self {
        OrbitGraphEdges {
            satellites: Vec::new(),
        }
    }
}

#[aoc_generator(day6)]
pub fn get_program(input: &str) -> HashMap<String, OrbitGraphEdges> {
    let pairs: Vec<OrbitPair> = input
        .lines()
        .map(|line| {
            let split = line.split(')').collect::<Vec<_>>();
            OrbitPair {
                parent: split.get(0).unwrap().to_string(),
                satellite: split.get(1).unwrap().to_string(),
            }
        })
        .collect();

    let mut orbit_graph: HashMap<String, OrbitGraphEdges> = HashMap::new();
    for orbit_pair in pairs {
        let entry = orbit_graph
            .entry(orbit_pair.parent)
            .or_insert(OrbitGraphEdges::new());
        entry.satellites.push(orbit_pair.satellite);
    }

    orbit_graph
}

#[aoc(day6, part1)]
pub fn part1(orbits: &HashMap<String, OrbitGraphEdges>) -> u32 {
    get_total_depths(orbits, "COM".to_string(), 0)
}

#[aoc(day6, part2)]
pub fn part2(orbits: &HashMap<String, OrbitGraphEdges>) -> u32 {
    //
    get_total_depths(orbits, "COM".to_string(), 0)
}

fn get_total_depths(
    orbits: &HashMap<String, OrbitGraphEdges>,
    current_node_name: String,
    current_depth: u32,
) -> u32 {
    let current_node = orbits.get(&current_node_name);

    return current_depth
        + current_node.map_or(0, |node| {
            node.satellites.iter().fold(0, |acc, curr| {
                acc + get_total_depths(orbits, curr.to_string(), current_depth + 1)
            })
        });
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    #[test]
    fn part1_single_satellite() {
        assert_eq!(part1(&get_program(&indoc!("COM)BFS"))), 1);
    }

    #[test]
    fn part1_two_satellites_of_parent() {
        assert_eq!(
            part1(&get_program(&indoc!(
                "COM)BFS
                COM)123"
            ))),
            2
        );
    }

    #[test]
    fn part1_two_satellite_chain() {
        assert_eq!(
            part1(&get_program(&indoc!(
                "COM)BFS
                BFS)123"
            ))),
            3
        );
    }
}
