/* ************************************************************************** */
/*                                                                            */
/*                                                        ::::::::            */
/*   main.rs                                            :+:    :+:            */
/*                                                     +:+                    */
/*   By: nstabel <nstabel@student.codam.nl>           +#+                     */
/*                                                   +#+                      */
/*   Created: 2021/04/22 15:48:20 by nstabel       #+#    #+#                 */
/*   Updated: 2021/11/23 23:06:36 by nstabel       ########   odam.nl         */
/*                                                                            */
/* ************************************************************************** */

use n_puzzle::*;
use std::{
    env,
    fs::File,
    io::{prelude::*, BufReader},
    path::Path,
    collections::HashMap
};
use std::num::ParseIntError;

fn get_puzzle(filename: impl AsRef<Path>) -> Result<Grid, ()> {
    let file = File::open(filename).expect("no such file");
    let buf = BufReader::new(file);
    let mut grid = Grid::new();

    let lines = buf.lines();

    for line in lines {
        let temp = line.unwrap();
        let temp0: &str = temp.split('#').collect::<Vec<_>>()[0];
        if temp0.starts_with("#") {
            continue;
        }
        let temp2: Vec<_> = temp0.split_whitespace().collect();
        let temp3: Result<Vec<u8>, _> = temp2.into_iter()
                                                .map(|x| x.parse::<u8>())
                                                .collect();
        grid.push(temp3.unwrap());
    }
    Ok(grid)
}

fn main () {
    let args: Vec<String> = env::args().collect();
    // let puzzle = Puzzle::default();
    // let mut test = HashMap::new();


    let grid = get_puzzle(&args[1]).unwrap();
    println!("{:#?}", grid);

}
