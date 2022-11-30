#![allow(dead_code)]

mod days;

use clap::Parser;

#[derive(Parser, Debug)]
struct Args {
    #[arg(short, long)]
    day: u8,

    #[arg(short, long)]
    part: u8,
}

fn main() {
    let args = Args::parse();
    println!("{}", days::get_answer(args.day, args.part));
}
