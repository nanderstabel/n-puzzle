mod input;
mod structs;

use std::error::Error;
use structs::*;

fn main() -> Result<(), Box<dyn Error>> {
    let mut puzzle = puzzle::Puzzle::new();
    puzzle.initialize_start()?;

    println!("{:?}", puzzle.open[0]);

    puzzle.solve();

    println!("{:#?}", puzzle.open);

    Ok(())
}
