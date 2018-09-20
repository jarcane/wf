// Frequency analyzer

use itertools::Itertools;
use std::collections::HashMap;
use std::io::{BufRead, StdinLock};

pub enum Sorted {
    No,
    Alpha,
    Freq,
}

// The frequency counters
fn word_freq<F>(lines: &mut StdinLock, sf: F) -> HashMap<String, u32>
where
    F: Fn(char) -> bool,
{
    let mut dictionary = HashMap::new();
    let mut line = String::new();
    let mut count = 0;

    while lines.read_line(&mut line).unwrap() > 0 {
        line.split(&sf)
            .filter(|s| !s.is_empty())
            .map(|s| s.to_string())
            .for_each(|s| *dictionary.entry(s).or_insert(0u32) += 1);
        count += 1;
        print!("curr line: {}\n", count);
        line.clear();
    }

    dictionary
}

fn word_freq_nums(s: &mut StdinLock) -> HashMap<String, u32> {
    word_freq(s, |c: char| !c.is_alphanumeric())
}

fn word_freq_no_nums(s: &mut StdinLock) -> HashMap<String, u32> {
    word_freq(s, |c: char| !c.is_alphabetic())
}

// Preparing for output
fn sort_by_freq(c: HashMap<String, u32>) -> String {
    let counts: Vec<String> = c
        .iter()
        .sorted_by(|a, b| Ord::cmp(&b.1, &a.1))
        .iter()
        .map(|&(k, v)| format!("{} {}", k, v))
        .collect();

    counts.join("\n")
}

fn sort_by_alpha(c: HashMap<String, u32>) -> String {
    let mut arr: Vec<String> = c.iter().map(|(k, v)| format!("{} {}", k, v)).collect();
    arr.sort();
    arr.join("\n")
}

fn no_sort(c: HashMap<String, u32>) -> String {
    let counts: Vec<String> = c.iter().map(|(k, v)| format!("{} {}", k, v)).collect();
    counts.join("\n")
}

// The main dispatch function
pub fn get_freqs(s: &mut StdinLock, nums: bool, sort: Sorted) -> String {
    let count = match nums {
        true => word_freq_nums(s),
        false => word_freq_no_nums(s),
    };
    match sort {
        Sorted::No => no_sort(count),
        Sorted::Alpha => sort_by_alpha(count),
        Sorted::Freq => sort_by_freq(count),
    }
}
