mod input;
mod structs;

use std::error::Error;
use structs::*;

fn get_end_state() -> Grid {
    let end_state = Grid::new();
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut puzzle = puzzle::Puzzle::new();
    puzzle.initialize_start()?;

    println!("{:?}", puzzle.open[0]);
    puzzle.solve();

    println!("{:?}", puzzle.open[0]);
    Ok(())
}
