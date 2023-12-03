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

    let mut memoized_distances: HashMap<(i32, i32), i32> = HashMap::new();
    let mut position: (i32, i32) = (0, 0);
    let mut max_distance = 0;
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

        let new_distance = get_distance(
            position,
            args.debug,
            &even_directions,
            &odd_directions,
            &mut memoized_distances,
        );
        if new_distance > max_distance {
            max_distance = new_distance;
        }

        if args.debug {
            println!("{}, {}", position.0, position.1);
        }
    }

    println!("Final position: {}, {}", position.0, position.1);

    let part1 = get_distance(
        position,
        true,
        &even_directions,
        &odd_directions,
        &mut memoized_distances,
    );

    println!("Part 1: {}", part1);
    println!("Part 2: {}", max_distance);
}

fn get_distance(
    position: (i32, i32),
    debug: bool,
    even_directions: &HashMap<&str, (i32, i32)>,
    odd_directions: &HashMap<&str, (i32, i32)>,
    memoized_distances: &mut HashMap<(i32, i32), i32>,
) -> i32 {
    let mut move_count = 0;
    let mut position = position;
    let mut trail = Vec::new();
    while position.0 != 0 || position.1 != 0 {
        if memoized_distances.contains_key(&position) {
            move_count += memoized_distances.get(&position).unwrap();
            break;
        }
        if debug {
            print!("{}, {} -> ", position.0, position.1);
        }
        trail.push(position);
        if position.0 % 2 == 0 {
            position = even_directions
                .values()
                .map(|(x, y)| (position.0 + x, position.1 + y))
                .min_by_key(|(x, y)| x.abs() * x.abs() + y.abs() + y.abs())
                .unwrap();
        } else {
            position = odd_directions
                .values()
                .map(|(x, y)| (position.0 + x, position.1 + y))
                .min_by_key(|(x, y)| x.abs() * x.abs() + y.abs() * y.abs())
                .unwrap();
        }
        move_count += 1;
        if debug {
            println!("{}, {}", position.0, position.1);
        }
    }

    for i in 0..trail.len() {
        if memoized_distances.contains_key(trail.get(i).unwrap()) {
            break;
        }

        memoized_distances.insert(*trail.get(i).unwrap(), move_count - i as i32);
    }

    move_count
}
