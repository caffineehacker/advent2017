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

    println!("Ring length: {}", total_length);
}
