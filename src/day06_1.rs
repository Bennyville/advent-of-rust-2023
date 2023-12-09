use std::fs::read_to_string;
use std::io;

pub fn solve() -> Result<(), io::Error> {
    let contents = read_to_string("input/day06.txt")?;

    let races: Vec<Vec<_>> = contents
        .lines()
        .map(|f| {
            f.split(":")
                .map(|f| {
                    f.split(" ")
                        .map(|f| f.trim())
                        .filter(|p| !p.is_empty())
                        .collect::<Vec<_>>()
                })
                .collect::<Vec<_>>()[1]
                .clone()
                .iter()
                .map(|f| f.parse::<i32>().unwrap())
                .collect::<Vec<_>>()
        })
        .collect();


    let mut result: Vec<i32> = Vec::new();

    for i in 0..=races[0].len()-1 {
        let mut time = races[0][i];
        let max_distance = races[1][i];

        let mut speed = 0;

        let mut solutions = 0;

        while time >= 0 {
            speed += 1;

            time -= 1;

            let my_distance = speed * time;

            if my_distance > max_distance {
                solutions += 1;
            }
        }

        result.push(solutions);
    }

    println!("{:?}", result.iter().fold(1, |acc, &x| acc * x));

    Ok(())
}
