use std::fs;

pub fn read_input(day: i32) -> String {
    fs::read_to_string(format!("inputs/day{}.txt", day))
        .unwrap()
        .trim()
        .to_string()
}
