// wf - a word frequency counter
// Copyright (c) 2015 John Berry
// This program is licensed under the Affero GPL v3 License.
// See agpl-3.0.txt for more information.

use std::collections::HashMap;

fn word_freq<F>(s: &str, sf: F) -> HashMap<String, u32>
    where F : Fn(char) -> bool {

        s.split(sf)
            .filter( |s| !s.is_empty() )
            .map(|s| { s.chars().flat_map(char::to_lowercase).collect::<String>() })
            .fold(HashMap::new(), |mut m, i| {
                *m.entry(i).or_insert(0u32) += 1;
                m
            })
    }

fn word_freq_nums(s: &str) -> HashMap<String, u32> {
    word_freq(s, |c: char| !c.is_alphanumeric())
}

fn word_freq_no_nums(s: &str) -> HashMap<String, u32> {
    word_freq(s, |c: char| !c.is_alphabetic())
}

fn main() {
    let sample = "Dave is fat. \n Dave sucks. \n My name is Dave. 42.";
    println!("{:?}", word_freq_no_nums(sample));
    println!("{:?}", word_freq_nums(sample))
}
