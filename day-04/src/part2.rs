use std::collections::{HashMap, VecDeque};
use crate::custom_error::AocError;

#[tracing::instrument]
pub fn process(
    input: &str,
) -> miette::Result<String, AocError> {
    let input_lines : Vec<&str>= input.lines().collect();
    let games: Vec<Game> = input_lines.iter().map(|line| to_game(line)).collect();
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
    Ok(card_count_map.values().sum::<i32>().to_string())
}

fn to_game(line: &str) -> Game {
    // Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
    let card_to_numbers: Vec<&str> = line.split(": ").collect();
    let card_to_score = card_to_numbers.first().expect("should exist");
    let card_no: i32 = card_to_score.split(' ').last().expect("should exist").parse().unwrap();
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
    game
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
        matches.len() as i32
    }

}