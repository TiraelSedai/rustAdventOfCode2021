use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    if let Ok(lines) = read_lines("data/input.txt") {
        let lines = lines.map(|x| x.unwrap()).collect::<Vec<String>>();
        let mut iter = lines.iter();
        let template_chars = iter.next().unwrap().chars();
        let initial_step = fill_initial_state(template_chars);
        let mut modifier: HashMap<(char, char), char> = HashMap::new();
        for rest in iter.filter(|x| !x.is_empty()) {
            let mut s = rest.split(" -> ");
            let mut from = s.next().unwrap().chars();
            let left = from.next().unwrap();
            let right = from.next().unwrap();
            let to = s.next().unwrap().chars().next().unwrap();
            modifier.insert((left, right), to);
        }
    }
}

fn step(
    state: &HashMap<(char, char), i32>,
    modifier: &HashMap<(char, char), char>,
) -> HashMap<(char, char), i32> {
    let mut new_map: HashMap<(char, char), i32> = HashMap::new();
    for ((a, b), amount) in state {
        if let Some(m) = modifier.get(&(*a, *b)) {
        } else {
            continue;
        }
    }
    new_map
}

fn fill_initial_state(chars: std::str::Chars) -> HashMap<(char, char), i32> {
    let mut first_step: HashMap<(char, char), i32> = HashMap::new();
    let mut prev: Option<char> = None;
    for c in chars {
        match prev {
            None => {
                prev = Some(c);
                continue;
            }
            Some(p) => {
                if let Some(v) = first_step.get_mut(&(p, c)) {
                    *v += 1;
                } else {
                    first_step.insert((p, c), 1);
                }
            }
        }
    }
    first_step
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
