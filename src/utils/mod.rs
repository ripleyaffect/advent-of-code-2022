use std::hash::Hash;
use std::collections::HashSet;
use std::fs;

pub fn read_input_lines(day: &str) -> Vec<String> {
    let input_string = read_input_string(day);

    let mut input_string_lines: Vec<String> = Vec::new();
    for line in input_string.lines().collect::<Vec<&str>>() {
        input_string_lines.push(line.to_string())
    }
    input_string_lines
}

pub fn read_input_string(day: &str) -> String {
    fs::read_to_string(format!("src/days/day_{}/input.txt", day)).unwrap()
}

pub fn set_intersection<T: Eq + Hash>(sets: &mut Vec<HashSet<T>>) -> &mut HashSet<T> {
    let (intersection, others) = sets.split_at_mut(1);
    let intersection = &mut intersection[0];
    for other in others {
        intersection.retain(|e| other.contains(e));
    }
    intersection
}