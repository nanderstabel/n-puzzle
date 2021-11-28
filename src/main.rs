mod input;
mod structs;

use std::error::Error;
use structs::*;

fn main() -> Result<(), Box<dyn Error>> {
    let mut puzzle = puzzle::Puzzle::new();

    puzzle.initialize()?;
    puzzle.solve();
    println!("{}", puzzle);
    Ok(())
}
