use std::fs;
use regex::Regex;

#[allow(dead_code)]
fn day2() -> i32 {
    // only 12 red cubes, 13 green cubes, and 14 blue cubes
    const MAX_RED: i32 = 12;
    const MAX_GREEN: i32 = 13;
    const MAX_BLUE: i32 = 14;

    let file_path = "src/day_2/day_2.txt";
    let file = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");
    let lines = file.split("\n");
    let mut games = Vec::new();
    for line in lines {
        games.push(get_subsets(line));
    }
    let mut score : usize = 0;
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
    score as i32
}

#[allow(dead_code)]
fn day2_part2() -> i32 {
    // only 12 red cubes, 13 green cubes, and 14 blue cubes
    const MAX_RED: i32 = 12;
    const MAX_GREEN: i32 = 13;
    const MAX_BLUE: i32 = 14;

    let file_path = "src/day_2/day_2.txt";
    let file = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");
    let lines = file.split("\n");
    let mut games = Vec::new();
    for line in lines {
        games.push(get_subsets(line));
    }
    let mut score = 0;
    for (_i, game) in games.iter().enumerate() {
        let mut red_subsets = game.into_iter().filter(|subset| subset.color == "red").collect::<Vec<&Subset>>();
        red_subsets.sort_by(|a, b| b.count.cmp(&a.count));
        let red_score = red_subsets.first().unwrap().count;
        let mut green_subsets = game.into_iter().filter(|subset| subset.color == "green").collect::<Vec<&Subset>>();
        green_subsets.sort_by(|a, b| b.count.cmp(&a.count));
        let green_score = green_subsets.first().unwrap().count;
        let mut blue_subsets = game.into_iter().filter(|subset| subset.color == "blue").collect::<Vec<&Subset>>();
        blue_subsets.sort_by(|a, b| b.count.cmp(&a.count));
        let blue_score = blue_subsets.first().unwrap().count;
        score = score + (red_score * green_score * blue_score);
    }
    score
}

fn get_subsets(line: &str) -> Vec<Subset> {
    let mut subsets = Vec::new();
    let pattern = Regex::new(r"^Game \d+: ").unwrap();
    let line = pattern.replace(line, "");
    let pattern = Regex::new(r", |; ").unwrap();
    let parts: Vec<&str> = pattern.split(&*line).collect();
    for part in parts {
        let number_to_color = part.split(" ").collect::<Vec<&str>>();
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
    use super::*;

    #[test]
    fn part_1() {
        assert_eq!(2331, day2())
    }

    #[test]
    fn part_2() {
        assert_eq!(71585, day2_part2())
    }

    #[test]
    fn converts_line_correctly() {
        let expected: Vec<Subset> = vec![
            Subset { color: String::from("blue"), count: 3 },
            Subset { color: String::from("red"), count: 4 },
            Subset { color: String::from("red"), count: 1 },
            Subset { color: String::from("green"), count: 2 },
            Subset { color: String::from("blue"), count: 6 },
            Subset { color: String::from("green"), count: 2 },
        ];
        let result = get_subsets("Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green");
        assert_eq!(expected, result);

        let expected: Vec<Subset> = vec![
            Subset { color: String::from("green"), count: 1 },
            Subset { color: String::from("red"), count: 3 },
            Subset { color: String::from("blue"), count: 6 },
            Subset { color: String::from("green"), count: 3 },
            Subset { color: String::from("red"), count: 6 },
            Subset { color: String::from("green"), count: 3 },
            Subset { color: String::from("blue"), count: 15 },
            Subset { color: String::from("red"), count: 14 },
        ];
        let result = get_subsets("Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red");
        assert_eq!(expected, result);
    }
}