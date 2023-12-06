use rayon::iter::*;
use crate::custom_error::AocError;

#[tracing::instrument]
pub fn process() -> miette::Result<String, AocError> {
    let race_1 = Race {
        time: 45977295,
        distance: 305106211101695,
    };
    Ok(number_of_ways_to_win(race_1).to_string())
}

struct Race {
    time: u64,
    distance: u64,
}

fn number_of_ways_to_win(race: Race) -> u64 {
    (0..race.time)
        .into_par_iter()
        .filter(|&i| {
            let time_to_race = race.time - i;
            let distance_covered = i * time_to_race;
            distance_covered > race.distance
        })
        .count() as u64
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() {
        let race = Race { time: 71530, distance: 940200 };
        assert_eq!(71503, number_of_ways_to_win(race));
    }
}