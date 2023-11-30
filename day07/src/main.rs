use clap::Parser;
use itertools::Itertools;
use petgraph::{data::DataMap, prelude::*};
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
    let mut node_indexes = HashMap::new();

    for line in lines.iter() {
        let parts = line.split_ascii_whitespace().collect_vec();
        if !node_indexes.contains_key(parts[0]) {
            node_indexes.insert(
                parts[0].to_string(),
                g.add_node(
                    parts[1]
                        .trim_start_matches("(")
                        .trim_end_matches(")")
                        .parse::<i32>()
                        .unwrap(),
                ),
            );
        }
    }

    for line in lines {
        let parts = line.split_ascii_whitespace().collect_vec();
        for i in 3..parts.len() {
            let target = parts[i].trim_end_matches(",").to_string();
            if args.debug {
                println!("{} -> {}", parts[0], target);
            }
            g.add_edge(node_indexes[parts[0]], node_indexes[&target], 1);
        }
    }

    let root = node_indexes
        .iter()
        .find(|(_, index)| g.edges_directed(**index, Direction::Incoming).count() == 0)
        .unwrap();
    println!("Part 1: {}", root.0);
    get_weight(&g, *root.1, &node_indexes, args.debug);
}

fn get_weight(
    g: &Graph<i32, i32>,
    start: NodeIndex<u32>,
    node_indexes: &HashMap<String, NodeIndex>,
    debug: bool,
) -> Option<i32> {
    let mut total_weight = *g.node_weight(start).unwrap();
    let subnode_weights = g
        .edges_directed(start, Direction::Outgoing)
        .map(|edge| {
            (
                edge.target(),
                get_weight(g, edge.target(), node_indexes, debug),
            )
        })
        .collect_vec();

    if subnode_weights.iter().any(|w| w.1.is_none()) {
        return None;
    }

    let subnode_weights = subnode_weights
        .into_iter()
        .map(|(node, weight)| (node, weight.unwrap()))
        .collect_vec();

    let node = node_indexes.iter().find(|node| *node.1 == start).unwrap().0;
    if debug {
        println!("{}", node);
    }

    if !subnode_weights.iter().map(|w| w.1).all_equal() {
        let weight_counts = subnode_weights
            .iter()
            .map(|(_, weight)| *weight)
            .sorted()
            .dedup_with_count()
            .collect_vec();
        if debug {
            weight_counts
                .iter()
                .for_each(|(count, weight)| println!("{} x {}", weight, count));
        }
        let good_weight = weight_counts
            .iter()
            .max_by_key(|(count, _)| *count)
            .unwrap()
            .1;
        let bad_weight = weight_counts
            .iter()
            .min_by_key(|(count, _)| *count)
            .unwrap()
            .1;

        let bad_subnode = subnode_weights
            .iter()
            .find(|(_, weight)| *weight == bad_weight)
            .unwrap()
            .0;

        let bad_subnode_name = node_indexes
            .iter()
            .find(|node| *node.1 == bad_subnode)
            .unwrap()
            .0;
        let bad_subnode_weight = *g.node_weight(bad_subnode).unwrap();
        if debug {
            println!(
                "Bad subnode: {}: {} ({} / {})",
                bad_subnode_name, bad_subnode_weight, bad_weight, good_weight
            );
        }

        println!(
            "Part 2: {}",
            bad_subnode_weight + (good_weight - bad_weight)
        );
        return None;
    }

    total_weight += subnode_weights.iter().map(|weight| weight.1).sum::<i32>();
    if debug {
        println!("{}: {}", node, total_weight);
    }
    return Some(total_weight);
}
