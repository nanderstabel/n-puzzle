mod input;
mod structs;

use std::error::Error;
use structs::*;

fn manhattan((cy, cx): (usize, usize), (ey, ex): (usize, usize)) -> u16 {
    (if cy > ey { cy - ey } else { ey - cy } + if cx > ex { cx - ex } else { ex - cx }) as u16
}

fn hamming((cy, cx): (usize, usize), (ey, ex): (usize, usize)) -> u16 {
    if (cy, cx) == (ey, ex) {
        0
    } else {
        1
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut puzzle = puzzle::Puzzle::new(manhattan);

    puzzle.initialize()?;
    puzzle.solve();
    println!("{}", puzzle);
    Ok(())
}
