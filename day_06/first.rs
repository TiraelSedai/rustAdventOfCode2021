use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    if let Ok(lines) = read_lines("data/input.txt") {
        let lines = lines.map(|x| x.unwrap()).collect::<Vec<String>>();
        let mut hash_map: HashMap<i32, i64> = HashMap::new();
        for i in 0..=8 {
            hash_map.insert(i, 0);
        }

        for fish in lines[0].split(',').map(|x| x.parse::<i32>().unwrap()) {
            if let Some(val) = hash_map.get_mut(&fish) {
                *val += 1;
            } else {
                panic!();
            }
        }

        for _ in 0..80 {
            tick(&mut hash_map);
        }

        println!("answer {}", hash_map.iter().map(|(_, v)| v).sum::<i64>())
    }
}

fn tick(hash_map: &mut HashMap<i32, i64>) {
    let newborns = hash_map.get(&0).unwrap().clone();

    for i in 0..=7 {
        let new = hash_map.get(&(i + 1)).unwrap().clone();
        let in_place = hash_map.get_mut(&i).unwrap();
        *in_place = new;
    }
    let just_birthed_place = hash_map.get_mut(&6).unwrap();
    *just_birthed_place += newborns;
    let just_born = hash_map.get_mut(&8).unwrap();
    *just_born = newborns;
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
