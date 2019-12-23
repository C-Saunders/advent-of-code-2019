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
pub fn generator(input: &str) -> Vec<Vec<u32>> {
    get_image_as_layers(input, 25, 6)
}

fn get_image_as_layers(input: &str, width: usize, height: usize) -> Vec<Vec<u32>> {
    let chunks = input.chars().chunks(width * height);
    chunks
        .into_iter()
        .map(|chunk| chunk.map(|i| i.to_digit(10).unwrap()).collect())
        .collect()
}

#[aoc(day8, part1)]
pub fn part1(image: &Vec<Vec<u32>>) -> u32 {
    // Find the layer that contains the fewest 0 digits. On that layer
    // what is the number of 1 digits multiplied by the number of 2 digits?
    let mut counts_with_fewest_zeros = OccurrenceCounts {
        zeros: std::u32::MAX,
        ones: 0,
        twos: 0,
    };

    for layer in image {
        let result = get_counts_for_layer(&layer);
        if result.zeros < counts_with_fewest_zeros.zeros {
            counts_with_fewest_zeros = result;
        }
    }

    counts_with_fewest_zeros.ones * counts_with_fewest_zeros.twos
}

fn get_counts_for_layer(layer: &Vec<u32>) -> OccurrenceCounts {
    let mut result = OccurrenceCounts::new();
    for digit in layer {
        match digit {
            0 => result.zeros += 1,
            1 => result.ones += 1,
            2 => result.twos += 1,
            _ => {}
        };
    }

    result
}

#[aoc(day8, part2)]
pub fn part2(image: &Vec<Vec<u32>>) -> String {
    make_image(image, 25, 6)
}

fn make_image(image_data: &Vec<Vec<u32>>, width: usize, height: usize) -> String {
    let mut image_as_line: Vec<Option<u32>> = vec![None; width * height];

    for index in 0..image_data[0].len() {
        for layer in image_data {
            if image_as_line[index].is_some() {
                continue;
            }
            if layer[index] != 2 {
                image_as_line[index] = Some(layer[index]);
            }
        }
    }

    let image_as_string: String = image_as_line
        .iter()
        .map(|item| match item {
            Some(0) => ' ',
            Some(1) => '#',
            _ => panic!("what?"),
        })
        .collect();

    let mut output = "\n".to_string();
    for chunk in &image_as_string.chars().chunks(width) {
        output.push_str(&chunk.collect::<String>());
        output.push_str("\n");
    }

    output
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_counts_for_layer_test() {
        assert_eq!(
            get_counts_for_layer(&vec![0, 1, 2, 3, 0, 2, 1, 0, 1, 1, 2, 1]),
            OccurrenceCounts {
                zeros: 3,
                ones: 5,
                twos: 3
            }
        );
    }

    #[test]
    fn image_test() {
        assert_eq!(
            make_image(&get_image_as_layers(&"0222112222120000", 2, 2), 2, 2),
            "\n #\n# \n"
        );
    }
}
