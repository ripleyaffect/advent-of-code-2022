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
    let input_lines = utils::read_input_lines("06");
    let chars: Vec<char> = input_lines[0].chars().collect();
    get_start(chars, 4)
}

fn part_2() -> i32 {
  let input_lines = utils::read_input_lines("06");
  let chars: Vec<char> = input_lines[0].chars().collect();
  get_start(chars, 14)
}

fn get_start(chars: Vec<char>, needed: usize) -> i32 {
  let mut idx = 0;
  while !is_unique(&chars[idx..=idx+(needed-1)]) {
      idx += 1;
  }
  (idx + needed) as i32
}

fn is_unique(slice: &[char]) -> bool {
    let set: HashSet<&char> = HashSet::from_iter(slice);
    set.len().eq(&slice.len())
}
