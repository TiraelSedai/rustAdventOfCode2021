use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    if let Ok(lines) = read_lines("data/input.txt") {
        let lines_chars = lines
            .map(|x| x.unwrap().chars().collect::<Vec<char>>())
            .collect::<Vec<Vec<char>>>();
        let lines_chars_clone = lines_chars.clone();

        let lines_chars = filter_by_bit_criteria(lines_chars, true, '1');
        let lines_chars_clone = filter_by_bit_criteria(lines_chars_clone, false, '0');

        let oxygen_str = lines_chars[0].clone().into_iter().collect::<String>();
        let oxygen_generator_rating = i32::from_str_radix(oxygen_str.as_str(), 2).unwrap();
        let co2_str = lines_chars_clone[0].clone().into_iter().collect::<String>();
        let co2_scrubber_rating = i32::from_str_radix(co2_str.as_str(), 2).unwrap();

        println!(
            "oxygen generator rating: {}, CO2 scrubber rating: {}, multiply: {}",
            oxygen_generator_rating,
            co2_scrubber_rating,
            oxygen_generator_rating * co2_scrubber_rating
        );
    }
}

fn filter_by_bit_criteria(
    lines_chars: Vec<Vec<char>>,
    most_popular: bool,
    choose_if_equal: char,
) -> Vec<Vec<char>> {
    let data_len = 12;

    let mut lines = lines_chars;

    for i in 0..data_len {
        let len = lines.len();
        let half_len = len as f64 / 2.;
        if len == 1 {
            break;
        }
        let ones = lines
            .iter()
            .fold(0, |acc, x| if x[i] == '1' { acc + 1 } else { acc }) as f64;

        let c = if ones == half_len {
            choose_if_equal
        } else if most_popular ^ (ones > half_len) {
            '0'
        } else {
            '1'
        };
        lines = lines.into_iter().filter(|x| x[i] == c).collect();
    }
    lines
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
