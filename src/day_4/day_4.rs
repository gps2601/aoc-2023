use std::collections::{HashMap, VecDeque};
use crate::utilities::input;

#[allow(dead_code)]
fn part_1(file_path: &str) -> i32 {
    let input_lines = input::as_lines(file_path);
    let games: Vec<Game> = input_lines.iter().map(|line| to_game(line)).collect();
    let all_scores: Vec<i32> = games.iter().map(|game| game.score()).collect();
    all_scores.iter().sum()
}

#[allow(dead_code)]
fn part_2(file_path: &str) -> i32 {
    let games: Vec<Game> = starting_games(file_path);
    let mut need_to_process: VecDeque<Game> = VecDeque::new();
    for game in &games {
        need_to_process.push_back(game.clone())
    }
    let mut card_count_map: HashMap<i32, i32> = HashMap::new();
    while !need_to_process.is_empty() {
        let game_to_process = need_to_process.pop_front().expect("should exist");
        let count = card_count_map.entry(game_to_process.card_no).or_insert(0);
        *count += 1;
        let card = game_to_process.card_no;
        let number_of_wins = game_to_process.number_of_wins.expect("should exist");
        for i in 1..number_of_wins + 1 {
            let card_to_copy = games.get(card as usize + i - 1).expect("should exist");
            let copy = Game {
                card_no : card_to_copy.card_no,
                winning_numbers: card_to_copy.winning_numbers.clone(),
                player_numbers: card_to_copy.player_numbers.clone(),
                number_of_wins: card_to_copy.number_of_wins
            };
            need_to_process.push_back(copy);
        }
    }
    card_count_map.values().sum()
}

#[allow(dead_code)]
fn starting_games(file_path: &str) -> Vec<Game> {
    let input_lines = input::as_lines(file_path);
    let games: Vec<Game> = input_lines.iter().map(|line| to_game(line)).collect();
    games
}

#[allow(dead_code)]
fn to_game(line: &String) -> Game {
    // Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
    let card_to_numbers: Vec<&str> = line.split(": ").collect();
    let card_to_score = card_to_numbers.first().expect("should exist");
    let card_no: i32 = card_to_score.split(" ").last().expect("should exist").parse().unwrap();
    let winning_numbers_to_numbers: Vec<&str> = card_to_numbers.last().expect("should exist").split(" | ").collect();
    let winning_numbers: Vec<i32> = winning_numbers_to_numbers.first().expect("should exist").split_whitespace().map(|number_string| number_string.parse().unwrap()).collect();
    let player_numbers: Vec<i32> = winning_numbers_to_numbers.last().expect("should exist").split_whitespace().map(|number_string| number_string.parse().unwrap()).collect();
    let mut game = Game {
        card_no,
        winning_numbers,
        player_numbers,
        number_of_wins: None,
    };
    game.number_of_wins = Option::from(game.number_of_winning_numbers() as usize);
    return game
}

#[derive(PartialEq)]
#[derive(Debug)]
#[derive(Clone)]
struct Game {
    card_no: i32,
    winning_numbers: Vec<i32>,
    player_numbers: Vec<i32>,
    number_of_wins: Option<usize>
}

impl Game {
    fn number_of_winning_numbers(&self) -> i32 {
        let matches: Vec<&i32> = self.player_numbers.iter()
            .filter(|number| self.winning_numbers.contains(number))
            .collect();
        matches.iter().count() as i32
    }

    fn score(&self) -> i32 {
        let score = self.number_of_winning_numbers();
        if score == 0 { return 0; }
        let mut i = 1;
        let times_to_double = score - 1;
        for _ in 0..times_to_double {
            i *= 2;
        }
        return i
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_1_sample() {
        assert_eq!(13, part_1("src/day_4/day_4_sample.txt"))
    }

    #[test]
    fn part_1_full() {
        assert_eq!(22488, part_1("src/day_4/day_4.txt"))
    }

    #[test]
    fn part_2_sample() {
        assert_eq!(30, part_2("src/day_4/day_4_sample.txt"))
    }

    #[test]
    fn part_2_full() {
        assert_eq!(7013204, part_2("src/day_4/day_4.txt"))
    }

    #[test]
    fn can_convert_game() {
        let expected = Game {
            card_no: 1,
            winning_numbers: vec![41, 48, 83, 86, 17],
            player_numbers: vec![83, 86, 6, 31, 17, 9, 48, 53],
            number_of_wins: Some(4),
        };
        let result = to_game(&"Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53".to_string());
        assert_eq!(expected, result);

        let expected = Game {
            card_no: 2,
            winning_numbers: vec![13, 32, 20, 16, 61],
            player_numbers: vec![61, 30, 68, 82, 17, 32, 24, 19],
            number_of_wins: Some(2),
        };
        let result = to_game(&"Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19".to_string());
        assert_eq!(expected, result);

        let expected = Game {
            card_no: 3,
            winning_numbers: vec![1, 21, 53, 59, 44],
            player_numbers: vec![69, 82, 63, 72, 16, 21, 14, 1],
            number_of_wins: Some(2),
        };
        let result = to_game(&"Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1".to_string());
        assert_eq!(expected, result);
    }

    #[test]
    fn number_of_winning_numbers() {
        let game = Game {
            card_no: 1,
            winning_numbers: vec![1],
            player_numbers: vec![],
            number_of_wins: None,
        };

        let actual = game.number_of_winning_numbers();
        assert_eq!(0, actual);

        let game = Game {
            card_no: 1,
            winning_numbers: vec![1],
            player_numbers: vec![1],
            number_of_wins: None,
        };

        let actual = game.number_of_winning_numbers();
        assert_eq!(1, actual);

        let game = Game {
            card_no: 1,
            winning_numbers: vec![1, 2],
            player_numbers: vec![1],
            number_of_wins: None,
        };

        let actual = game.number_of_winning_numbers();
        assert_eq!(1, actual);

        let game = Game {
            card_no: 1,
            winning_numbers: vec![1, 2, 3],
            player_numbers: vec![1, 2],
            number_of_wins: None,
        };

        let actual = game.number_of_winning_numbers();
        assert_eq!(2, actual);
    }

    #[test]
    fn can_calculate_score() {
        let game = Game {
            card_no: 1,
            winning_numbers: vec![1],
            player_numbers: vec![],
            number_of_wins: None,
        };

        assert_eq!(0, game.score());

        let game = Game {
            card_no: 1,
            winning_numbers: vec![1],
            player_numbers: vec![1],
            number_of_wins: None,
        };

        assert_eq!(1, game.score());

        let game = Game {
            card_no: 1,
            winning_numbers: vec![1, 2],
            player_numbers: vec![1, 2],
            number_of_wins: None,
        };

        assert_eq!(2, game.score());

        let game = Game {
            card_no: 1,
            winning_numbers: vec![1, 2, 3],
            player_numbers: vec![1, 2, 3],
            number_of_wins: None,
        };

        assert_eq!(4, game.score());

        let game = Game {
            card_no: 1,
            winning_numbers: vec![1, 2, 3, 4],
            player_numbers: vec![1, 2, 3, 4],
            number_of_wins: None,
        };

        assert_eq!(8, game.score());
    }
}
