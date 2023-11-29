use clap::Parser;
use itertools::Itertools;
use petgraph::prelude::*;
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

    let mut g = DiGraph::new();
    let mut weights = HashMap::new();
    let mut node_indexes = HashMap::new();

    for line in lines {
        let parts = line.split_ascii_whitespace().collect_vec();
        if !node_indexes.contains_key(parts[0]) {
            node_indexes.insert(parts[0].to_string(), g.add_node(1));
        }
        for i in 3..parts.len() {
            let target = parts[i].trim_end_matches(",").to_string();
            if !node_indexes.contains_key(&target) {
                node_indexes.insert(target.clone(), g.add_node(1));
            }
            if args.debug {
                println!("{} -> {}", parts[0], target);
            }
            g.add_edge(node_indexes[parts[0]], node_indexes[&target], 1);
        }

        weights.insert(
            parts[0].to_string(),
            parts[1]
                .trim_start_matches("(")
                .trim_end_matches(")")
                .parse::<i32>()
                .unwrap(),
        );
    }

    let root = node_indexes
        .iter()
        .find(|(_, index)| g.edges_directed(**index, Direction::Incoming).count() == 0)
        .unwrap()
        .0;
    println!("Part 1: {}", root);
}
