use std::fs;

pub fn as_lines(file_path: &str) -> Vec<String> {
    let lines = fs::read_to_string(file_path)
        .expect("Should have been able to read the file")
        .lines().map(String::from)
        .collect();
    return lines;
}