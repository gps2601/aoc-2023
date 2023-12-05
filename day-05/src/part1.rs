use crate::custom_error::AocError;

#[tracing::instrument]
pub fn process(
    input: &str,
) -> miette::Result<String, AocError> {
    let seeds: Vec<i64> = input.lines().next().expect("should exist").split_whitespace().filter_map(|it| it.parse().ok()).collect();
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
                    range: numbers[2],
                }
            }).collect();
            SeedToType {
                ranges,
            }
        })
        .collect();

    let mut lowest_final_values = vec![];
    seeds.iter().for_each(|seed| {
        let mut seed_to_convert: i64 = *seed;
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
    });
    Ok(lowest_final_values.iter().cloned().min().expect("should exist").to_string())
}

struct SeedToType {
    ranges: Vec<ConversionRange>,
}

struct ConversionRange {
    dest_start: i64,
    source_start: i64,
    range: i64,
}
