use std::io::Result;

pub fn solve() -> Result<()> {
    let rows: u32 = std::fs::read_to_string("input/day01.txt")?
        .split("\n")
        .map(|f| {
            f.chars()
                .filter(|c| c.is_numeric())
                .map(|f| f.to_digit(10).unwrap())
                .collect::<Vec<u32>>()
        })
        .filter(|f| f.len() > 0)
        .map(|f| {
            let first = f.first().unwrap().clone();
            let last = f.last().unwrap().clone();

            first * 10 + last
        })
        .sum();

    println!("{:?}", rows);

    Ok(())
}
