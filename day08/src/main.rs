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

    let mut registers: HashMap<String, i32> = HashMap::new();

    for line in lines.iter() {
        let (
            register,
            instruction,
            amount,
            _, // if
            control_register,
            comparison_operation,
            comparison_value,
        ) = line.split_ascii_whitespace().collect_tuple().unwrap();

        if !registers.contains_key(register) {
            registers.insert(register.to_string(), 0);
        }

        if !registers.contains_key(control_register) {
            registers.insert(control_register.to_string(), 0);
        }

        let comparison_register_value = *registers.get(control_register).unwrap();
        let comparison_value = comparison_value.parse::<i32>().unwrap();

        let should_execute = match comparison_operation {
            ">" => comparison_register_value > comparison_value,
            "<" => comparison_register_value < comparison_value,
            ">=" => comparison_register_value >= comparison_value,
            "<=" => comparison_register_value <= comparison_value,
            "==" => comparison_register_value == comparison_value,
            "!=" => comparison_register_value != comparison_value,
            _ => panic!("Bad comparison"),
        };

        if should_execute {
            let amount = amount.parse::<i32>().unwrap();
            let amount = if instruction == "dec" {
                -amount
            } else {
                amount
            };

            *registers.get_mut(register).unwrap() += amount;
        }
    }

    println!("Part 1: {}", registers.values().max().unwrap());
}
