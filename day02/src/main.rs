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
}

fn main() {
    let args = Args::parse();

    let file = File::open(&args.data_file).expect("Failed to open file");
    let reader = BufReader::new(file);
    let lines: Vec<String> = reader
        .lines()
        .map(|line| line.expect("Failed to read line"))
        .collect();

    let split_lines: Vec<Vec<i32>> = lines
        .into_iter()
        .map(|line| {
            line.split_ascii_whitespace()
                .map(|entry| entry.parse::<i32>().unwrap())
                .collect_vec()
        })
        .collect();

    let result1: i32 = split_lines
        .iter()
        .map(|line| line.iter().max().unwrap() - line.iter().min().unwrap())
        .sum();
    println!("Part 1: {}", result1);

    let result2: i32 = split_lines
        .iter()
        .map(|line| {
            let good_pair = line
                .iter()
                .permutations(2)
                .find(|pair| pair[0].max(pair[1]) % pair[0].min(pair[1]) == 0)
                .unwrap();
            good_pair[0].max(good_pair[1]) / good_pair[0].min(good_pair[1])
        })
        .sum();
    println!("Part 2: {}", result2)
}
