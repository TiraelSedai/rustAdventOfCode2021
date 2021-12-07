use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    if let Ok(lines) = read_lines("data/input.txt") {
        let lines = lines.map(|x| x.unwrap()).collect::<Vec<String>>();

        let answer = calc_answer(lines);

        println!("{}", answer);
    }
}

fn calc_answer(input: Vec<String>) -> usize {
    let coords = input
        .iter()
        .map(|l| {
            let mut xy = l
                .split(" -> ")
                .map(|x| x.split(',').map(|n| n.parse::<usize>().unwrap()));
            let mut xy1 = xy.next().unwrap();
            let mut xy2 = xy.next().unwrap();
            (
                (xy1.next().unwrap(), xy1.next().unwrap()),
                (xy2.next().unwrap(), xy2.next().unwrap()),
            )
        })
        .filter(|c| {
            let ((x1, y1), (x2, y2)) = c;
            x1 == x2 || y1 == y2
        })
        .collect::<Vec<((usize, usize), (usize, usize))>>();

    let mut map: Vec<Vec<usize>> = vec![vec![0; 1000]; 1000];

    for coord in coords {
        let ((x1, y1), (x2, y2)) = coord;
        if x1 == x2 {
            let min = std::cmp::min(y1, y2);
            let max = std::cmp::max(y1, y2);
            for y in min..=max {
                map[x1][y] += 1;
            }
        } else {
            let min = std::cmp::min(x1, x2);
            let max = std::cmp::max(x1, x2);
            for x in min..=max {
                map[x][y1] += 1;
            }
        }
    }

    let answer: usize = map
        .iter()
        .map(|x| x.iter().filter(|&&i| i > 1_usize).count())
        .sum();

    answer
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

#[test]
fn it_works() {
    let contents = "0,9 -> 5,9
        8,0 -> 0,8
        9,4 -> 3,4
        2,2 -> 2,1
        7,0 -> 7,4
        6,4 -> 2,0
        0,9 -> 2,9
        3,4 -> 1,4
        0,0 -> 8,8
        5,5 -> 8,2"
        .split('\n')
        .map(|x| x.trim().into())
        .collect::<Vec<String>>();
    let ans = calc_answer(contents);
    assert_eq!(ans, 5);
}
