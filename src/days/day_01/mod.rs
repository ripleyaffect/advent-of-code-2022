use crate::utils;

pub fn get_part(part: u8) -> i32 {
    match part {
        1 => part_1(),
        2 => part_2(),
        _ => panic!("Days only have two parts!")
    }
}

fn part_1() -> i32 {
    let input_lines = utils::read_input_lines("01");

    let collapsed = collapse_lines(&input_lines);

    *collapsed.iter().max().unwrap()
}

fn part_2() -> i32 {
    let input_lines = utils::read_input_lines("01");

    let mut collapsed = collapse_lines(&input_lines);

    collapsed.sort();
    collapsed.reverse();

    let sum = collapsed.drain(..3).into_iter().sum::<i32>();

    sum
}

fn collapse_lines(lines: &Vec<String>) -> Vec<i32> {
    let mut collapsed_lines: Vec<i32> = Vec::new();
    let mut sum = 0;

    for line in lines {
        if line.is_empty() {
            collapsed_lines.push(sum);
            sum = 0;
        } else {
            sum += line.parse::<i32>().unwrap();
        }
    }

    collapsed_lines
}
