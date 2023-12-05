use crate::custom_error::AocError;

#[tracing::instrument]
pub fn process(
    input: &str,
) -> miette::Result<String, AocError> {
    let grid: Vec<Vec<char>> = input.lines().map(String::from).map(|s| s.chars().collect()).collect();
    let all_found_numbers : Vec<FoundNumber> = grid.iter().enumerate().flat_map(|(index, row)| {
        find_number_positions_in_row(row, index as i32)
    }).collect();
    let mut final_numbers = vec![];
    all_found_numbers.iter().for_each(|found_number| {
        let special_chars = "%*#&$@/=+-";
        let coordinates = &found_number.coords;
        let mut is_part = false;
        coordinates.iter().for_each(|&coord| {
            let surrounding = get_surrounding_values(&grid, coord.0, coord.1);
            let x = surrounding.iter().any(|character| special_chars.contains(*character));
            if x { is_part = true }
        });
        if is_part {
            final_numbers.push(found_number.value);
        }
    });
    Ok(final_numbers.iter().sum::<i32>().to_string())
}

fn find_number_positions_in_row(row: &Vec<char>, y_pos: i32) -> Vec<FoundNumber>{
    let max_x = row.len() - 1;
    let mut current_x = 0;
    let mut found_numbers:Vec<FoundNumber> = Vec::new();

    while current_x < max_x {
        if !row[current_x].is_numeric() {
            current_x += 1;
        } else {
            let mut coords : Vec<(usize, usize)> = Vec::new();
            let mut number_chars : Vec<char> = vec![row[current_x]];
            coords.push((current_x, y_pos as usize));
            while (current_x < max_x) && row[current_x+1].is_numeric() {
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

#[allow(clippy::needless_range_loop)]
fn get_surrounding_values(grid: &Vec<Vec<char>>, x: usize, y: usize) -> Vec<char> {
    let mut surrounding_values : Vec<char> = Vec::new();
    if x < grid.len() && y < grid[0].len() {
        for i in x.saturating_sub(1)..=(x + 1).min(grid.len() - 1) {
            for j in y.saturating_sub(1)..=(y + 1).min(grid[0].len() - 1) {
                if i != x || j != y {
                    surrounding_values.push(grid[j][i]);
                }
            }
        }
    }
    surrounding_values
}

#[derive(PartialEq)]
#[derive(Debug)]
struct FoundNumber {
    value: i32,
    coords: Vec<(usize, usize)>,
}

