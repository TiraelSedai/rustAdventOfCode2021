use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::str::FromStr;

fn main() {
    if let Ok(lines) = read_lines("data/input.txt") {
        let mut prev = i32::MAX;
        let mut result = 0;
        let int_lines: Vec<i32> = lines
            .map(|l| i32::from_str(&l.expect("not a line")).expect("cannot parse to int"))
            .collect();

        println!("{} lines found", int_lines.len());

        for cur in int_lines {
            if cur > prev {
                result += 1;
            }
            prev = cur;
        }

        println!("{}", result)
    }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
