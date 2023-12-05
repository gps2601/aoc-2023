use regex::Regex;

use crate::utilities::input;

#[allow(dead_code)]
pub fn part_1(file_path: &str) -> u32 {
    let output = input::all(file_path)
        .lines()
        .map(|line| {
            let mut it = line.chars().filter_map(|character| {
                character.to_digit(10)
            });
            let first = it.next().expect("should be a number");
            match it.last() {
                Some(num) => format!("{first}{num}"),
                None => format!("{first}{first}")
            }.parse::<u32>().expect("should be a number")
        })
        .sum::<u32>();
    output
}

#[allow(dead_code)]
fn part_2(file_path: &str) -> u32 {
    return input::all(file_path).lines().map(|line| {
        let mut it = (0..line.len()).filter_map(|index| {
            let reduced_line = &line[index..];
            let result = if reduced_line.starts_with("one") {
                Some(1)
            } else if reduced_line.starts_with("two") {
                Some(2)
            } else if reduced_line.starts_with("three") {
                Some(3)
            } else if reduced_line.starts_with("four") {
                Some(4)
            } else if reduced_line.starts_with("five") {
                Some(5)
            } else if reduced_line.starts_with("six") {
                Some(6)
            } else if reduced_line.starts_with("seven") {
                Some(7)
            } else if reduced_line.starts_with("eight") {
                Some(8)
            } else if reduced_line.starts_with("nine") {
                Some(9)
            } else {
                reduced_line
                    .chars()
                    .next()
                    .unwrap()
                    .to_digit(10)
            };

            result
        });
        let first = it.next().expect("should be a number");

        match it.last() {
            Some(num) => first * 10 + num,
            None => first * 10 + first,
        }
    }).sum::<u32>();
}

#[allow(dead_code)]
fn find_number_words_or_digit(input: &String) -> Vec<&str> {
    let mut matches = Vec::new();
    let pattern = Regex::new(r#"(one|two|three|four|five|six|seven|eight|nine|\d)"#).unwrap();
    for mat in pattern.find_iter(input) {
        matches.push(mat.as_str());
    }
    matches
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_1_sample() {
        assert_eq!(142, part_1("src/day_1/day_1_sample.txt"));
    }

    #[test]
    fn part_1_full() {
        assert_eq!(54953, part_1("src/day_1/day_1.txt"));
    }

    #[test]
    fn part_2_sample() {
        assert_eq!(281, part_2("src/day_1/day_1_sample_2.txt"));
    }

    #[test]
    fn part_2_full() {
        assert_eq!(53868, part_2("src/day_1/day_1.txt"));
    }
}