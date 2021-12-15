use std::path::Path;
use std::fs;
use std::collections::HashMap;
use pathfinding::prelude::dijkstra;

// Taken from the dijkstra exemple in the doc
// https://github.com/samueltardieu/pathfinding/blob/main/src/directed/dijkstra.rs#L41
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
struct Pos(usize, usize);

impl Pos {
  fn successors(&self, data: &HashMap<Pos, usize>, n_cols: usize, n_rows: usize) -> Vec<(Pos, usize)> {
    let &Pos(x, y) = self;
    let mut neighbours: Vec<Pos> = Vec::new();

    if x < n_cols -1 {
        neighbours.push(Pos(x + 1, y));
    }
    
    if y < n_rows - 1 {
        neighbours.push(Pos(x, y + 1));
    }

    // prevent panic overflow for unsigned
    match (x != 0, y != 0) {
        (false, true) => {
            neighbours.push(Pos(x, y - 1));
        },
        (true, false) => {
            neighbours.push(Pos(x - 1, y));
        },
        (true, true) => {
            neighbours.push(Pos(x, y - 1));
            neighbours.push(Pos(x - 1, y));
        },
        (_, _) => (),
    }
    neighbours.into_iter()
        .map(|p| {
            // println!("{:?}", p);
            let weight = data.get(&p).unwrap();
            (p, *weight)
         }).collect()
  }
}


fn parse_input(content: &str) -> (HashMap<Pos, usize>, usize, usize) {
    let n_rows = content.lines().next().unwrap().len();
    let n_cols = content.lines().collect::<Vec<_>>().len();

    let mut data: HashMap<Pos, usize> = HashMap::new();

    // data
    content
        .lines()
        .enumerate()
        .for_each(|(y, line)| {
            line.chars()
                .enumerate()
                .for_each(|(x, c)| {
                    let n = c.to_string().parse().unwrap();
                    data.insert(Pos(x, y), n);
                })
        });

       (data, n_rows, n_cols)
}


fn part1(input: &str) -> usize {

    let (data, n_rows, n_cols) = parse_input(input);

    let result = dijkstra(&Pos(0, 0),
                            |p| p.successors(&data, n_rows, n_cols),
                            |p| *p == Pos(n_cols - 1, n_rows - 1));
    let (_path, score) = result.unwrap();

    score
}


fn part2(input: &str) -> usize {

    let (data, n_rows, n_cols) = parse_input(input);

    let mut extended_data: HashMap<Pos, usize> = HashMap::new();
    for y in 0..n_rows*5 {
        for x in 0..n_cols*5 {
            let base = data.get(&Pos(x % n_cols, y % n_rows)).unwrap();
            let f_x = x / n_cols;
            let f_y = y / n_rows;
            let new_value = (base + f_x + f_y - 1 ) % 9 + 1;
            extended_data.insert(Pos(x, y), new_value);
        }
    };

    let result = dijkstra(&Pos(0, 0),
                            |p| p.successors(&extended_data, n_rows * 5, n_cols * 5),
                            |p| *p == Pos(n_cols * 5 - 1, n_rows * 5 - 1));
    let (_path, score) = result.unwrap();

    score
}

fn main() {
    let path = Path::new("./inputs/day15.txt");
    let input = fs::read_to_string(path).expect("Unable to read file");

    let p1 = part1(&input);
    println!("{}", p1);
    let p2 = part2(&input);
    println!("{}", p2);
}
