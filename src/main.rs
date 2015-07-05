// wf - a word frequency counter
// Copyright (c) 2015 John Berry
// This program is licensed under the Affero GPL v3 License.
// See agpl-3.0.txt for more information.

extern crate itertools;

mod freq;

use freq::{get_freqs, Sorted};

fn main() {
    let sample = "Dave is fat. \n Dave sucks. \n My name is Dave. 42.".to_string();
    print!("{}", get_freqs(sample, false, Sorted::Alpha));
}
