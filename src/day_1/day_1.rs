use std::collections::HashMap;
use regex::Regex;
use crate::utilities::input;
#[allow(dead_code)]

pub fn part_1(file_path: &str) -> i32 {
    let calibration_values = input::as_lines(file_path);
    let calibration_numbers: Vec<String> = calibration_values
        .iter()
        .map(|calibration_value| just_numeric(calibration_value)).collect();
    let first_lasts: Vec<i32> = calibration_numbers.iter().map(|numbers| {
        let mut first = numbers.chars().next().expect("").to_string();
        let last = numbers.chars().last().expect("");
        first.push(last);
        let result: i32 = first.parse().unwrap();
        result
    }).collect();
    first_lasts.iter().sum()
}

#[allow(dead_code)]
fn part_2(file_path: &str) -> i32 {
    let number_map: HashMap<&str, &str> = vec![
        ("one", "1"),
        ("two", "2"),
        ("three", "3"),
        ("four", "4"),
        ("five", "5"),
        ("six", "6"),
        ("seven", "7"),
        ("eight", "8"),
        ("nine", "9"),
    ].into_iter().collect();
    let lines = input::as_lines(file_path);
    let lines_of_word_digits = lines.iter()
        .map(|line| { find_number_words_or_digit(line) }).collect::<Vec<Vec<&str>>>();
    let mut total = 0;
    lines_of_word_digits.iter().for_each(|entry| {
        let mut first = entry.first().expect("should exist");
        let mut last = entry.last().expect("should exist");
        if number_map.contains_key(first) {
            first = number_map.get(first).expect("should find");
        }
        if number_map.contains_key(last) {
            last = number_map.get(last).expect("should find");
        }
        let combined = first.to_string() + last;
        let result = combined.parse::<i32>().expect("should parse");
        total = total + result;
    });
    total
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

fn just_numeric(value: &String) -> String {
    return value.chars().filter(|x| x.is_numeric()).collect();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn just_numeric_strips_alphabetical() {
        assert_eq!("12", just_numeric(&String::from("1abc2")));
        assert_eq!("38", just_numeric(&String::from("pqr3stu8vwx")));
        assert_eq!("12345", just_numeric(&String::from("a1b2c3d4e5f")));
        assert_eq!("7", just_numeric(&String::from("treb7uchet")));
    }

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

    #[test]
    fn find_numbers_words_or_digit_finds_all_words() {
        assert_eq!(
            vec!["one", "two", "3"],
            find_number_words_or_digit(&String::from("onetwo3"))
        );
    }
}