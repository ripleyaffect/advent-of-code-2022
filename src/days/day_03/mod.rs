use std::collections::HashSet;
use crate::utils;

pub fn get_part(part: u8) -> i32 {
    match part {
        1 => part_1(),
        2 => part_2(),
        _ => panic!("Days only have two parts!")
    }
}

fn part_1() -> i32 {
    let input_lines = utils::read_input_lines("03");

    let mut sum = 0;
    for line in input_lines {
        let mut arr = line.chars().collect::<Vec<char>>();
        let chunks: Vec<Vec<char>> = arr.chunks(arr.len() / 2).map(|s| s.into()).collect();

        let mut sets: Vec<HashSet<char>>  = vec![
            HashSet::from_iter(chunks[0].clone()),
            HashSet::from_iter(chunks[1].clone()),
        ];

        let intersection = utils::set_intersection(&mut sets);
        let item = intersection.drain().next().unwrap();

        sum += get_item_priority(item);
    }
    sum
}

fn part_2() -> i32 {
    let input_lines = utils::read_input_lines("03");

    let groups: Vec<Vec<String>> = input_lines.chunks(3).map(|s| s.into()).collect();

    let mut sum = 0;
    for group in groups {
        let mut sets: Vec<HashSet<char>>  = vec![
            HashSet::from_iter(group[0].chars()),
            HashSet::from_iter(group[1].chars()),
            HashSet::from_iter(group[2].chars()),
        ];

        let intersection = utils::set_intersection(&mut sets);
        let item = intersection.drain().next().unwrap();

        sum += get_item_priority(item);
    }
    sum
}

fn get_item_priority(item: char) -> i32 {
    (" abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ".find(item).unwrap()) as i32
}
