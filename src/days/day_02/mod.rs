use crate::utils;

enum RPS {
    R,
    P,
    S,
}

pub fn get_part(part: u8) -> i32 {
    match part {
        1 => part_1(),
        2 => part_2(),
        _ => panic!("Days only have two parts!")
    }
}

fn part_1() -> i32 {
    let input_lines = utils::read_input_lines("02");

    let rounds = transform_line(&input_lines);

    let mut sum = 0;
    for round in rounds {
        let round_points = get_round_points(&round.0, &round.1);
        sum += round_points;
    }
    sum
}

fn part_2() -> i32 {
    let input_lines = utils::read_input_lines("02");

    let rounds = transform_line_two(&input_lines);

    let mut sum = 0;
    for round in rounds {
        let round_points = get_round_points(&round.0, &round.1);
        sum += round_points;
    }
    sum
}

fn transform_line(line: &Vec<String>) -> Vec<(RPS, RPS)> {
    let mut rounds: Vec<(RPS, RPS)> = Vec::new();
    for line in line {
        let split: Vec<char> = line.chars().collect();
        rounds.push((char_to_rps(&split[0]), char_to_rps(&split[2])));
    }
    rounds
}

fn char_to_rps(c: &char) -> RPS {
    match c {
        'A' => RPS::R,
        'B' => RPS::P,
        'C' => RPS::S,
        'X' => RPS::R,
        'Y' => RPS::P,
        'Z' => RPS::S,
        _ => panic!("Character not recognized"),
    }
}

fn transform_line_two(line: &Vec<String>) -> Vec<(RPS, RPS)> {
    let mut rounds: Vec<(RPS, RPS)> = Vec::new();
    for line in line {
        let split: Vec<char> = line.chars().collect();
        rounds.push(char_to_rps_two(&split[0], &split[2]));
    }
    rounds
}

fn char_to_rps_two(a: &char, b: &char) -> (RPS, RPS) {
    let opp  = char_to_rps(a);
    let you = match a {
        'A' => match b {
            'X' => RPS::S,
            'Y' => RPS::R,
            'Z' => RPS::P,
            _ => panic!("Character not recognized"),
        },
        'B' => match b {
            'X' => RPS::R,
            'Y' => RPS::P,
            'Z' => RPS::S,
            _ => panic!("Character not recognized"),
        },
        'C' => match b {
            'X' => RPS::P,
            'Y' => RPS::S,
            'Z' => RPS::R,
            _ => panic!("Character not recognized"),
        },
        _ => panic!("Character not recognized"),
    };

    (opp, you)
}

fn get_round_points(a: &RPS, b: &RPS) -> i32 {
    let base = match b {
        RPS::R => 1,
        RPS::P => 2,
        RPS::S => 3,
    };

    let score = match b {
        RPS::R => match a {
            RPS::R => 3,
            RPS::P => 0,
            RPS::S => 6,
        },
        RPS::P => match a {
            RPS::R => 6,
            RPS::P => 3,
            RPS::S => 0,
        },
        RPS::S => match a {
            RPS::R => 0,
            RPS::P => 6,
            RPS::S => 3,
        },
    };

    base + score
}
