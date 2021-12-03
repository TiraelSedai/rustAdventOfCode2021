use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    if let Ok(lines) = read_lines("data/input.txt") {
        let data_len = 12; // hardcode ;(
        let lines_chars = lines
            .map(|x| x.unwrap().chars().collect::<Vec<char>>())
            .collect::<Vec<Vec<char>>>();
        let ones_init = vec![0; data_len];
        let ones_amount = lines_chars.iter().fold(ones_init, |mut acc, x| {
            for i in 0..data_len {
                if x[i] == '1' {
                    acc[i] += 1
                }
            }
            acc
        });

        let lines_amount = lines_chars.len();
        let half_lines = lines_amount / 2;
        let gamma_binary = ones_amount
            .iter()
            .map(|&x| if x > half_lines as i32 { '1' } else { '0' })
            .collect::<String>();
        let epsilon_binary = gamma_binary
            .chars()
            .map(|x| if x == '0' { '1' } else { '0' })
            .collect::<String>();
        let gamma = i32::from_str_radix(gamma_binary.as_str(), 2).unwrap();
        let epsilon = i32::from_str_radix(epsilon_binary.as_str(), 2).unwrap();

        print!(
            "Gamma: {}, epsilon: {}, multipy: {}",
            gamma,
            epsilon,
            gamma * epsilon
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
