use regex::Regex;

use crate::custom_error::AocError;

#[tracing::instrument]
pub fn process(
    input: &str,
) -> miette::Result<String, AocError> {
    let lines = input.split('\n');
    let mut games = Vec::new();
    for line in lines {
        games.push(get_subsets(line));
    }
    let mut score = 0;
    for (_i, game) in games.iter().enumerate() {
        let mut red_subsets = game.iter().filter(|subset| subset.color == "red").collect::<Vec<&Subset>>();
        red_subsets.sort_by(|a, b| b.count.cmp(&a.count));
        let red_score = red_subsets.first().unwrap().count;
        let mut green_subsets = game.iter().filter(|subset| subset.color == "green").collect::<Vec<&Subset>>();
        green_subsets.sort_by(|a, b| b.count.cmp(&a.count));
        let green_score = green_subsets.first().unwrap().count;
        let mut blue_subsets = game.iter().filter(|subset| subset.color == "blue").collect::<Vec<&Subset>>();
        blue_subsets.sort_by(|a, b| b.count.cmp(&a.count));
        let blue_score = blue_subsets.first().unwrap().count;
        score += red_score * green_score * blue_score;
    }
    Ok(score.to_string())
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
mod tests {}