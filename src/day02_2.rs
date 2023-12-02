use std::{collections::HashMap, io::Result};

use regex::Regex;

pub fn solve() -> Result<()> {
    let input = std::fs::read_to_string("day02.txt")?;

    let re = Regex::new(r"([0-9]+)\s(blue|green|red)").unwrap();

    let mut games: HashMap<usize, [usize; 3]> = HashMap::new();
    let mut game_no = 1;

    input.split("\n").for_each(|f| {
        for (_, [amount, color]) in re.captures_iter(f).map(|c| c.extract()) {

            let mut amount = amount.parse::<usize>().unwrap();

            let color_index = match color {
                "red" => 0,
                "green" => 1,
                "blue" => 2,
                _ => continue,
            };

            games
                .entry(game_no)
                .and_modify(|f| {
                    match f.get_mut(color_index) {
                        Some(x) if x < &mut amount => *x = amount,
                        Some(x) => *x = *x,
                        None => (),
                    }

                    ()
                })
                .or_insert_with(|| {
                    let mut counts = [0; 3];
                    counts[color_index] = amount;
                    counts
                });

        }
        game_no = game_no + 1;
        return; 
    });

    let result: Vec<usize> = games
        .iter()
        .map(|(_, value)| value[0] as usize * value[1] as usize * value[2] as usize)
        .collect::<Vec<usize>>();

    let sum: usize = result.iter().sum();
    println!("{:#?}", sum);

    Ok(())
}
