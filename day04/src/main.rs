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

    let valid_lines: Vec<&String> = lines
        .iter()
        .filter(|line| line.split_ascii_whitespace().all_unique())
        .collect();

    println!("Part 1: {}", valid_lines.len());

    let valid_lines_anagram: Vec<&String> = lines
        .iter()
        .filter(|line| {
            line.split_ascii_whitespace()
                .map(|word| word.chars().sorted().collect::<String>())
                .all_unique()
        })
        .collect();

    println!("Part 2: {}", valid_lines_anagram.len());
}
