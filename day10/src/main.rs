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

    let mut string = (0..256).collect_vec();
    let mut index = 0;
    let mut skip = 0;

    for instruction in lines.get(0).unwrap().split(',') {
        let instruction = instruction.parse::<usize>().unwrap();
        for i in 0..(instruction / 2) {
            string.swap((i + index) % 256, (instruction - i - 1 + index) % 256);
        }

        if args.debug {
            println!("Instr: {}, skip: {}, index: {}", instruction, skip, index);
            string.iter().for_each(|n| print!("{}, ", n));
            println!("");
        }

        index += instruction + skip;
        index %= 256;
        skip += 1;
    }

    if args.debug {
        string.iter().for_each(|n| print!("{}, ", n));
        println!("");
    }

    println!("Part 1: {}", string[0] * string[1])
}
