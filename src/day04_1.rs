use std::fs::read_to_string;
use std::io;

pub fn solve() -> Result<(), io::Error> {
    let contents = read_to_string("day04.txt")?;

    let test: i32 = contents
        .lines()
        .map(|line| {
            line.trim().split(":").collect::<Vec<&str>>()[1]
                .split("|")
                .map(|numbers| {
                    numbers
                        .trim()
                        .split(" ")
                        .filter(|number| number.to_owned() != "")
                        .collect::<Vec<&str>>()
                })
                .collect::<Vec<Vec<&str>>>()
        })
        .map(|numbers| {
            let winning_numbers = numbers[0].clone();
            let your_numbers = numbers[1].clone();

            your_numbers
                .iter()
                .filter(|your_number| winning_numbers.contains(your_number))
                .cloned()
                .collect::<Vec<&str>>()
        })
        .filter(|numbers| !numbers.is_empty())
        .map(|winning_numbers| {
            match winning_numbers.len() {
                1 => 1,
                x => 2i32.pow(x as u32-1)
            }
        })
        .sum();

    println!("{:#?}", test);

    Ok(())
}
