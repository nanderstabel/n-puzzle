use n_puzzle::*;
use std::{
    fs::File,
    io::{prelude::*, BufReader}
};
use std::num::ParseIntError;

pub fn get_puzzle(file: File) -> Result<Grid, ParseIntError> {
    let buf = BufReader::new(file);
    let mut grid = Grid::new();

    let lines = buf.lines();

    for line in lines {
        let line = line.unwrap();
        if !line.starts_with('#') {
            let string: &str = line.split('#').collect::<Vec<_>>()[0];
            let result: Vec<u8> = string.split_whitespace()
                                        .map(|x| x.parse::<u8>())
                                        .collect::<Result<Vec<u8>, _>>()?;
            grid.push(result);
        }
    }
    Ok(grid)
}