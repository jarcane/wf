use std::collections::HashMap;

pub fn word_count(s: &str) -> HashMap<String, u32> {
    s.split(|c: char| !c.is_alphanumeric())
        .filter( |s| !s.is_empty() )
        .map(|s| { s.chars().flat_map(char::to_lowercase).collect::<String>() })
        .fold(HashMap::new(), |mut m, i| {
            *m.entry(i).or_insert(0u32) += 1;
            m
        })
}

fn main() {
    println!("{:?}", word_count("Dave is fat. \n Dave sucks. \n My name is Dave."));
}
