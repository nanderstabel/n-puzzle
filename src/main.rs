/* ************************************************************************** */
/*                                                                            */
/*                                                        ::::::::            */
/*   main.rs                                            :+:    :+:            */
/*                                                     +:+                    */
/*   By: nstabel <nstabel@student.codam.nl>           +#+                     */
/*                                                   +#+                      */
/*   Created: 2021/04/22 15:48:20 by nstabel       #+#    #+#                 */
/*   Updated: 2021/11/24 15:12:14 by nstabel       ########   odam.nl         */
/*                                                                            */
/* ************************************************************************** */

use n_puzzle::*;
use std::{
    env,
    fs::File,
    io::{prelude::*, BufReader},
    path::Path,
    error::Error,
    collections::HashMap
};
use std::num::ParseIntError;

fn get_puzzle(file: File) -> Result<Grid, ParseIntError> {
    let buf = BufReader::new(file);
    let mut grid = Grid::new();

    let lines = buf.lines();

    for line in lines {
        let line = line.unwrap();
        let string: &str = line.split('#').collect::<Vec<_>>()[0];
        if string.starts_with("#") {
            continue;
        }
        let result: Vec<u8> = string.split_whitespace()
                                    .map(|x| x.parse::<u8>())
                                    .collect::<Result<Vec<u8>, _>>()?;
        grid.push(result);
    }
    Ok(grid)
}

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();
    let file = File::open(Path::new(&args[1])).expect("no such file");
    // let puzzle = Puzzle::default();
    // let mut test = HashMap::new();


    let grid = get_puzzle(file)?;
    println!("{:#?}", grid);
    Ok(())
}
