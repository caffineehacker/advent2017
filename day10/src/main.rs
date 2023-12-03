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

    let instructions = lines
        .get(0)
        .unwrap()
        .split(',')
        .map(|instruction| instruction.parse::<usize>().unwrap())
        .collect_vec();
    let part1_string = do_knot(instructions, 1, args.debug);

    if args.debug {
        part1_string.iter().for_each(|n| print!("{}, ", n));
        println!("");
    }

    println!("Part 1: {}", part1_string[0] * part1_string[1]);

    let mut part2_instructions = lines
        .get(0)
        .unwrap()
        .chars()
        .map(|c| c as u8 as usize)
        .collect_vec();
    part2_instructions.append(&mut vec![17, 31, 73, 47, 23]);
    let part2_string = do_knot(part2_instructions, 64, args.debug);
    let mut dense_hash = Vec::new();
    for i in 0..16 {
        let mut value = 0;
        for j in 0..16 {
            value ^= part2_string[i * 16 + j];
        }
        dense_hash.push(value);
    }

    print!("Part 2: ");
    for entry in dense_hash.iter() {
        print!("{:2x}", entry);
    }
    println!("");
}

fn do_knot(instructions: Vec<usize>, rounds: i32, debug: bool) -> Vec<i32> {
    let mut string = (0..256).collect_vec();
    let mut index = 0;
    let mut skip = 0;

    for _ in 0..rounds {
        for instruction in instructions.iter() {
            for i in 0..(instruction / 2) {
                string.swap((i + index) % 256, (instruction - i - 1 + index) % 256);
            }

            if debug {
                println!("Instr: {}, skip: {}, index: {}", instruction, skip, index);
                string.iter().for_each(|n| print!("{}, ", n));
                println!("");
            }

            index += instruction + skip;
            index %= 256;
            skip += 1;
        }
    }

    string
}
