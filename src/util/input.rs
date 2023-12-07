use std::fs;

pub fn get_input_path(day: i32) -> String {
    format!("input/day{}.txt", day)
}

pub fn get_input(day: i32) -> String {
    let path = get_input_path(day);
    fs::read_to_string(path)
        .expect("Something went wrong reading the file")
}

pub fn input_to_lines(input: &str) -> Vec<String> {
    input
        .lines()
        .filter_map(|s| if s.is_empty() { None } else { Some(s.to_string()) })
        .collect()
}