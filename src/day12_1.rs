use std::collections::HashMap;
use std::fs::read_to_string;
use std::{io, char};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum SpringState {
    Operational,
    Damaged,
    Unknown,
}

impl SpringState {
    fn from_char(c: char) -> Self {
        match c {
            '.' => Self::Operational,
            '#' => Self::Damaged,
            _ => Self::Unknown,
        }
    }

    fn to_char(&self) -> char {
        match self {
            Self::Operational => '.',
            Self::Damaged => '#',
            Self::Unknown => '?',
        }
    }
}

#[derive(Debug)]
struct SpringRow {
    springs: Vec<SpringState>,
    groups: Vec<usize>,
}

impl SpringRow {
    fn is_valid(&self) -> bool {
        let springs_by_state = self.group_by_state();
        // println!("{:?}", springs_by_state);
        springs_by_state
            .into_iter()
            .filter_map(|(state, count)| {
                if state == SpringState::Damaged {
                    Some(count)
                } else {
                    None
                }
            })
            .eq(self.groups.iter().copied())
    }

    fn group_by_state(&self) -> Vec<(SpringState, usize)> {
        let mut current_char = self.springs[0];
        let mut groups = vec![(current_char, 0)];

        for spring in self.springs.iter() {
            if *spring == current_char {
                let (_, count) = groups.last_mut().unwrap();
                *count += 1;
            } else {
                current_char = *spring;
                groups.push((current_char, 1));
            }
        }

        groups
    }

    fn valid_combinations(&self) -> usize {
        if let Some(index) = self
            .springs
            .iter()
            .position(|spring| *spring == SpringState::Unknown)
        {
            let mut operational_springs = self.springs.clone();
            operational_springs[index] = SpringState::Operational;

            let operational = SpringRow {
                springs: operational_springs.clone(),
                groups: self.groups.clone(),
            };


            let mut damaged_springs = self.springs.clone();
            damaged_springs[index] = SpringState::Damaged;

            let damaged = SpringRow {
                springs: damaged_springs.clone(),
                groups: self.groups.clone(),
            };

            operational.valid_combinations() + damaged.valid_combinations()
        } else {
            if self.is_valid() {
                return 1;
            } else {
                return 0;
            }
        }
    }

    fn to_string(&self) -> String {
        self.springs
            .iter()
            .map(|spring| spring.to_char())
            .collect::<String>()
    }
}

pub fn solve() -> Result<(), io::Error> {
    let contents = read_to_string("input/day12.txt")?;

    let mut springs = contents
        .lines()
        .map(|f| {
            let (springs, groups) = f.split_once(" ").unwrap();

            let springs = springs
                .chars()
                .map(|f| SpringState::from_char(f))
                .collect::<Vec<_>>();

            let groups = groups
                .split(",")
                .map(|f| f.parse::<usize>().unwrap())
                .collect::<Vec<_>>();

            SpringRow { springs, groups }
        })
        .collect::<Vec<_>>();

    println!("{:?}", springs.iter().map(|f| f.valid_combinations()).sum::<usize>());

    Ok(())
}
