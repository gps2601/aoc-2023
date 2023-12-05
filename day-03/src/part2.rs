use std::collections::{HashMap, HashSet};
use crate::custom_error::AocError;

#[tracing::instrument]
pub fn process(
    input: &str,
) -> miette::Result<String, AocError> {
    let grid: Vec<Vec<char>> = input.lines().map(String::from).map(|s| s.chars().collect()).collect();
    let all_found_numbers: Vec<FoundNumber> = grid.iter().enumerate().flat_map(|(index, row)| {
        find_number_positions_in_row(row, index as i32)
    }).collect();

    let mut gears_to_numbers: HashMap<String, HashSet<i32>> = HashMap::new();

    for found_number in all_found_numbers {
        let coords_at_number = found_number.coords;
        for coords in coords_at_number {
            let gears_around_coord = get_gear(&grid, coords.0, coords.1);
            for gear in gears_around_coord {
                if gears_to_numbers.contains_key(&*gear) {
                    gears_to_numbers
                        .get_mut(&*gear)
                        .expect("should be there")
                        .insert(found_number.value);
                } else {
                    let mut vec: HashSet<i32> = HashSet::new();
                    vec.insert(found_number.value);
                    gears_to_numbers.insert(gear, vec);
                }
            }
        }
    }
    let mut total = 0;
    for entry in gears_to_numbers {
        if entry.1.len() == 2 {
            let mut score = 1;
            entry.1.iter().for_each(|thing| {
                score *= thing
            });
            total += score;
        }
    }
    Ok(total.to_string())
}

fn get_gear(grid: &Vec<Vec<char>>, x: usize, y: usize) -> Vec<String> {
    let mut surrounding_values: Vec<String> = Vec::new();
    // Check if the coordinates are within the bounds of the grid
    if x < grid.len() && y < grid[0].len() {
        // Iterate over the neighboring coordinates
        for i in (x.saturating_sub(1))..=(x + 1).min(grid.len() - 1) {
            for j in (y.saturating_sub(1))..=(y + 1).min(grid[0].len() - 1) {
                // Exclude the central coordinate (x, y)
                if i != x || j != y {
                    let x1 = grid[j][i];
                    let string = j.to_string() + &*i.to_string();
                    if x1 == '*' {
                        surrounding_values.push(string);
                    }
                }
            }
        }
    }
    surrounding_values
}

fn find_number_positions_in_row(row: &Vec<char>, y_pos: i32) -> Vec<FoundNumber> {
    let max_x = row.len() - 1;
    let mut current_x = 0;
    let mut found_numbers: Vec<FoundNumber> = Vec::new();

    while current_x < max_x {
        if !row[current_x].is_numeric() {
            current_x += 1;
        } else {
            let mut coords: Vec<(usize, usize)> = Vec::new();
            let mut number_chars: Vec<char> = vec![row[current_x]];
            coords.push((current_x, y_pos as usize));
            while (current_x < max_x) && row[current_x + 1].is_numeric() {
                current_x += 1;
                coords.push((current_x, y_pos as usize));
                number_chars.push(row[current_x])
            }
            current_x += 1;
            let string_number: String = number_chars.iter().collect();
            let x: i32 = string_number.parse().expect("oops not a number");
            let found_number = FoundNumber {
                value: x,
                coords,
            };
            found_numbers.push(found_number)
        }
    }
    found_numbers
}

#[derive(PartialEq)]
#[derive(Debug)]
struct FoundNumber {
    value: i32,
    coords: Vec<(usize, usize)>,
}
