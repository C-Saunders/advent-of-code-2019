use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;

#[derive(Debug, PartialEq)]
pub struct OccurrenceCounts {
    zeros: u32,
    ones: u32,
    twos: u32,
}

impl OccurrenceCounts {
    fn new() -> Self {
        OccurrenceCounts {
            zeros: 0,
            ones: 0,
            twos: 0,
        }
    }
}

#[aoc_generator(day8)]
pub fn get_input_rows(input: &str) -> Vec<Vec<u32>> {
    let chunks = input.chars().chunks(25);
    chunks
        .into_iter()
        .map(|chunk| chunk.map(|i| i.to_digit(10).unwrap()).collect())
        .collect()
}

#[aoc(day8, part1)]
pub fn part1(image: &Vec<Vec<u32>>) -> u32 {
    let num_layers = image.len() / 6;
    let layers = image.chunks(num_layers);

    // Find the layer that contains the fewest 0 digits. On that layer
    // what is the number of 1 digits multiplied by the number of 2 digits?
    let mut counts_with_fewest_zeros = OccurrenceCounts {
        zeros: std::u32::MAX,
        ones: 0,
        twos: 0,
    };

    for layer in layers {
        let result = get_counts_for_layer(&layer);
        if result.zeros < counts_with_fewest_zeros.zeros {
            counts_with_fewest_zeros = result;
        }
    }

    dbg!(&counts_with_fewest_zeros);

    counts_with_fewest_zeros.ones * counts_with_fewest_zeros.twos
}

fn get_counts_for_layer(layer: &[Vec<u32>]) -> OccurrenceCounts {
    let mut result = OccurrenceCounts::new();
    for chunk in layer {
        for digit in chunk {
            match digit {
                0 => result.zeros += 1,
                1 => result.ones += 1,
                2 => result.twos += 1,
                _ => {}
            };
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_counts_for_layer_test() {
        assert_eq!(
            get_counts_for_layer(&[vec![0, 1, 2, 3], vec![0, 2, 1, 0], vec![1, 1, 2, 1]]),
            OccurrenceCounts {
                zeros: 3,
                ones: 5,
                twos: 3
            }
        );
    }
}
