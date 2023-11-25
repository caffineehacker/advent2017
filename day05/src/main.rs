use clap::Parser;
use itertools::Itertools;
use std::{
    fs::File,
    io::{BufRead, BufReader},
};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(long)]
    data_file: String,
    #[arg(long)]
    debug: bool,
    #[arg(long)]
    part2: bool,
}

fn main() {
    let args = Args::parse();

    let file = File::open(&args.data_file).expect("Failed to open file");
    let reader = BufReader::new(file);
    let lines: Vec<String> = reader
        .lines()
        .map(|line| line.expect("Failed to read line"))
        .collect();

    let mut jumps: Vec<i32> = lines
        .iter()
        .map(|line| line.parse::<i32>().unwrap())
        .collect_vec();

    let mut index: i32 = 0;
    let mut step_number = 0;
    loop {
        if index < 0 || index >= (jumps.len() as i32) {
            println!("Answer: {}", step_number);
            break;
        }

        let jump = jumps[index as usize];
        if args.part2 && jumps[index as usize] >= 3 {
            jumps[index as usize] -= 1;
        } else {
            jumps[index as usize] += 1;
        }
        if args.debug {
            println!("{} -> {}", index, index + jump);
        }
        index += jump;
        step_number += 1;
    }
}
