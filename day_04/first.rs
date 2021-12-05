use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    if let Ok(lines) = read_lines("data/input.txt") {
        let lines = lines.map(|x| x.unwrap()).collect::<Vec<String>>();
        let numbers = lines
            .iter()
            .take(1)
            .map(|x| {
                x.split(",")
                    .map(|x| x.parse::<i32>().unwrap())
                    .collect::<Vec<i32>>()
            })
            .collect::<Vec<Vec<i32>>>();

        let board_lines = lines
            .into_iter()
            .skip(1)
            .filter(|x| !x.is_empty())
            .map(|x| {
                x.split_whitespace()
                    .map(|s| (s.parse::<i32>().unwrap(), false))
                    .collect::<Vec<(i32, bool)>>()
            })
            .collect::<Vec<Vec<(i32, bool)>>>();

        let mut boards = board_lines
            .chunks_exact(5)
            .map(|x| x.into())
            .collect::<Vec<Vec<Vec<(i32, bool)>>>>();

        for number in numbers.first().unwrap() {
            for x in 0..boards.len() {
                let board = &mut boards[x];
                mark_number(number, board);
                let board = &boards[x];
                if is_solved(board) {
                    let sum: i32 = board
                        .iter()
                        .map(|row| {
                            row.iter()
                                .filter(|(_, chosen)| chosen == &false)
                                .map(|(value, _)| value)
                                .sum::<i32>()
                        })
                        .sum();
                    println!(
                        "sum = {}, step = {}, multiply = {}",
                        sum,
                        number,
                        sum * number
                    );
                    return;
                }
            }
        }

        // println!("{:?}", boards);
    }
}

fn mark_number(number: &i32, board: &mut Vec<Vec<(i32, bool)>>) {
    for i in 0..5 {
        for j in 0..5 {
            let (num, chosen) = &mut board[i][j];
            if num == number {
                *chosen = true;
                return;
            }
        }
    }
}

fn is_solved(board: &Vec<Vec<(i32, bool)>>) -> bool {
    for i in 0..5 {
        let mut row = true;
        let mut column = true;
        for j in 0..5 {
            let (_, chosen) = board[i][j];
            let (_, chosen2) = board[j][i];
            if chosen == false {
                row = false;
            }
            if chosen2 == false {
                column = false;
            }
        }
        if row || column {
            return true;
        }
    }

    return false;
}

// fn is_solved(board: &Vec<Vec<(i32, bool)>>) -> bool {
//     for i in 0..5 {
//         if board[i].iter().all(|(_, chosen)| chosen == &true) {
//             return true;
//         }
//         if (0..5).all(|j| {
//             let (_, chosen) = board[i][j];
//             chosen == true
//         }) {
//             return true;
//         }
//     }

//     return false;
// }

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
