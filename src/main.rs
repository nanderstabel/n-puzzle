mod heuristics;
mod input;
mod structs;

use crate::heuristics::*;
use std::error::Error;
use structs::*;

fn main() -> Result<(), Box<dyn Error>> {
    let mut puzzle = puzzle::Puzzle::new(manhattan);

    puzzle.initialize()?;
    puzzle.solve();
    println!("{}", puzzle);
    Ok(())
}
