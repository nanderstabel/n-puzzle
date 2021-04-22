/* ************************************************************************** */
/*                                                                            */
/*                                                        ::::::::            */
/*   main.rs                                            :+:    :+:            */
/*                                                     +:+                    */
/*   By: nstabel <nstabel@student.codam.nl>           +#+                     */
/*                                                   +#+                      */
/*   Created: 2021/04/22 15:48:20 by nstabel       #+#    #+#                 */
/*   Updated: 2021/04/22 20:21:20 by nstabel       ########   odam.nl         */
/*                                                                            */
/* ************************************************************************** */

use std::env;
use std::fs::File;
use std::path::Path;
use std::io::prelude::*;
use std::io::BufReader;

fn main() {
    let args: Vec<String> = env::args().collect();
    let path = Path::new(&args[1]);
    let file = File::open(path).expect("File does not exist!");
    let f = BufReader::new(file);

    for line in f.lines() {
        println!("{}", line.unwrap());
    }

}
