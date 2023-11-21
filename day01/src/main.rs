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

    let chars: Vec<char> = lines.get(0).unwrap().chars().collect();

    let mut result = chars.iter().dedup_with_count().map(|(count, elem)| elem.to_string().parse::<u32>().unwrap() * (count as u32 - 1)).sum::<u32>();
    if chars[0] == chars[chars.len() - 1] {
        result += chars[0].to_string().parse::<u32>().unwrap()
    }
    println!("Result: {}", result)
}
