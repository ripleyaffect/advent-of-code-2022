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