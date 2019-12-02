use aoc_runner_derive::{aoc, aoc_generator};
use std::num::ParseFloatError;

#[aoc_generator(day1)]
pub fn get_masses(input: &str) -> Result<Vec<f32>, ParseFloatError> {
    input.lines().map(|l| l.parse::<f32>()).collect()
}

#[aoc(day1, part1)]
pub fn part1(masses: &[f32]) -> f32 {
    masses
        .iter()
        .fold(0.0, |acc, mass| acc + (mass / 3.0).floor() - 2.0)
}

#[aoc(day1, part2)]
pub fn part2(masses: &[f32]) -> f32 {
    masses
        .iter()
        .fold(0.0, |acc, mass| acc + calculate_fuel(mass, 0.0))
}

fn calculate_fuel(mass: &f32, current_fuel_required: f32) -> f32 {
    let required_fuel = (mass / 3.0).floor() - 2.0;

    if required_fuel <= 0.0 {
        return current_fuel_required;
    }

    calculate_fuel(&required_fuel, current_fuel_required + required_fuel)
}

#[cfg(test)]
mod tests {
    use super::calculate_fuel;

    #[test]
    fn test() {
        assert_eq!(calculate_fuel(&100756.0, 0.0), 50346.0)
    }
}
