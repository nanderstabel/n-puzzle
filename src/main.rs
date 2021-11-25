/* ************************************************************************** */
/*                                                                            */
/*                                                        ::::::::            */
/*   main.rs                                            :+:    :+:            */
/*                                                     +:+                    */
/*   By: nstabel <nstabel@student.codam.nl>           +#+                     */
/*                                                   +#+                      */
/*   Created: 2021/04/22 15:48:20 by nstabel       #+#    #+#                 */
/*   Updated: 2021/11/25 01:54:44 by nstabel       ########   odam.nl         */
/*                                                                            */
/* ************************************************************************** */

mod input;
use input::*;
use n_puzzle::*;
use std::{collections::HashMap, error::Error};

fn manhattan(x: usize, y: usize, i: usize, j: usize) -> u16 {
    (if x > i { x - i } else { i - x } + if y > j { y - j } else { j - y }) as u16
}

fn get_data(u: u8, y: usize, x: usize, end: &Grid) -> Data {
    for (j, row) in end.iter().enumerate() {
        for (i, v) in row.iter().enumerate() {
            if u == *v {
                return Data::new(manhattan(x, y, i, j), (y, x), (j, i));
            }
        }
    }
    Data::new(0, (y, x), (y, x))
}

fn main() -> Result<(), Box<dyn Error>> {
    // let puzzle = Puzzle::default();
    let mut data = HashMap::new();
    let mut start = Node {
        grid: get_grid()?,
        cursor: (0, 0),
        h: 0,
        parent: None,
    };
    let mut end = Node {
        grid: [
            [1, 2, 3, 4].to_vec(),
            [12, 13, 14, 5].to_vec(),
            [11, 0, 15, 6].to_vec(),
            [10, 9, 8, 7].to_vec(),
        ]
        .to_vec(),
        cursor: (0, 0),
        h: 0,
        parent: None,
    };
    for (y, row) in start.grid.iter().enumerate() {
        for (x, u) in row.iter().enumerate() {
            data.insert(u, get_data(*u, y, x, &end.grid));
        }
    }
    (*data.get_mut(&0).unwrap()).h = 9000;
    // println!("{}", start);
    println!("{}", end);
    println!("{:?}", data);
    Ok(())
}
