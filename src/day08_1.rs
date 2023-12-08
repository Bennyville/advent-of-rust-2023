use std::collections::{HashMap, BTreeMap};
use std::fs::read_to_string;
use std::io;

use regex::Regex;

pub fn solve() -> Result<(), io::Error> {
    let contents = read_to_string("day08.txt")?;

    let (directions, nodes) = contents.split_once("\n\n").unwrap();

    let directions: Vec<char> = directions.chars().collect();
    let nodes: BTreeMap<&str, (&str, &str)> = nodes
        .lines()
        .map(|f| {
            let (node, neighbours) = f
                .split_once("=")
                .map(|(node, neighbours)| (node.trim(), neighbours.trim()))
                .unwrap();
            let re = Regex::new(r"^\(([A-Z]{3}),\s([A-Z]{3})\)$").unwrap();
            
            let captures = re.captures(neighbours).unwrap();

            let left = captures.get(1).unwrap().as_str();
            let right = captures.get(2).unwrap().as_str();

            (node, (left, right))
        })
        .collect();

    let mut steps = 0;
    let mut found = false;
    let mut current_node = "";

    while !found {
        for dir in directions.clone() {
            if current_node.is_empty() {
                current_node = nodes.keys().next().unwrap();
            }
            current_node = match dir {
                'L' => nodes.get(current_node).unwrap().0,
                'R' => nodes.get(current_node).unwrap().1,
                _ => current_node
            };
            steps += 1;
            if current_node == "ZZZ" {
                found = true;
                break;
            }
        }
    }

    println!("{}", steps);

    Ok(())
}
