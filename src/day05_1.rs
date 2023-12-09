use std::fs::read_to_string;
use std::io;

pub fn solve() -> Result<(), io::Error> {
    let contents = read_to_string("input/day05.txt")?;

    let test: Vec<&str> = contents.split("\n\n").collect();

    let seeds: Vec<i64> = test[0].split(":").collect::<Vec<_>>()[1]
        .split(" ")
        .filter(|p| !p.is_empty())
        .map(|f| f.parse::<i64>().unwrap())
        .collect();

    let maps: &Vec<_> = &test[1..]
        .iter()
        .map(|map| {
            map.split(":").collect::<Vec<&str>>()[1]
                .lines()
                .filter(|line| !line.is_empty())
                .map(|line| line.split(" ").map(|numbers| numbers.parse::<i64>().unwrap()).collect::<Vec<_>>())
                .collect::<Vec<_>>()
        })
        .collect();

    let mut locations: Vec<i64> = Vec::new();

    for mut seed in seeds {
        for cat in maps {
            let mut found = false;
            for ranges in cat {
                if found {
                    continue;
                }

                let dest: &i64 = ranges.get(0).unwrap();
                let src: &i64 = ranges.get(1).unwrap();
                let d: &i64 = ranges.get(2).unwrap();

                if seed >= *src && seed <= src+d {
                    println!("seed {} found in range {} - {}", seed, src, src+d);
                    let offset = seed - src;
                    seed = dest + offset;
                    println!("next seed is {}", seed);
                    found = true
                }
            }
            if !found {
                println!("seed {} not found, seed {}", seed, seed);
            }
        }
        println!("location is {}, next seed", seed);
        println!("=====================");
        locations.push(seed);
    }

    println!("{}", locations.iter().min().unwrap());

    Ok(())
}
