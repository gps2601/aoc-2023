use crate::utilities::input;

#[allow(dead_code)]
fn part_1(file_path: &str) -> i32 {
    let _input_lines = input::as_lines(file_path);
    1
}


#[allow(dead_code)]
fn part_2(file_path: &str) -> i32 {
    let _input_lines = input::as_lines(file_path);
    1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_1_sample() {
        assert_eq!(1, part_1("src/day_5/day_5_sample.txt"))
    }

    #[test]
    fn part_1_full() {
        assert_eq!(1, part_1("src/day_5/day_5.txt"))
    }

    #[test]
    fn part_2_sample() {
        assert_eq!(1, part_2("src/day_5/day_5_sample.txt"))
    }

    #[test]
    fn part_2_full() {
        assert_eq!(1, part_2("src/day_5/day_5.txt"))
    }
}
