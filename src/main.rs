mod input;
mod structs;

use n_puzzle::*;
use std::error::Error;
use structs::*;

fn get_end_state(size: u8) -> Grid {
    let mut grid: Grid = vec![vec![0; size.into()]; size.into()];
    let mut set: Vec<u8> = (1..(size.pow(2))).rev().collect();
    let (mut top, mut bottom, mut left, mut right): (isize, isize, isize, isize) =
        (0, (size - 1).into(), 0, (size - 1).into());

    macro_rules! fill {
        ($range:expr, $level:ident, $direction:literal, $increment:literal) => {
            if set.is_empty() {
                break;
            }
            for index in $range {
                match $direction {
                    "hor" => grid[$level as usize][index as usize] = set.pop().unwrap(),
                    _ => grid[index as usize][$level as usize] = set.pop().unwrap(),
                }
            }
            $level += $increment;
        };
    }

    loop {
        fill!(left..=right, top, "hor", 1);
        fill!(top..=bottom, right, "ver", -1);
        fill!((left..=right).rev(), bottom, "hor", -1);
        fill!((top..=bottom).rev(), left, "ver", 1);
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
