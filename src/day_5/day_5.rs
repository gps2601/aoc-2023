use std::ops::Range;
use indicatif::ParallelProgressIterator;

use rayon::prelude::*;
use crate::utilities::input;

struct SeedToType {
    ranges: Vec<ConversionRange>,
}

struct ConversionRange {
    dest_start: i64,
    source_start: i64,
    range: i64,
    source_range: Range<i64>,
}

#[allow(dead_code)]
fn part_1(file_path: &str) -> i64 {
    let seeds: Vec<i64> = input::as_lines(file_path).first().expect("should exist").split_whitespace().filter_map(|it| it.parse().ok()).collect();
    let seed_to_types: Vec<SeedToType> = input::all(file_path)
        .splitn(2, "\n\n")
        .last().expect("should exist")
        .split("\n\n")
        .map(|map| {
            let lines_in_map = map.split("\n");
            let ranges: Vec<ConversionRange> = lines_in_map.map(|line| {
                let numbers: Vec<i64> = line.split(" ").map(|number| number.parse::<i64>().expect("should parse")).collect();
                ConversionRange {
                    dest_start: numbers[0],
                    source_start: numbers[1],
                    range: numbers[2],
                    source_range: 0..0,
                }
            }).collect();
            SeedToType {
                ranges,
            }
        })
        .collect();

    let mut lowest_final_values = vec![];
    seeds.iter().for_each(|seed| {
        println!("Doing seed: {seed}");
        let mut seed_to_convert: i64 = seed.clone();
        for map in &seed_to_types {
            let ranges_that_match = map.ranges.iter().filter(|range| {
                let range1 = range.source_start..range.source_start + range.range;
                range1.contains(&seed_to_convert)
            });

            let maybe_range = ranges_that_match.into_iter().next();
            if maybe_range.is_some() {
                let range_to_use = &maybe_range.expect("should exist");
                let position_in_range = seed_to_convert - range_to_use.source_start;
                seed_to_convert = range_to_use.dest_start + position_in_range
            }
        }
        lowest_final_values.push(seed_to_convert);
        println!("final value for seed is: {}", seed_to_convert);
    });
    lowest_final_values.iter().cloned().min().expect("should exist")
}


#[allow(dead_code)]
pub fn part_2(file_path: &str) -> i64 {
    let seeds: Vec<i64> = input::as_lines(file_path).first().expect("should exist").split_whitespace().filter_map(|it| it.parse().ok()).collect();
    let seeds: Vec<i64> = seeds.par_chunks(2)
        .flat_map(|chunk| {
            let start: i64 = chunk[0];
            let number: i64 = chunk[1];
            (start..start + number).collect::<Vec<i64>>()
        })
        .collect();
    let seed_to_types: Vec<SeedToType> = input::all(file_path)
        .splitn(2, "\n\n")
        .last().expect("should exist")
        .split("\n\n")
        .map(|map| {
            let lines_in_map = map.split("\n");
            let ranges: Vec<ConversionRange> = lines_in_map.map(|line| {
                let numbers: Vec<i64> = line.split(" ").map(|number| number.parse::<i64>().expect("should parse")).collect();
                ConversionRange {
                    dest_start: numbers[0],
                    source_start: numbers[1],
                    range: numbers[2],
                    source_range: numbers[1]..numbers[1] + numbers[2]
                }
            }).collect();
            SeedToType {
                ranges,
            }
        })
        .collect();

    let lowest: Vec<i64> = seeds.into_par_iter().progress().map(|seed| {
        let mut seed_to_convert: i64 = seed;
        for map in &seed_to_types {
            let maybe_range = map
                .ranges
                .iter()
                .find(|range| {
                    let range1 = &range.source_range;
                    range1.contains(&seed_to_convert)
                });

            if let Some(range_to_use) = maybe_range {
                let position_in_range = seed_to_convert - range_to_use.source_start;
                seed_to_convert = range_to_use.dest_start + position_in_range;
            }
        }
        seed_to_convert
    }).collect();

    let min_value = lowest.iter().cloned().min().expect("should exist");
    return min_value;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_1_sample() {
        assert_eq!(35, part_1("src/day_5/day_5_sample.txt"))
    }

    #[test]
    fn part_1_full() {
        assert_eq!(486613012, part_1("src/day_5/day_5.txt"))
    }

    #[test]
    fn part_2_sample() {
        assert_eq!(46, part_2("src/day_5/day_5_sample.txt"))
    }

    // #[test]
    // fn part_2_full() {
    //     assert_eq!(56931769, part_2("src/day_5/day_5.txt"))
    // }
}
