use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    if let Ok(lines) = read_lines("data/input.txt") {
        let lines = lines.map(|x| x.unwrap()).collect::<Vec<String>>();
        let mut iter = lines.iter();
        let template_str = iter.next().unwrap();
        let template_chars = template_str.chars();

        let mut elements:HashMap<char, i64> = HashMap::new();
        elements.insert(template_str.chars().next().unwrap(), 1);

        iter.next();
        let mut polymer = fill_initial_state(template_chars);
        let mut modifier: HashMap<(char, char), char> = HashMap::new();
        for rest in iter.filter(|x| !x.is_empty()) {
            let mut s = rest.split(" -> ");
            let mut from = s.next().unwrap().chars();
            let left = from.next().unwrap();
            let right = from.next().unwrap();
            let to = s.next().unwrap().chars().next().unwrap();
            modifier.insert((left, right), to);
        }

        println!("{:?}", polymer);

        for _i in 0..10 {
            polymer = step(&polymer, &modifier);
        }

        println!("{:?}", polymer);

        for el in polymer {
            let (key, value) = el;
            let (_, right) = key;
            add_or_insert(&mut elements, right, value);
        }

        println!("{:?}", elements);

        let (least_char, least_amt) = elements.iter().min_by_key(|x| x.1).unwrap();
        let (max_char, max_amt) = elements.iter().max_by_key(|x| x.1).unwrap();

        println!("Most: {}, least: {}, diff: {}", max_char, least_char, max_amt - least_amt);
    }
}

fn add_or_insert(hm: &mut HashMap<char, i64>, key: char, amt: i64) {
    if let Some(val) = hm.get_mut(&key) {
        *val += amt;
    } else {
        hm.insert(key, amt);
    }
}

fn step(
    state: &HashMap<(char, char), i64>,
    modifier: &HashMap<(char, char), char>,
) -> HashMap<(char, char), i64> {
    let mut new_map: HashMap<(char, char), i64> = HashMap::new();
    for ((a, b), amount) in state {
        if let Some(new) = modifier.get(&(*a, *b)) {
            add_or_insert2(&mut new_map, *a, *new, *amount);
            add_or_insert2(&mut new_map, *new, *b, *amount);
        } else {
            panic!("Replacement not found");
        }
    }
    new_map
}

fn add_or_insert2(hm: &mut HashMap<(char, char), i64>, a: char, b: char, amt: i64) {
    if let Some(val) = hm.get_mut(&(a, b)) {
        *val += amt;
    } else {
        hm.insert((a, b), amt);
    }
}

fn fill_initial_state(chars: std::str::Chars) -> HashMap<(char, char), i64> {
    let mut state: HashMap<(char, char), i64> = HashMap::new();
    let mut prev: Option<char> = None;
    for c in chars {
        match prev {
            None => {
                prev = Some(c);
                continue;
            }
            Some(p) => {
                if let Some(v) = state.get_mut(&(p, c)) {
                    *v += 1;
                } else {
                    state.insert((p, c), 1);
                }
                prev = Some(c)
            }
        }
    }
    state
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
