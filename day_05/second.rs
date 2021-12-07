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
                .map(|x| x.split(',').map(|n| n.parse::<i32>().unwrap()));
            let mut xy1 = xy.next().unwrap();
            let mut xy2 = xy.next().unwrap();
            (
                (xy1.next().unwrap(), xy1.next().unwrap()),
                (xy2.next().unwrap(), xy2.next().unwrap()),
            )
        })
        .collect::<Vec<((i32, i32), (i32, i32))>>();

    let mut map: Vec<Vec<usize>> = vec![vec![0; 1000]; 1000];

    for coord in coords {
        let ((x1, y1), (x2, y2)) = coord;
        let mut x = x1;
        let mut y = y1;
        let dx = (x2 - x1).signum();
        let dy = (y2 - y1).signum();
        while x != x2 || y != y2 {
            map[y as usize][x as usize] += 1;
            x += dx;
            y += dy;
        }
        map[y as usize][x as usize] += 1;
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
    assert_eq!(ans, 12);
}
