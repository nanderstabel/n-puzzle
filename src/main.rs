/* ************************************************************************** */
/*                                                                            */
/*                                                        ::::::::            */
/*   main.rs                                            :+:    :+:            */
/*                                                     +:+                    */
/*   By: nstabel <nstabel@student.codam.nl>           +#+                     */
/*                                                   +#+                      */
/*   Created: 2021/04/22 15:48:20 by nstabel       #+#    #+#                 */
/*   Updated: 2021/11/24 16:23:32 by nstabel       ########   odam.nl         */
/*                                                                            */
/* ************************************************************************** */

mod input;
use std::{
    env,
    fs::File,
    path::Path,
    error::Error,
    collections::HashMap
};

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();
    let file = File::open(Path::new(&args[1])).expect("no such file");
    // let puzzle = Puzzle::default();
    // let mut test = HashMap::new();


    let grid = input::get_puzzle(file)?;
    println!("{:#?}", grid);
    Ok(())
}
