use clap::Parser;
use itertools::Itertools;
use std::{
    collections::HashMap,
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

    // We're defining the grid to be (0, 0) has (-1, -1) to the NW, (0, -1) to the N, (1, -1) to the NE, (1, 0) to the SE, (0, 1) to the S, and (-1, 0) to the SW
    // Odd columns will instead have (-1, 0) NW, (0, -1) N, (1, 0) NE, (1, 1) SE, (0, 1) S, (-1, 1) SW

    let mut even_directions = HashMap::new();
    even_directions.insert("nw", (-1, -1));
    even_directions.insert("n", (0, -1));
    even_directions.insert("ne", (1, -1));
    even_directions.insert("se", (1, 0));
    even_directions.insert("s", (0, 1));
    even_directions.insert("sw", (-1, 0));
    let mut odd_directions = HashMap::new();
    odd_directions.insert("nw", (-1, 0));
    odd_directions.insert("n", (0, -1));
    odd_directions.insert("ne", (1, 0));
    odd_directions.insert("se", (1, 1));
    odd_directions.insert("s", (0, 1));
    odd_directions.insert("sw", (-1, 1));

    let mut position: (i32, i32) = (0, 0);
    for instruction in lines.get(0).unwrap().split(",") {
        if args.debug {
            print!("{}: {}, {} -> ", instruction, position.0, position.1);
        }
        if position.0 % 2 == 0 {
            let (x, y) = even_directions.get(instruction).unwrap();
            position.0 += x;
            position.1 += y;
        } else {
            let (x, y) = odd_directions.get(instruction).unwrap();
            position.0 += x;
            position.1 += y;
        }

        if args.debug {
            println!("{}, {}", position.0, position.1);
        }
    }

    println!("Final position: {}, {}", position.0, position.1);

    let mut move_count = 0;
    while position.0 != 0 || position.1 != 0 {
        if args.debug {
            print!("{}, {} -> ", position.0, position.1);
        }
        if position.0 % 2 == 0 {
            position = even_directions
                .values()
                .map(|(x, y)| (position.0 + x, position.1 + y))
                .min_by_key(|(x, y)| x.abs() + y.abs())
                .unwrap();
        } else {
            position = odd_directions
                .values()
                .map(|(x, y)| (position.0 + x, position.1 + y))
                .min_by_key(|(x, y)| x.abs() + y.abs())
                .unwrap();
        }
        move_count += 1;
        if args.debug {
            println!("{}, {}", position.0, position.1);
        }
    }

    println!("Part 1: {}", move_count);
}
