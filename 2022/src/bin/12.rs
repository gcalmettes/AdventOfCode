use hashbrown::HashMap;
use once_cell::sync::Lazy;
use pathfinding::prelude::dijkstra;

static ALPHABET: Lazy<String> = Lazy::new(|| ('a'..='z').into_iter().collect::<String>());

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
struct Pos(usize, usize);

impl Pos {
    fn successors(&self, map: &HashMap<Pos, isize>, dim: &(usize, usize)) -> Vec<(Pos, isize)> {
        let &Pos(x, y) = self;
        let value = map.get(self).unwrap();
        let mut neighbors: Vec<Pos> = vec![];

        if x < dim.0 - 1 {
            let cmp = Pos(x + 1, y);
            if map.get(&cmp).unwrap() - value <= 1 {
                neighbors.push(cmp)
            }
        }
        if y < dim.1 - 1 {
            let cmp = Pos(x, y + 1);
            if map.get(&cmp).unwrap() - value <= 1 {
                neighbors.push(cmp)
            }
        }
        if x > 0 {
            let cmp = Pos(x - 1, y);
            if map.get(&cmp).unwrap() - value <= 1 {
                neighbors.push(cmp)
            }
        }
        if y > 0 {
            let cmp = Pos(x, y - 1);
            if map.get(&cmp).unwrap() - value <= 1 {
                neighbors.push(cmp)
            }
        }

        neighbors
            .into_iter()
            .map(|p| {
                let v = map.get(&p).unwrap();
                (p, *v)
            })
            .collect::<Vec<(Pos, isize)>>()
    }
}

fn parse_input(input: &str) -> (HashMap<Pos, isize>, (usize, usize), Vec<Pos>, Pos) {
    let nrows = input.lines().next().unwrap().len();
    let ncols = input.lines().collect::<Vec<_>>().len();
    let mut starts: Vec<Pos> = vec![];
    let mut goal = Pos(0, 0);
    let mut board: HashMap<Pos, isize> = HashMap::new();
    input.lines().enumerate().for_each(|(y, line)| {
        line.chars().enumerate().for_each(|(x, c)| {
            match c {
                'S' => {
                    // we insert true start as first position in vec
                    starts.insert(0, Pos(x, y));
                    board.insert(Pos(x, y), 1);
                }
                'E' => {
                    goal = Pos(x, y);
                    board.insert(Pos(x, y), 26);
                }
                'a' => {
                    starts.push(Pos(x, y));
                    board.insert(Pos(x, y), 1);
                }
                _ => {
                    board.insert(Pos(x, y), (ALPHABET.find(c).unwrap() + 1) as isize);
                }
            };
        });
    });
    (board, (nrows, ncols), starts, goal)
}

fn part1(data: &HashMap<Pos, isize>, dim: &(usize, usize), start: &Pos, goal: &Pos) -> usize {
    let result = dijkstra(
        start,
        |p| p.successors(data, dim),
        |p| p == &Pos(goal.0, goal.1),
    );
    let (path, _score) = result.unwrap();
    path.len() - 1
}

fn part2(
    data: &HashMap<Pos, isize>,
    dim: &(usize, usize),
    starts: &Vec<Pos>,
    goal: &Pos,
    p1: usize,
) -> usize {
    starts
        .iter()
        .map(|pos| {
            let result = dijkstra(
                pos,
                |p| p.successors(data, dim),
                |p| p == &Pos(goal.0, goal.1),
            );
            if let Some((path, _score)) = result {
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
    let (board, dim, starts, goal) = parse_input(input);
    let p1 = part1(&board, &dim, &starts[0], &goal);
    let p2 = part2(&board, &dim, &starts, &goal, p1);
    (p1, p2)
}
