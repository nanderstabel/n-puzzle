use n_puzzle::*;
use std::{
    fs::File,
    io::{prelude::*, BufReader}
};
use std::num::ParseIntError;

fn parse_input(file: File) -> Result<(u8, Vec<Vec<u8>>), ParseIntError> {
    let buf = BufReader::new(file);
    let lines = buf.lines();
    let mut vector: Vec<Vec<u8>> = Vec::new();

    for line in lines {
        let line = line.unwrap();
        if !line.starts_with('#') {
            let string: &str = line.split('#').collect::<Vec<_>>()[0];
            let result: Vec<u8> = string.split_whitespace()
                                        .map(|x| x.parse::<u8>())
                                        .collect::<Result<Vec<u8>, _>>()?;
            vector.push(result);
        }
    }
    Ok((vector[0][0], vector[1..].to_vec()))
}

pub fn get_grid(file: File) -> Result<Grid, ParseIntError> {
    let (size, grid) = parse_input(file)?;
    Ok(grid)
}