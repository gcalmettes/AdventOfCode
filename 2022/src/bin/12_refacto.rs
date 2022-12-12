use once_cell::sync::Lazy;
use pathfinding::matrix::Matrix;
use pathfinding::prelude::bfs;

static ALPHABET: Lazy<String> = Lazy::new(|| {
    let mut s = String::from("S");
    s.extend(('a'..='z').into_iter());
    s.push('E');
    s
});

type Pos = (usize, usize);

fn parse_input(input: &str) -> (Matrix<usize>, Pos, Pos) {
    let mut m = Matrix::from_rows(
        input
            .lines()
            .map(|line| line.chars().map(|c| (ALPHABET.find(c).unwrap()) as usize)),
    )
    .unwrap();
    let start = m.indices().find(|&pos| m[pos] == 0).unwrap();
    let goal = m.indices().find(|&pos| m[pos] == 27).unwrap();
    // back to corresponding values
    m[start] = 1;
    m[goal] = 26;
    (m, start, goal)
}

fn part1(m: &Matrix<usize>, start: &Pos, goal: &Pos) -> usize {
    if let Some(path) = bfs(
        start,
        |&p| {
            m.neighbours(p, false).into_iter().filter(move |n| {
                // neighbors as to be at most 1 step above,
                // but can lower without restriction
                (0..=(m[p] + 1)).contains(&m[*n])
            })
        },
        |&p| p == *goal,
    ) {
        path.len() - 1
    } else {
        0
    }
}

fn part2(m: &Matrix<usize>, goal: &Pos, p1: usize) -> usize {
    m.indices()
        .filter(|&p| m[p] == 1)
        .map(|start| {
            if let Some(path) = bfs(
                &start,
                |&p| {
                    m.neighbours(p, false).into_iter().filter(move |n| {
                        // neighbors as to be at most 1 step above,
                        // but can lower without restriction
                        (0..=(m[p] + 1)).contains(&m[*n])
                    })
                },
                |&p| {
                    //yolo
                    p == *goal
                },
            ) {
                path.len() - 1
            } else {
                p1
            }
        })
        .min()
        .unwrap()
}

#[aoc::main()]
fn main(input: &str) -> (usize, usize) {
    let (board, start, goal) = parse_input(input);
    let p1 = part1(&board, &start, &goal);
    let p2 = part2(&board, &goal, p1);
    (p1, p2)
}
