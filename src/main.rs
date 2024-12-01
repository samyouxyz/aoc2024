mod days;
use std::fs;

use clap::Parser;

fn read_input(day: u8) -> String {
    let file_path = format!("inputs/day{:02}.txt", day);
    fs::read_to_string(file_path).expect("Error reading the file")
}

#[derive(Parser)]
struct Args {
    day: u8,
}

fn main() {
    let args = Args::parse();
    let input = read_input(args.day);

    match args.day {
        1 => {
            println!("{}", days::day01::solve_part1(&input));
            println!("{}", days::day01::solve_part2(&input));
        }
        _ => eprintln!("Solution for day {} is not implemented!", args.day),
    }
}
