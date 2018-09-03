// Frequency analyzer

use std::io::{Lines, StdinLock};
use std::collections::HashMap;
use itertools::Itertools;

pub enum Sorted {
    No,
    Alpha,
    Freq
}

// The frequency counters
fn word_freq<F>(lines: Lines<StdinLock>, sf: F) -> HashMap<String, u32>
    where F : Fn(char) -> bool {

    let mut dictionary = HashMap::new();

    for line in lines {
        match line {
            Ok(l) => l.split(&sf)
                        .filter( |s| !s.is_empty() )
                        .map(|s| { s.to_lowercase() })
                        .for_each(|s| *dictionary.entry(s).or_insert(0u32) += 1 ),
            Err(e) => panic!("{:?}", e)
        }
    }

    dictionary
}

fn word_freq_nums(s: Lines<StdinLock>) -> HashMap<String, u32> {
    word_freq(s, |c: char| !c.is_alphanumeric())
}

fn word_freq_no_nums(s: Lines<StdinLock>) -> HashMap<String, u32> {
    word_freq(s, |c: char| !c.is_alphabetic())
}

// Preparing for output
fn sort_by_freq(c: HashMap<String, u32>) -> String {
    c.iter()
        .sorted_by(|a, b| Ord::cmp(&b.1, &a.1))
        .iter()
        .map(|&(k, v)| format!("{} {}", k, v))
        .fold("".to_string(), |i, v| format!("{}{}\n", i, v))
} 

fn sort_by_alpha(c: HashMap<String, u32>) -> String {
    let mut arr: Vec<String> = c.iter().map(|(k, v)| format!("{} {}", k, v)).collect();
    arr.sort();
    arr.iter().fold("".to_string(), |i, v| i + v + "\n")
}

fn no_sort(c: HashMap<String, u32>) -> String {
    c.iter().map(|(k, v)| format!("{} {}", k, v)).fold("".to_string(), |i, v| format!("{}{}\n", i, v))
}

// The main dispatch function
pub fn get_freqs(s: Lines<StdinLock>, nums: bool, sort: Sorted) -> String {
    let count = match nums {
        true  => word_freq_nums(s),
        false => word_freq_no_nums(s),
    };
    match sort {
        Sorted::No    => no_sort(count),
        Sorted::Alpha => sort_by_alpha(count),
        Sorted::Freq  => sort_by_freq(count),
    }
}
