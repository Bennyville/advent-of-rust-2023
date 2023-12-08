use std::collections::{HashMap, HashSet};
use std::fs::read_to_string;
use std::{io, result, usize};

pub fn solve() -> Result<(), io::Error> {
    let order = HashMap::from([
        ('2', 2),
        ('3', 3),
        ('4', 4),
        ('5', 5),
        ('6', 6),
        ('7', 7),
        ('8', 8),
        ('9', 9),
        ('T', 10),
        ('J', 11),
        ('Q', 12),
        ('K', 13),
        ('A', 14),
    ]);

    let contents = read_to_string("day07.txt")?;

    let mut hands: Vec<_> = contents
        .lines()
        .map(|f| {
            let (hand, bid) = f.split_once(" ").unwrap();
            let bid = bid.parse::<usize>().unwrap();

            let cards: HashSet<_> = hand.clone().chars().into_iter().collect();

            let hand_values: Vec<usize> = hand
                .clone()
                .chars()
                .map(|f| order.get(&f).unwrap().clone())
                .collect();

            let mut card_occ = cards
                .iter()
                .map(|h| hand.chars().filter(|p| p == h).count())
                .collect::<Vec<usize>>();

            card_occ.sort_unstable();

            let hand_type = match card_occ.as_slice() {
                [1, 1, 1, 1, 1] => 0,
                [1, 1, 1, 2] => 1,
                [1, 2, 2] => 2,
                [1, 1, 3] => 3,
                [2, 3] => 4,
                [1, 4] => 5,
                [5] => 6,
                _ => 0,
            };

            (hand_type, hand_values, bid)
        })
        .collect();

    hands.sort_by(|a, b| {
        match a.0.cmp(&b.0) {
            std::cmp::Ordering::Equal => compare_vectors(&a.1, &b.1),
            other => other,
        }
    });

    let result = hands
        .iter()
        .enumerate()
        .map(|(rank, (_, _, bid))| (rank + 1) * bid)
        .sum::<usize>();

    println!("{:?}", result);

    Ok(())
}

fn compare_vectors(a: &Vec<usize>, b: &Vec<usize>) -> std::cmp::Ordering {
    for (a_elem, b_elem) in a.iter().zip(b.iter()) {
        match a_elem.cmp(b_elem) {
            std::cmp::Ordering::Equal => continue,
            other => return other,
        }
    }
    std::cmp::Ordering::Equal
}
