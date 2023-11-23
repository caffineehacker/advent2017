use std::collections::HashMap;

use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(long)]
    puzzle: i32,
    #[arg(long)]
    debug: bool,
}

fn main() {
    let args = Args::parse();

    let mut current_ring_length = 0;
    let mut total_length = 1;
    let mut long_side = 1;

    while total_length < args.puzzle {
        current_ring_length += 8;
        total_length += current_ring_length;
        long_side += 2;
    }

    if args.puzzle > total_length - long_side {
        let y = current_ring_length / 8;
        let x = ((long_side - 1) / 2) - (total_length - args.puzzle);
        println!("Part 1: {}", y.abs() + x.abs());
    } else if args.puzzle >= total_length - ((long_side - 1) * 2) {
        let x = current_ring_length / 8;
        let bottom_left = total_length - long_side + 1;
        let y = ((long_side - 1) / 2) - (bottom_left - args.puzzle);
        println!("Part 1: {}", y.abs() + x.abs());
    } else if args.puzzle >= total_length - ((long_side - 1) * 3) {
        let y = current_ring_length / 8;
        let top_left = total_length - ((long_side - 1) * 2);
        let x = ((long_side - 1) / 2) - (top_left - args.puzzle);
        println!("Part 1: {}", y.abs() + x.abs());
    } else {
        let x = current_ring_length / 8;
        let top_right = total_length - current_ring_length + long_side - 1;
        let y = ((long_side - 1) / 2) - (top_right - args.puzzle);
        println!("Part 1: {}", y.abs() + x.abs());
    }

    if args.debug {
        println!("Ring length: {}", total_length);
    }

    // Naive approach since I think the numbers will grow quickly
    long_side = 1;
    let mut direction = (1, 0);
    let mut values: HashMap<(i32, i32), i32> = HashMap::new();
    let mut position = (0, 0);
    values.insert(position, 1);
    loop {
        position.0 += direction.0;
        position.1 += direction.1;

        if direction.0 == 1 && position.0 == (long_side - 1) / 2 + 1 {
            direction.0 = 0;
            direction.1 = -1;
            long_side += 2;
        } else if direction.1 == -1 && position.1 == (long_side - 1) / -2 {
            direction.0 = -1;
            direction.1 = 0;
        } else if direction.0 == -1 && position.0 == (long_side - 1) / -2 {
            direction.0 = 0;
            direction.1 = 1;
        } else if direction.1 == 1 && position.1 == (long_side - 1) / 2 {
            direction.0 = 1;
            direction.1 = 0;
        }

        let mut sum = 0;
        for x in -1..=1 {
            for y in -1..=1 {
                if values.contains_key(&(position.0 + x, position.1 + y)) {
                    sum += values.get(&(position.0 + x, position.1 + y)).unwrap();
                }
            }
        }

        if args.debug {
            println!("({}, {}): {}", position.0, position.1, sum);
        }

        if sum > args.puzzle {
            println!("Part 2: {}", sum);
            return;
        }

        values.insert(position, sum);
    }
}
