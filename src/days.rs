mod day_01;
mod day_02;
mod day_03;
mod day_04;
mod day_05;
mod day_06;
mod day_07;
mod day_08;
mod day_09;
mod day_10;
mod day_11;
mod day_12;
mod day_13;
mod day_14;
mod day_15;
mod day_16;
mod day_17;
mod day_18;
mod day_19;
mod day_20;
mod day_21;
mod day_22;
mod day_23;
mod day_24;
mod day_25;

use std::collections::HashMap;

type IGetPart = fn(u8) -> i32;

pub fn get_answer(day: u8, part: u8) -> String {
    let days = get_days();
    let get_part = days.get(&day);

    if let None = get_part {
        return format!("No day {}, exiting", day);
    }

    get_part.unwrap()(part).to_string()
}

fn get_days() -> HashMap<u8, IGetPart> {
    let mut map: HashMap<u8, IGetPart> = HashMap::new();
    map.insert(01, day_01::get_part);
    map.insert(02, day_02::get_part);
    map.insert(03, day_03::get_part);
    map.insert(04, day_04::get_part);
    map.insert(05, day_05::get_part);
    map.insert(06, day_06::get_part);
    map.insert(07, day_07::get_part);
    map.insert(08, day_08::get_part);
    map.insert(09, day_09::get_part);
    map.insert(10, day_10::get_part);
    map.insert(11, day_11::get_part);
    map.insert(12, day_12::get_part);
    map.insert(13, day_13::get_part);
    map.insert(14, day_14::get_part);
    map.insert(15, day_15::get_part);
    map.insert(16, day_16::get_part);
    map.insert(17, day_17::get_part);
    map.insert(18, day_18::get_part);
    map.insert(19, day_19::get_part);
    map.insert(20, day_20::get_part);
    map.insert(21, day_21::get_part);
    map.insert(22, day_22::get_part);
    map.insert(23, day_23::get_part);
    map.insert(24, day_24::get_part);
    map.insert(25, day_25::get_part);
    return map;
}
