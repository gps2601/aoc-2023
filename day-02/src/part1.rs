use regex::Regex;

use crate::custom_error::AocError;

#[tracing::instrument]
pub fn process(
    input: &str,
) -> miette::Result<String, AocError> {
    // only 12 red cubes, 13 green cubes, and 14 blue cubes
    const MAX_RED: i32 = 12;
    const MAX_GREEN: i32 = 13;
    const MAX_BLUE: i32 = 14;
    let lines = input.split('\n');
    let mut games = Vec::new();
    for line in lines {
        games.push(get_subsets(line));
    }
    let mut score: usize = 0;
    for (i, game) in games.iter().enumerate() {
        let all_possible = game.iter().all(|subset| {
            if subset.color == "green" {
                subset.count <= MAX_GREEN
            } else if subset.color == "red" {
                subset.count <= MAX_RED
            } else {
                subset.count <= MAX_BLUE
            }
        });
        if all_possible {
            score = score + i + 1
        }
    }
    Ok((score as i32).to_string())
}

fn get_subsets(line: &str) -> Vec<Subset> {
    let mut subsets = Vec::new();
    let pattern = Regex::new(r"^Game \d+: ").unwrap();
    let line = pattern.replace(line, "");
    let pattern = Regex::new(r", |; ").unwrap();
    let parts: Vec<&str> = pattern.split(&line).collect();
    for part in parts {
        let number_to_color = part.split(' ').collect::<Vec<&str>>();
        subsets.push(Subset {
            color: number_to_color[1].parse().unwrap(),
            count: number_to_color[0].parse().unwrap(),
        })
    }
    subsets
}

#[derive(PartialEq)]
#[derive(Debug)]
struct Subset {
    color: String,
    count: i32,
}

#[cfg(test)]
mod tests {

}
