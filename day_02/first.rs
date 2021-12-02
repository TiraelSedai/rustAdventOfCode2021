use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    if let Ok(lines) = read_lines("data/input.txt") {
        let tuples = lines.map(|x| x.unwrap()).map(|line| {
            let tokens: Vec<&str> = line.split_whitespace().collect();
            (tokens[0].to_string(), tokens[1].parse::<i32>().unwrap())
        });

        let (fwd, depth) = tuples.fold((0, 0), |acc, x| {
            let (fwd, depth) = acc;
            let (command, i) = x;
            match command.as_str() {
                "forward" => (fwd + i, depth),
                "down" => (fwd, depth + i),
                "up" => (fwd, depth - i),
                _ => panic!("at the disco"),
            }
        });

        println!(
            "fwd: {} depth: {}, multiplication: {}",
            fwd,
            depth,
            fwd * depth
        );
    }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
