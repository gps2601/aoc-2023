use crate::custom_error::AocError;

#[tracing::instrument]
pub fn process() -> miette::Result<String, AocError> {
    let race_1 = Race {
        time: 45,
        distance: 305,
    };

    let race_2 = Race {
        time: 97,
        distance: 1062,
    };

    let race_3 = Race {
        time: 72,
        distance: 1110,
    };

    let race_4 = Race {
        time: 95,
        distance: 1695,
    };

    let race_win_numbers: [i64; 4] = [race_1, race_2, race_3, race_4].map(number_of_ways_to_win);
    let multiplied: i64 = race_win_numbers.iter().product();
    Ok(multiplied.to_string())
}

struct Race {
    time: i64,
    distance: i64,
}

fn number_of_ways_to_win(race: Race) -> i64 {
    let mut no_wins: i64 = 0;
    for i in 0..race.time + 1 {
        let speed = i;
        let time_to_race = race.time - i;
        let distance_covered = speed * time_to_race;
        if distance_covered > race.distance {
            no_wins += 1
        }
    }
    no_wins
}

#[cfg(test)]
mod tests {
}