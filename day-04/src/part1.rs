use crate::custom_error::AocError;

#[tracing::instrument]
pub fn process(
    input: &str,
) -> miette::Result<String, AocError> {
    let games: Vec<Game> = input.lines().map(to_game).collect();
    let all_scores: Vec<i32> = games.iter().map(|game| game.score()).collect();
    Ok(all_scores.iter().sum::<i32>().to_string())
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

    fn score(&self) -> i32 {
        let score = self.number_of_winning_numbers();
        if score == 0 { return 0; }
        let mut i = 1;
        let times_to_double = score - 1;
        for _ in 0..times_to_double {
            i *= 2;
        }
        i
    }
}
