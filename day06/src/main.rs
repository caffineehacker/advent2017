use clap::Parser;
use itertools::Itertools;
use std::{
    collections::HashSet,
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
}

fn main() {
    let args = Args::parse();

    let file = File::open(&args.data_file).expect("Failed to open file");
    let reader = BufReader::new(file);
    let lines: Vec<String> = reader
        .lines()
        .map(|line| line.expect("Failed to read line"))
        .collect();

    let mut banks = lines
        .get(0)
        .unwrap()
        .split_ascii_whitespace()
        .map(|bank| bank.parse::<i32>().unwrap())
        .collect_vec();

    let mut seen_states: HashSet<Vec<i32>> = HashSet::new();
    let mut iterations = 0;
    loop {
        if seen_states.contains(&banks) {
            println!("Part 1: {}", iterations);
            break;
        }

        iterations += 1;
        seen_states.insert(banks.clone());

        let max = banks.iter().max().unwrap();
        let (mut index, count) = banks.iter().find_position(|bank| *bank == max).unwrap();
        let mut count = *count;

        banks[index as usize] = 0;
        while count > 0 {
            index = (index + 1) % banks.len();
            banks[index as usize] += 1;
            count -= 1;
        }
    }
}
