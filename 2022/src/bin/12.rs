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
    let start = m.keys().find(|&pos| m[pos] == 0).unwrap();
    let goal = m.keys().find(|&pos| m[pos] == 27).unwrap();
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

fn part2(m: &Matrix<usize>, goal: &Pos) -> usize {
    // we start from the goal and stop as soon as we hit a 'a'
    if let Some(path) = bfs(
        goal,
        |&p| {
            m.neighbours(p, false).into_iter().filter(move |n| {
                // neighbors as to be at most 1 step above,
                // but can lower without restriction
                ((m[p] - 1)..27).contains(&m[*n])
            })
        },
        |&p| m[p] == 1,
    ) {
        path.len() - 1
    } else {
        0
    }
}

#[aoc::main()]
fn main(input: &str) -> (usize, usize) {
    let (board, start, goal) = parse_input(input);
    let p1 = part1(&board, &start, &goal);
    let p2 = part2(&board, &goal);
    (p1, p2)
}
