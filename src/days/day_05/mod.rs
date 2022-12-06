use regex::Regex;
use crate::utils;

pub fn get_part(part: u8) -> i32 {
    match part {
        1 => part_1(),
        2 => part_2(),
        _ => panic!("Days only have two parts!")
    }
}

fn part_1() -> i32 {
    let mut stacks = get_initial_stacks();

    let input_lines = utils::read_input_lines("05");
    for line in input_lines {
        let instruction = Instruction::from(parse_line(&line));
        perform_instruction(&mut stacks, instruction);
    }
    print_stack_tops(&stacks);

    0
}

fn part_2() -> i32 {
    let mut stacks = get_initial_stacks();

    let input_lines = utils::read_input_lines("05");
    for line in input_lines {
        let instruction = Instruction::from(parse_line(&line));
        perform_instruction_two(&mut stacks, instruction);
    }
    print_stack_tops(&stacks);

    0
}

struct Instruction {
    amount: usize,
    from: usize,
    to: usize,
}

impl Instruction {
    fn from(vals: (usize, usize, usize)) -> Instruction {
        Instruction {
            amount: vals.0,
            from: vals.1,
            to: vals.2,
        }
    }
}

// [D]
// [N] [C]
// [Z] [M] [P]
//  1   2   3
fn get_test_stacks() -> Vec<Vec<char>> {
    let mut stacks = Vec::new();

    stacks.push(vec![]);
    stacks.push("ZN".chars().collect());
    stacks.push("MCD".chars().collect());
    stacks.push("P".chars().collect());

    stacks
}

//                [J] [Z] [G]
//                [Z] [T] [S] [P] [R]
//    [R]         [Q] [V] [B] [G] [J]
//    [W] [W]     [N] [L] [V] [W] [C]
//    [F] [Q]     [T] [G] [C] [T] [T] [W]
//    [H] [D] [W] [W] [H] [T] [R] [M] [B]
//    [T] [G] [T] [R] [B] [P] [B] [G] [G]
//    [S] [S] [B] [D] [F] [L] [Z] [N] [L]
//     1   2   3   4   5   6   7   8   9
fn get_initial_stacks() -> Vec<Vec<char>> {
    let mut stacks = Vec::new();

    stacks.push(vec![]);
    stacks.push("STHFWR".chars().collect());
    stacks.push("SGDQW".chars().collect());
    stacks.push("BTW".chars().collect());
    stacks.push("DRWTNQZJ".chars().collect());
    stacks.push("FBHGLVTZ".chars().collect());
    stacks.push("LPTCVBSG".chars().collect());
    stacks.push("ZBRTWGP".chars().collect());
    stacks.push("NGMTCJR".chars().collect());
    stacks.push("LGBW".chars().collect());

    stacks
}

fn parse_line(line: &String) -> (usize, usize, usize) {
    let re = Regex::new(r"move (\d+) from (\d+) to (\d+)").unwrap();
    let cap = re.captures(line).unwrap();
    (
        cap[1].parse().unwrap(),
        cap[2].parse().unwrap(),
        cap[3].parse().unwrap(),
    )
}

fn perform_instruction(stacks: &mut Vec<Vec<char>>, instruction: Instruction) -> &mut Vec<Vec<char>> {
    let mut from_stack = stacks.get(instruction.from).cloned().unwrap();
    let mut chars: Vec<char> = from_stack.iter().rev().take(instruction.amount).cloned().collect();
    from_stack.truncate(from_stack.len() - instruction.amount);
    stacks[instruction.from] = from_stack;

    let to_stack = stacks.get_mut(instruction.to).unwrap();
    to_stack.append(&mut chars);

    stacks
}

fn perform_instruction_two(stacks: &mut Vec<Vec<char>>, instruction: Instruction) -> &mut Vec<Vec<char>> {
    let mut from_stack = stacks.get(instruction.from).cloned().unwrap();
    let mut chars: Vec<char> = from_stack.iter().rev().take(instruction.amount).rev().cloned().collect();
    from_stack.truncate(from_stack.len() - instruction.amount);
    stacks[instruction.from] = from_stack;

    let to_stack = stacks.get_mut(instruction.to).unwrap();
    to_stack.append(&mut chars);

    stacks
}

fn print_stack_tops(stacks: &Vec<Vec<char>>) {
    let mut chars: Vec<&char> = Vec::new();
    for stack in stacks[1..].iter() {
        chars.push(stack.last().unwrap())
    }
    println!("{:?}", chars.into_iter().collect::<String>());
}
