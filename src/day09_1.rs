use std::fs::read_to_string;
use std::io;

pub fn solve() -> Result<(), io::Error> {
    let contents = read_to_string("input/day09.txt")?;

    let result: i32 = contents
        .lines()
        .map(|f| {
            let mut digits: Vec<i32> = f.split(" ").map(|f| f.parse::<i32>().unwrap()).collect();
            let start_seq = digits.clone();
            let mut diffs: Vec<Vec<i32>> = Vec::new();
            let mut found = false;

            while !found {
                let diff: Vec<i32> = digits[..digits.len() - 1]
                    .iter()
                    .enumerate()
                    .map(|(i, f)| digits[i + 1] - f)
                    .collect();

                diffs.push(diff.clone());

                if diff.iter().all(|f| *f == 0) {
                    found = true;
                }
                digits = diff;
            }

            diffs.push(start_seq);

            let next_value: i32 = diffs.iter().map(|f| f.last().unwrap()).sum();

            next_value
        })
        .sum();

    println!("{:?}", result);

    Ok(())
}
