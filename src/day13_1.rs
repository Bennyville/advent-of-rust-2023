use std::fs::read_to_string;
use std::io;

pub fn solve() -> Result<(), io::Error> {
    let contents = read_to_string("input/day13.txt")?;

    let valleys = contents
        .split("\n\n")
        .map(|x| x.lines().collect::<Vec<&str>>())
        .collect::<Vec<Vec<&str>>>();

    let mut result = 0;

    for valley in valleys {
        for line in &valley {
            println!("{}", line);
        }

        let mut found = false;

        'rows: for (i, line) in valley.iter().enumerate() {
            if i == valley.len() - 1 {
                break;
            }

            let mut current_line = line;
            let mut next_line = valley[i + 1];

            let mut j = 0;

            while current_line == &next_line {
                if i - j <= 0 || i + 1 + j >= valley.len() - 1 {
                    found = true;
                    result += (i + 1) * 100;
                    break 'rows;
                }
                println!("Row {} and {} are equal", i - j, i + 1 + j);
                // println!("Row {}: {}", i, current_line);
                // println!("Row {}: {}", i + 1, next_line);
                j += 1;
                current_line = &valley[i - j];
                next_line = valley[i + 1 + j];
                println!("Checking lines {} and {}", i - j, i + 1 + j);
                // println!("Row {}: {}", i - j, current_line);
                // println!("Row {}: {}", i + 1 + j, next_line);
            }
        }

        if found {
            println!("Result: {:?}", result);
            continue;
        }

        println!("Reflection row not found, checking columns");

        let valley_chars = valley
            .iter()
            .map(|x| x.chars().collect::<Vec<char>>())
            .collect::<Vec<Vec<char>>>();

        'cols: for i in 0..valley_chars[0].len() - 1 {
            let mut current_column = get_valley_column(&valley_chars, i);
            let mut next_column = get_valley_column(&valley_chars, i + 1);

            let mut j = 0;

            while current_column == next_column {
                if i - j <= 0 || i + 1 + j >= valley_chars[0].len() - 1 {
                    found = true;
                    result += i + 1;
                    break 'cols;
                }
                println!("Column {} and {} are equal", i - j, i + 1 + j);
                j += 1;
                let new_i = i - j;
                let new_i2 = i + 1 + j;
                current_column = get_valley_column(&valley_chars, new_i);
                next_column = get_valley_column(&valley_chars, new_i2);
                println!("Checking columns {} and {}", new_i, new_i2);
            }
        }

        if found {
            println!("Result: {:?}", result);
            continue;
        }
    }

    Ok(())
}

fn get_valley_column(valley: &Vec<Vec<char>>, column: usize) -> Vec<char> {
    let mut result = Vec::new();

    for row in valley {
        result.push(row[column]);
    }

    result
}
