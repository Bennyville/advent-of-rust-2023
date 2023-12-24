use std::fs::read_to_string;
use std::{char, io, usize};

pub fn solve() -> Result<(), io::Error> {
    let contents = read_to_string("input/day11.txt")?;

    let mut map: Vec<Vec<char>> = contents
        .lines()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect();

    let empty_rows: Vec<usize> = map
        .iter()
        .enumerate()
        .filter(|(i, _)| map[*i].iter().all(|p| *p == '.'))
        .map(|(i, _)| i)
        .collect();

    let empty_cols: Vec<usize> = map
        .iter()
        .enumerate()
        .filter(|(i, _)| map.iter().all(|p| p[*i] == '.'))
        .map(|(i, _)| i)
        .collect();

    let mut offset = 0;
    for row_index in empty_rows.iter() {
        let offset_index = *row_index + offset;
        let row = map.get(offset_index).unwrap().clone();
        map.insert(offset_index, row);
        offset += 1;
    }

    offset = 0;
    for row in &mut map {
        for col_index in empty_cols.iter() {
            let offset_index = *col_index + offset;
            row.insert(offset_index, '.');
            offset += 1;
        }
        offset = 0;
    }

    let mut galaxies = map
        .iter()
        .enumerate()
        .map(|(i, row)| {
            row.iter()
                .enumerate()
                .filter(|(_, p)| **p == '#')
                .map(|(j, _)| (i, j))
                .collect::<Vec<(usize, usize)>>()
        })
        .flatten()
        .collect::<Vec<(usize, usize)>>();

    let distances = galaxies
        .clone()
        .iter()
        .map(|g| {
            let mut distance = Vec::new();

            for galaxy in &galaxies {
                let x: i32 = galaxy.0 as i32 - g.0 as i32;
                let y: i32 = galaxy.1 as i32 - g.1 as i32;
                let d = x.abs() + y.abs();
                distance.push(d);
            }
            galaxies.remove(0);

            distance
        })
        .flatten()
        .sum::<i32>();

    println!("distances: {:?}", distances);

    Ok(())
}
