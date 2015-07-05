// wf - a word frequency counter
// Copyright (c) 2015 John Berry
// This program is licensed under the Affero GPL v3 License.
// See agpl-3.0.txt for more information.

extern crate itertools;
//extern crate getopts;

mod freq;

use freq::{get_freqs, Sorted};
use std::io::prelude::*;
use std::io;
//use getopts::Options;
//use std::env;

fn main() {
    let mut input = io::stdin();
    let mut instr = String::new();

    match input.read_to_string(&mut instr) {
        Ok(s)  => print!("{}", get_freqs(instr, false, Sorted::Alpha)),
        Err(e) => panic!("{:?}", e)
    }
}
