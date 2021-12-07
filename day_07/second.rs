use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    if let Ok(lines) = read_lines("data/input.txt") {
        let lines = lines.map(|x| x.unwrap()).collect::<Vec<String>>();
        let initial = lines[0]
            .split(',')
            .map(|x| x.parse::<i32>().unwrap())
            .collect::<Vec<i32>>();
        let min: i32 = initial.iter().min().unwrap().clone();
        let max = initial.iter().max().unwrap().clone();

        let mut min_fuel = i32::MAX;
        for move_to in min..=max {
            let need_fuel = initial
                .iter()
                .fold(0, |acc, x| acc + calc_move((x - move_to).abs()));
            if need_fuel < min_fuel {
                min_fuel = need_fuel
            }
        }

        println!("need_fuel {}", min_fuel)
    }
}

// dirty brute force, not even memoizing known values
fn calc_move(input: i32) -> i32 {
    (0..=input).fold(0, |acc, x| acc + x)
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
