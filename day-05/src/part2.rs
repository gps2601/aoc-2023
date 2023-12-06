use std::ops::Range;

use rayon::prelude::*;

use crate::custom_error::AocError;

#[tracing::instrument]
pub fn process(
    input: &str,
) -> miette::Result<String, AocError> {
    let seeds: Vec<i64> = input.lines().next().expect("should exist").split_whitespace().filter_map(|it| it.parse().ok()).collect();
    let seeds: Vec<i64> = seeds.par_chunks(2)
        .flat_map(|chunk| {
            let start: i64 = chunk[0];
            let number: i64 = chunk[1];
            (start..start + number).collect::<Vec<i64>>()
        })
        .collect();
    let seed_to_types: Vec<SeedToType> = input
        .splitn(2, "\n\n")
        .last().expect("should exist")
        .split("\n\n")
        .map(|map| {
            let lines_in_map = map.split('\n');
            let ranges: Vec<ConversionRange> = lines_in_map.map(|line| {
                let numbers: Vec<i64> = line.split(' ').map(|number| number.parse::<i64>().expect("should parse")).collect();
                ConversionRange {
                    dest_start: numbers[0],
                    source_start: numbers[1],
                    source_range: numbers[1]..numbers[1] + numbers[2],
                }
            }).collect();
            SeedToType {
                ranges,
            }
        })
        .collect();

    let lowest: i64 = seeds.into_par_iter().map(|seed| {
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
    }).min().expect("should exist");

    Ok(lowest.to_string())
}

struct SeedToType {
    ranges: Vec<ConversionRange>,
}

struct ConversionRange {
    dest_start: i64,
    source_start: i64,
    source_range: Range<i64>,
}
