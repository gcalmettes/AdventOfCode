use itertools::Itertools;
use std::collections::HashMap;

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
struct Num {
    value: usize,
    is_valid: bool,
}

fn has_adjacent_symbol_and_adjacent_stars(
    x: isize,
    y: isize,
    board: &Vec<Vec<char>>,
) -> (bool, Vec<(isize, isize)>) {
    let (len_y, len_x) = (board.len() as isize, board[0].len() as isize);

    let neigbors = vec![
        (x - 1, y),
        (x + 1, y),
        (x - 1, y - 1),
        (x - 1, y + 1),
        (x + 1, y - 1),
        (x + 1, y + 1),
        (x, y - 1),
        (x, y + 1),
    ];

    (
        neigbors
            .clone()
            .into_iter()
            .filter(|(x, y)| x >= &(0 as isize) && x < &len_x && y >= &(0 as isize) && y < &len_y)
            .map(|(x, y)| {
                let neighbor = board[y as usize][x as usize];
                !neighbor.is_digit(10) && neighbor != '.'
            })
            .any(|b| b),
        neigbors
            .into_iter()
            .filter(|(x, y)| x >= &(0 as isize) && x < &len_x && y >= &(0 as isize) && y < &len_y)
            .filter_map(|(x, y)| {
                let neighbor = board[y as usize][x as usize];
                match neighbor == '*' {
                    true => Some((x, y)),
                    false => None,
                }
            })
            .collect(),
    )
}

fn parse_board(input: &str) -> (usize, usize) {
    let lines = input.split("\n");
    let board = lines
        .clone()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();

    let len_x = board[0].len();

    let mut numbers: Vec<Num> = vec![];
    let mut star_symbols: HashMap<(isize, isize), Vec<Num>> = HashMap::new();

    for (y, line) in lines.enumerate() {
        let mut pos: Vec<(isize, isize)> = vec![];
        let mut val: Vec<char> = vec![];
        for (x, c) in line.chars().enumerate() {
            if c.is_digit(10) {
                val.push(c);
                pos.push((x as isize, y as isize));
                if x == len_x - 1 {
                    let value = String::from_iter(&val).parse::<usize>().unwrap();
                    let is_valid = pos
                        .iter()
                        .map(|(x, y)| has_adjacent_symbol_and_adjacent_stars(*x, *y, &board).0)
                        .any(|b| b);
                    numbers.push(Num { value, is_valid });
                    pos.iter().for_each(|(x, y)| {
                        let stars = has_adjacent_symbol_and_adjacent_stars(*x, *y, &board).1;
                        for star in stars {
                            let s = star_symbols.entry(star).or_insert(vec![]);
                            s.push(Num { value, is_valid });
                        }
                    });
                    pos = vec![];
                    val = vec![];
                }
            } else {
                if val.len() > 0 {
                    let value = String::from_iter(&val).parse::<usize>().unwrap();
                    let is_valid = pos
                        .iter()
                        .map(|(x, y)| has_adjacent_symbol_and_adjacent_stars(*x, *y, &board).0)
                        .any(|b| b);
                    numbers.push(Num { value, is_valid });
                    pos.iter().for_each(|(x, y)| {
                        let stars = has_adjacent_symbol_and_adjacent_stars(*x, *y, &board).1;
                        for star in stars {
                            let s = star_symbols.entry(star).or_insert(vec![]);
                            s.push(Num { value, is_valid });
                        }
                    });
                    pos = vec![];
                    val = vec![];
                }
            }
        }
    }

    (
        numbers.iter().filter(|n| n.is_valid).map(|n| n.value).sum(),
        star_symbols
            .into_iter()
            .map(|(_pos, v)| v.into_iter().unique().collect::<Vec<Num>>())
            .filter(|v| v.iter().len() == 2)
            .map(|v| v.iter().map(|n| n.value).product::<usize>())
            .sum(),
    )
}

#[aoc::main()]
fn main(input: &str) -> (usize, usize) {
    let (p1, p2) = parse_board(input);
    (p1, p2)
}
