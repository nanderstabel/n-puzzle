mod input;
mod structs;

use n_puzzle::*;
use std::error::Error;
use structs::*;

fn get_end_state(size: u8) -> Grid {
    let mut grid: Grid = vec![vec![0; size.into()]; size.into()];
    let mut set: Vec<u8> = (1..(size.pow(2))).rev().collect();
    let (mut top, mut bottom, mut left, mut right) = (0, (size - 1).into(), 0, (size - 1).into());

    macro_rules! fill {
        ($range:expr, $level:ident, "hor") => {{
            if set.is_empty() {
                break;
            }
            for i in $range {
                grid[$level][i] = set.pop().unwrap();
            }
            1
        }};
        ($range:expr, $level:ident, "ver") => {{
            if set.is_empty() {
                break;
            }
            for i in $range {
                grid[i][$level] = set.pop().unwrap();
            }
            1
        }};
    }

    loop {
        top += fill!(left..=right, top, "hor");
        right -= fill!(top..=bottom, right, "ver");
        bottom -= fill!((left..=right).rev(), bottom, "hor");
        left += fill!((top..=bottom).rev(), left, "ver");
    }

    println!("{:?}\n\n\n", grid);
    grid
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut puzzle = puzzle::Puzzle::new();

    get_end_state(5);

    // puzzle.initialize()?;

    // println!("{:?}", puzzle.open[0]);
    // puzzle.solve();

    // println!("{:?}", puzzle.open[0]);
    Ok(())
}
