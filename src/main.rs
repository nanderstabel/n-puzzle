/* ************************************************************************** */
/*                                                                            */
/*                                                        ::::::::            */
/*   main.rs                                            :+:    :+:            */
/*                                                     +:+                    */
/*   By: nstabel <nstabel@student.codam.nl>           +#+                     */
/*                                                   +#+                      */
/*   Created: 2021/04/22 15:48:20 by nstabel       #+#    #+#                 */
/*   Updated: 2021/11/24 22:37:21 by nstabel       ########   odam.nl         */
/*                                                                            */
/* ************************************************************************** */

mod input;
use input::*;
use std::{
    error::Error,
    collections::HashMap
};
use n_puzzle::*;

fn main() -> Result<(), Box<dyn Error>> {
    // let puzzle = Puzzle::default();
    let mut data = HashMap::new();
    let mut start = Node {
        grid: get_grid()?,
        cursor: (0, 0),
        h: 0,
        parent: None
    };
    for (y, row) in start.grid.iter().enumerate() {
        for (x, u) in row.iter().enumerate() {
            data.insert(u, Data::new(0, (y, x)));
        }
    }
    println!("{}", start);
    println!("{:#?}", data);
    Ok(())
}
