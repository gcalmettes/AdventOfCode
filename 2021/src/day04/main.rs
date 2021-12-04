use std::path::Path;
use std::fs;

type Board = Vec<Vec<usize>>;


trait BoardMethods {
    fn show(&self);
}

impl BoardMethods for Board {
    fn show(&self) {
        println!("--------");
        for line in self {
            println!("{:?}", line)
        }
        println!("--------");
    }
}

fn parse_input(content: &str) -> (Vec<usize>, Vec<Board>) {
    let draws_and_boards = content
        .split("\n\n")
        .collect::<Vec<&str>>();
    
    let draws = draws_and_boards[0].split(",")
        .map(|x| x.parse().unwrap())
        .collect();

    let boards = draws_and_boards.iter()
        .skip(1)
        .map(|board| board.lines()
                .map(|r| r.split_whitespace()
                    .map(|v| v.parse().unwrap())
                    .collect::<Vec<usize>>())
                .collect::<Vec<Vec<usize>>>())
        .collect::<Vec<Board>>();
    (draws, boards)
}


fn get_cols(board: Board) -> Vec<Vec<usize>> {
    let mut t = vec![Vec::with_capacity(board.len()); board[0].len()];
    for row in board {
        for i in 0..row.len() {
            t[i].push(row[i])
        }
    }
    t
}

fn get_score(draws: Vec<usize>, board: Board) -> usize {
    board.iter()
        .flatten()
        .filter(|v| !draws.contains(v))
        .sum()
}

fn part1() -> usize {
    let path = Path::new("./inputs/day04.txt");
    let input = fs::read_to_string(path).expect("Unable to read file");

    let (draws, boards) = parse_input(&input);

    for i in boards[0].len()..draws.len() {
        for b in &boards {

            // check rows
            for row in b.to_vec() {
                if row.iter().all(|v| draws[0..i].contains(v)) {
                    println!("We have a winner after {} was drawn !", draws[i-1]);
                    let board_score = get_score(draws[0..i].to_vec(), b.to_vec());
                    return board_score * draws[i-1]
                }

            }

            // check cols
            for col in get_cols(b.to_vec()) {
                if col.iter().all(|v| draws[0..i].contains(v)) {
                    println!("We have a winner after {} was drawn !", draws[i-1]);
                    let board_score = get_score(draws[0..i].to_vec(), b.to_vec());
                    return board_score * draws[i-1]
                }
            }
        }
    }
    0
}

fn part2() -> usize {
    let path = Path::new("./inputs/day04.txt");
    let input = fs::read_to_string(path).expect("Unable to read file");

    let (draws, boards) = parse_input(&input);

    let mut winners:Vec<usize> = vec![];
    let mut winners_draws:Vec<usize> = vec![];
    let mut winners_draws_pos:usize = 0;

    for i in boards[0].len()..draws.len() {
        let mut board_number = 0;
        for b in &boards {
            // check rows
            for row in b.to_vec() {
                if row.iter()
                    .all(|v| draws[0..i].contains(v)) {
                        if !winners.contains(&board_number) {
                            winners.push(board_number);
                            winners_draws.push(draws[i-1]);
                            winners_draws_pos = i;
                            // println!("We have a winner after {} was drawn ! {}", draws[i-1], board_number);
                        }
                }

            }

            // check cols
            for col in get_cols(b.to_vec()) {
                if col.iter()
                    .all(|v| draws[0..i].contains(v)) {
                        if !winners.contains(&board_number) {
                            winners.push(board_number);
                            winners_draws.push(draws[i-1]);
                            winners_draws_pos = i;
                            // println!("We have a winner after {} was drawn ! {}", draws[i-1], board_number);
                        }
                }
            }
            board_number += 1
        }
    }
    // println!("{:?}", winners);
    // println!("{:?}", winners_draws);
    let last_winner = &boards[winners[winners.len()-1]].to_vec();
    let last_draw = winners_draws[winners_draws.len()-1];
    let score = get_score(draws[0..winners_draws_pos].to_vec(), last_winner.to_vec());
    println!("The last winner board is board #{}, after {} was drawn !", winners[winners.len()-1], last_draw);
    score * last_draw
}

fn main() {
    let p1 = part1();
    println!("{}", p1);
    let p2 = part2();
    println!("{}", p2);
}
