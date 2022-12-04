use crate::utils;

pub fn get_part(part: u8) -> i32 {
    match part {
        1 => part_1(),
        2 => part_2(),
        _ => panic!("Days only have two parts!")
    }
}

fn part_1() -> i32 {
    let input_lines = utils::read_input_lines("04");

    let mut count = 0;
    for line in input_lines {
        let vals = parse_line(&line);
        if get_contains(&vals[0], &vals[1]) || get_contains(&vals[1], &vals[0]) { count += 1; }
    }

    count
}

fn part_2() -> i32 {
    let input_lines = utils::read_input_lines("04");

    let mut count = 0;
    for line in input_lines {
        let vals = parse_line(&line);
        if get_overlaps(&vals[0], &vals[1]) || get_contains(&vals[1], &vals[0]) { count += 1; }
    }

    count
}

fn parse_line(line: &String) -> Vec<Vec<i32>> {
    line.split(",")
    .map(|n| n.split("-").map(|string_num| string_num.to_string().parse().expect("parse error")).collect())
    .collect()
}

fn get_contains(a: &Vec<i32>, b: &Vec<i32>) -> bool {
    a[0] <= b[0] && a[1] >= b[1]
}

fn get_overlaps(a: &Vec<i32>, b: &Vec<i32>) -> bool {
    (a[0] <= b[0] && a[1] >= b[0]) || (a[0] <= b[1] && a[1] >= b[1])
}