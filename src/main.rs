// wf - a word frequency counter
// Copyright (c) 2015 John Berry
// This program is licensed under the Affero GPL v3 License.
// See agpl-3.0.txt for more information.

extern crate itertools;
extern crate getopts;

mod freq;

use freq::{get_freqs, Sorted};
use std::io;
use getopts::Options;
use std::env;

fn print_usage(program: &str, opts: &Options) {
    let brief = format!("{} - Counts word frequency from stdin\nUsage: wf [options]", program);
    print!("{}", opts.usage(&brief));
}

fn main() {
    // Fetching CLI arguments
    let args: Vec<String> = env::args().collect();
    let program = args[0].clone();

    // Create possible option flags
    let mut opts = Options::new();
    opts.optflag("n", "nums", "Include numbers");
    opts.optflag("s", "sort", "Sort output alphabetically by words (incompatible with -f)");
    opts.optflag("f", "freq", "Sort output but most to least frequent (incompatible with -s)");
    opts.optflag("h", "help", "Display usage information");

    // Parse option matches
    let matches = match opts.parse(&args[1..]) {
        Ok(m) => { m }
        Err(f) => { panic!("{}", f.to_string()) }
    };

    // Display usage on -h || --help
    if matches.opt_present("h") {
        print_usage(&program, &opts);
        return;
    }

    // Declare options for output
    let nums = matches.opt_present("n");
    let sort = match (matches.opt_present("s"), matches.opt_present("f")) {
        (true , false) => Sorted::Alpha,
        (false, true ) => Sorted::Freq,
        (false, false) => Sorted::No,
        (_    , _    ) => {
            println!("Incompatible arguments");
            print_usage(&program, &opts);
            return;
        },
    };
    
    // Grabbing stdin
    let input = io::stdin();
    let mut lines = input.lock();
    
    get_freqs(&mut lines, nums, &sort);
}
