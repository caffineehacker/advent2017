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

    let input = lines.get(0).unwrap().chars().collect_vec();
    let mut depth = 0;
    let mut score = 0;
    let mut in_garbage = false;

    let mut index = 0;
    while index < input.len() {
        if in_garbage {
            match input[index] {
                '!' => {
                    index += 1;
                }
                '>' => {
                    in_garbage = false;
                }
                _ => {}
            }
        } else {
            match input[index] {
                '{' => {
                    depth += 1;
                }
                '}' => {
                    score += depth;
                    depth -= 1;
                }
                '!' => {
                    index += 1;
                }
                '<' => {
                    in_garbage = true;
                }
                _ => {}
            };
        }

        index += 1;
    }

    println!("Part 1: {}", score);
}
