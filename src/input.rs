use n_puzzle::*;
use std::num::ParseIntError;
use std::{
    env,
    fs::File,
    io::{prelude::*, BufReader},
    path::Path,
};

fn get_buffer() -> BufReader<File> {
    let args: Vec<String> = env::args().collect();
    let file = File::open(Path::new(&args[1])).expect("no such file");
    BufReader::new(file)
}

fn parse_input() -> Result<(u8, Vec<Row>), ParseIntError> {
    let mut vector: Vec<Row> = Vec::new();
    let lines = get_buffer().lines();

    for line in lines {
        let line = line.unwrap();
        if !line.starts_with('#') {
            vector.push(
                (line.split('#').collect::<Vec<_>>()[0])
                    .split_whitespace()
                    .map(|x| x.parse::<u8>())
                    .collect::<Result<Row, _>>()?,
            );
        }
    }
    if vector[0].len() != 1 {
        "dummy".parse::<u8>()?;
    }
    Ok((vector[0][0], vector[1..].to_vec()))
}

pub fn get_grid() -> Result<(u8, Grid), ParseIntError> {
    let (size, grid) = parse_input()?;
    for row in &grid {
        if row.len() != size.into() {
            "dummy".parse::<u8>()?;
        }
    }
    let flat: Vec<&u8> = grid.iter().flat_map(|v| v).collect();
    let set: Row = (0..(size * size)).collect();
    if set.iter().all(|u| flat.contains(&u)) == false {
        "dummy".parse::<u8>()?;
    }
    if flat.len() != set.len() {
        "dummy".parse::<u8>()?;
    }
    Ok((size, grid))
}
