use std::io::Result;

pub fn solve() -> Result<()> {
    let input = std::fs::read_to_string("input/day03.txt")?;

    let parsed_lines: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();

    let mut part_nos: Vec<u32> = Vec::new();
    let mut part_no: Vec<&char> = Vec::new();
    let mut is_part_no: bool = false;

    parsed_lines
        .clone()
        .iter()
        .enumerate()
        .for_each(|(line_index, line)| {
            line.iter().enumerate().for_each(|(char_index, char)| {
                match char {
                    char if char.is_numeric() => {
                        part_no.push(char);

                        for i in -1..=1 as i32 {
                            for j in -1..=1 as i32 {
                                let n_line_index = line_index as i32 + i;
                                let n_char_index = char_index as i32 + j;

                                if n_line_index < 0
                                || n_line_index > parsed_lines.len() as i32 - 1
                                || n_char_index < 0
                                || n_char_index > line.len() as i32 - 1
                                {
                                    continue;
                                }

                                if !parsed_lines[n_line_index as usize][n_char_index as usize].is_numeric()
                                && parsed_lines[n_line_index as usize][n_char_index as usize] != '.'
                                {
                                    is_part_no = true
                                }
                            }
                        }

                        if parsed_lines[line_index].len() == char_index + 1 || !parsed_lines[line_index][char_index + 1].is_numeric() {
                            if !is_part_no {
                                part_no.clear();
                                is_part_no = false;
                                return;
                            }

                            let mut mul = 1;
                            let mut part_no_concat = 0;

                            for _ in 1..part_no.len() {
                                mul *= 10
                            }

                            for part_no_digit in &part_no {
                                part_no_concat = part_no_digit.to_digit(10).unwrap() * mul + part_no_concat;
                                mul /= 10;
                            }

                            part_nos.push(part_no_concat);

                            part_no.clear();
                            is_part_no = false;
                        }
                    },
                    _ => (),
                };
            })
        });

    println!("{:?}", part_nos.iter().copied().sum::<u32>());

    Ok(())
}
