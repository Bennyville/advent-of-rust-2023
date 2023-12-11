use std::fs::read_to_string;
use std::{io, clone};

#[derive(Debug, PartialEq, Eq)]
enum TileType {
    VPipe,
    HPipe,
    LPipe,
    JPipe,
    SevenPipe,
    FPipe,
    Ground,
    Start,
}

impl TileType {
    fn from_char(char: char) -> Option<Self> {
        match char {
            '|' => Some(Self::VPipe),
            '-' => Some(Self::HPipe),
            'L' => Some(Self::LPipe),
            'J' => Some(Self::JPipe),
            '7' => Some(Self::SevenPipe),
            'F' => Some(Self::FPipe),
            '.' => Some(Self::Ground),
            'S' => Some(Self::Start),
            _ => None,
        }
    }

    fn direction(&self) -> (i32, i32) {
        match self {
            Self::VPipe => (0,0),
            Self::HPipe => (0,0),
            Self::LPipe => (1,-1),
            Self::JPipe => (-1,-1),
            Self::SevenPipe => (-1,1),
            Self::FPipe => (1,1),
            _ => unreachable!()
        }
    }
}

#[derive(Debug)]
struct Tile {
    tile_type: TileType,
}

impl Tile {
    fn new(char: char) -> Option<Self> {
        let tile_type = TileType::from_char(char)?;

        Option::Some(Tile { tile_type })
    }
}

struct Navigator<'a> {
    map: &'a Vec<Vec<Tile>>,
    current_pos: (i32, i32),
    direction: (i32, i32)
}

impl<'a> Navigator<'a> {
    fn new(map: &'a Vec<Vec<Tile>>) -> Self {
        Self {
            map,
            current_pos: (0, 0),
            direction: (0, 0)
        }
    }

    fn find_start(&self) -> Option<(i32, i32)> {
        for (y, row) in self.map.iter().enumerate() {
            for (x, col) in row.iter().enumerate() {
                if col.tile_type == TileType::Start {
                    return Some((x as i32, y as i32));
                }
            }
        }
        
        None
    }
}

pub fn solve() -> Result<(), io::Error> {
    let contents = read_to_string("input/day10.txt")?;

    let map: Vec<_> = contents
        .lines()
        .map(|f| {
            f.chars()
                .map(|f| {
                    Tile::new(f).unwrap()
                })
                .collect::<Vec<Tile>>()
        })
        .collect();

    let mut navigator = Navigator::new(&map);
    navigator.direction = (0, -1);

    let start = Navigator::find_start(&navigator).unwrap();
    navigator.current_pos = (start.0 + navigator.direction.0, start.1 + navigator.direction.1);

    let mut steps = 0;

    loop {
        steps += 1;

        let current_tile = &navigator.map[navigator.current_pos.1 as usize][navigator.current_pos.0 as usize];

        if current_tile.tile_type == TileType::Start {
            break;
        }

        navigator.direction.0 += current_tile.tile_type.direction().0;
        navigator.direction.1 += current_tile.tile_type.direction().1;

        navigator.current_pos = (navigator.current_pos.0 + navigator.direction.0, navigator.current_pos.1 + navigator.direction.1);
    }

    println!("{:?}", steps/2);

    Ok(())
}
