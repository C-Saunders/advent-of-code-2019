use aoc_runner_derive::{aoc, aoc_generator};
use std::collections::HashMap;

#[derive(Debug)]
pub struct OrbitPair {
    parent: String,
    satellite: String,
}

#[derive(Debug)]
pub struct OrbitGraphEdges {
    parent: Option<String>,
    satellites: Vec<String>,
}

impl OrbitGraphEdges {
    fn new() -> Self {
        OrbitGraphEdges {
            parent: None,
            satellites: Vec::new(),
        }
    }
}

#[aoc_generator(day6)]
pub fn get_orbits(input: &str) -> Vec<OrbitPair> {
    input
        .lines()
        .map(|line| {
            let split = line.split(')').collect::<Vec<_>>();
            OrbitPair {
                parent: split.get(0).unwrap().to_string(),
                satellite: split.get(1).unwrap().to_string(),
            }
        })
        .collect()
}

fn build_graph(pairs: &[OrbitPair]) -> HashMap<String, OrbitGraphEdges> {
    let mut orbit_graph: HashMap<String, OrbitGraphEdges> = HashMap::new();
    for orbit_pair in pairs {
        let parent_entry = orbit_graph
            .entry(orbit_pair.parent.clone())
            .or_insert(OrbitGraphEdges::new());
        parent_entry.satellites.push(orbit_pair.satellite.clone());

        let satellite_entry = orbit_graph
            .entry(orbit_pair.satellite.clone())
            .or_insert(OrbitGraphEdges::new());
        satellite_entry.parent = Some(orbit_pair.parent.clone());
    }

    orbit_graph
}

#[aoc(day6, part1)]
pub fn part1(orbit_pairs: &[OrbitPair]) -> u32 {
    let orbits = build_graph(&orbit_pairs);
    get_total_depths(&orbits, "COM".to_string(), 0)
}

#[aoc(day6, part2)]
pub fn part2(orbit_pairs: &[OrbitPair]) -> Option<u32> {
    get_num_orbital_transfers(&orbit_pairs, "SAN", "YOU")
}

fn get_num_orbital_transfers(orbit_pairs: &[OrbitPair], start: &str, end: &str) -> Option<u32> {
    let start_is_orbiting = find_parent(&orbit_pairs, start).unwrap();
    let end_is_orbiting = find_parent(&orbit_pairs, end).unwrap();
    let orbit_graph = build_graph(&orbit_pairs);

    let mut current_orbit_name = start_is_orbiting;
    let mut result: Option<u32> = None;
    let mut distance = 0;

    while result.is_none() {
        result = get_depth_to_target(
            &orbit_graph,
            &current_orbit_name,
            distance,
            &end_is_orbiting,
        );

        if result.is_some() {
            break;
        }

        // covers the case where the end node is orbiting the root
        if current_orbit_name == end_is_orbiting {
            result = Some(distance);
            break;
        }

        let current_orbit = orbit_graph.get(&current_orbit_name.clone()).unwrap();
        if current_orbit.parent.is_none() {
            // We've hit the root of the tree without finding it
            break;
        }

        // Go up one level and continue
        current_orbit_name = current_orbit.parent.as_ref().unwrap().clone();
        distance += 1;
    }

    result
}

fn find_parent(orbit_pairs: &[OrbitPair], target_name: &str) -> Option<String> {
    orbit_pairs.iter().find_map(|orbit| {
        if orbit.satellite == target_name {
            return Some(orbit.parent.clone());
        }

        None
    })
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

fn get_depth_to_target(
    orbits: &HashMap<String, OrbitGraphEdges>,
    current_node_name: &String,
    current_depth: u32,
    target_node_name: &String,
) -> Option<u32> {
    let current_node = orbits.get(current_node_name).unwrap();

    for satellite in current_node.satellites.iter() {
        // we want the depth of the parent, not the satellite itself
        // but target_node_name *is* the parent, so it's the next depth
        // in retrospect, target_node_name should probably have been the actual target
        if satellite == target_node_name {
            return Some(current_depth + 1);
        }

        let result = get_depth_to_target(&orbits, satellite, current_depth + 1, target_node_name);

        if result.is_some() {
            return result;
        }
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    #[test]
    fn total_orbits_single_satellite() {
        assert_eq!(part1(&get_orbits(&indoc!("COM)BFS"))), 1);
    }

    #[test]
    fn total_orbits_two_satellites_of_parent() {
        assert_eq!(
            part1(&get_orbits(&indoc!(
                "COM)BFS
                COM)123"
            ))),
            2
        );
    }

    #[test]
    fn total_orbits_two_satellite_chain() {
        assert_eq!(
            part1(&get_orbits(&indoc!(
                "COM)BFS
                BFS)123"
            ))),
            3
        );
    }

    #[test]
    fn orbital_transfers() {
        let orbits = get_orbits(&indoc!(
            "COM)START
            START)123
            START)ABC
            ABC)FOO
            FOO)END"
        ));
        assert_eq!(
            get_num_orbital_transfers(&orbits, &"START", &"END"),
            Some(3)
        );
    }

    #[test]
    fn orbital_transfers_rev() {
        let orbits = get_orbits(&indoc!(
            "COM)START
            START)123
            START)ABC
            ABC)FOO
            FOO)END"
        ));
        assert_eq!(
            get_num_orbital_transfers(&orbits, &"END", &"START"),
            Some(3)
        );
    }

    #[test]
    fn orbital_transfers_spurs_only() {
        let orbits = get_orbits(&indoc!(
            "COM)A
            A)B
            B)START
            B)C
            C)D
            D)END"
        ));
        assert_eq!(
            get_num_orbital_transfers(&orbits, &"START", &"END"),
            Some(2)
        );
    }

    #[test]
    fn orbital_transfers_spurs_only_rev() {
        let orbits = get_orbits(&indoc!(
            "COM)A
            A)B
            B)START
            B)C
            C)D
            D)END"
        ));
        assert_eq!(
            get_num_orbital_transfers(&orbits, &"END", &"START"),
            Some(2)
        );
    }

    #[test]
    fn part2_provided_example() {
        let orbits = get_orbits(&indoc!(
            "COM)B
            B)C
            C)D
            D)E
            E)F
            B)G
            G)H
            D)I
            E)J
            J)K
            K)L
            K)YOU
            I)SAN"
        ));
        assert_eq!(get_num_orbital_transfers(&orbits, &"YOU", &"SAN"), Some(4));
        assert_eq!(get_num_orbital_transfers(&orbits, &"SAN", &"YOU"), Some(4));
    }
}
