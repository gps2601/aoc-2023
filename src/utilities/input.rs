use std::fs::read_to_string;

pub fn as_lines(file_path: &str) -> Vec<String> {
    return read_to_string(file_path)
        .expect("Should have been able to read the file")
        .lines().map(String::from)
        .collect();
}

pub fn all(file_path: &str) -> String {
    return read_to_string(file_path).expect("should exist");
}