use itertools::Itertools;
use std::cmp::Reverse;
// use std::collections::{HashMap, HashSet, VecDeque};
use hashbrown::{HashMap, HashSet};

fn neighbors(pos: &(usize, usize)) -> Vec<(usize, usize)> {
    vec![
        (((pos.0 as isize) - 1) as usize, pos.1),
        (pos.0 + 1, pos.1),
        (pos.0, ((pos.1 as isize) - 1) as usize),
        (pos.0, pos.1 + 1),
    ]
}

fn parse_map(
    input: &str,
) -> (
    HashMap<(usize, usize), char>,
    (usize, usize),
    (usize, usize),
) {
    let map = input
        .lines()
        .enumerate()
        .flat_map(|(r, line)| {
            line.chars()
                .enumerate()
                .map(move |(c, s)| ((c, r), s))
                .collect::<Vec<_>>()
        })
        .collect::<HashMap<(usize, usize), char>>();
    let start = map
        .iter()
        .find_map(|((c, r), s)| (r == &0 && s == &'.').then(|| (*c, *r)))
        .unwrap();
    let r_max = map.iter().map(|((_c, r), _s)| r).max().unwrap();
    let end = map
        .iter()
        .sorted_unstable_by_key(|(_c, r)| Reverse(*r))
        .find_map(|((c, r), s)| (r == r_max && s == &'.').then(|| (*c, *r)))
        .unwrap();
    (map, start, end)
}

fn longest_path(
    map: &HashMap<(usize, usize), char>,
    start: &(usize, usize),
    end: &(usize, usize),
    p2: bool,
) -> usize {
    // make a graph of connected nodes
    let mut graph = map
        .iter()
        .filter_map(|(pos, s)| {
            let possible_neighbors = match s {
                '#' => None,
                _ if p2 => Some(neighbors(pos)),
                '>' => Some(vec![(pos.0 + 1, pos.1)]),
                '<' => Some(vec![((pos.0 as isize - 1) as usize, pos.1)]),
                'v' => Some(vec![(pos.0, pos.1 + 1)]),
                '^' => Some(vec![(pos.0, (pos.1 as isize - 1) as usize)]),
                _ => Some(neighbors(pos)),
            };
            possible_neighbors.map(|nbrs| {
                (
                    *pos,
                    nbrs.into_iter()
                        .filter_map(|nb| (map.get(&nb).unwrap_or(&'#') != &'#').then(|| (nb, 1)))
                        .collect::<Vec<((usize, usize), usize)>>(),
                )
            })
        })
        .collect::<HashMap<(usize, usize), Vec<((usize, usize), usize)>>>();

    // detect hallways, they have only 2 neighbors
    let hallways = graph
        .clone()
        .into_iter()
        .filter_map(|(pos, neighbors)| (neighbors.len() == 2).then(|| pos))
        .collect::<Vec<(usize, usize)>>();

    // compress the graph by removing all the long hallways
    for (x, y) in hallways {
        let nbrs = graph.remove(&(x, y)).unwrap();
        let ((x1, y1), d1) = nbrs[0];
        let ((x2, y2), d2) = nbrs[1];
        let n1 = graph.get_mut(&(x1, y1)).unwrap();
        if let Some(i) = n1.iter().position(|&((xx, yy), _)| (xx, yy) == (x, y)) {
            n1[i] = ((x2, y2), d1 + d2);
        }
        let n2 = graph.get_mut(&(x2, y2)).unwrap();
        if let Some(i) = n2.iter().position(|&((xx, yy), _)| (xx, yy) == (x, y)) {
            n2[i] = ((x1, y1), d1 + d2);
        }
    }

    let mut stack = Vec::<((usize, usize), usize, HashSet<(usize, usize)>, usize)>::new();
    let start_neighbors = graph
        .get(start)
        .unwrap()
        .iter()
        .map(|(pos, dist)| (*pos, *dist, HashSet::from([*start]), 0));
    stack.extend(start_neighbors);

    let mut path_lengths = HashSet::<usize>::new();

    while let Some((pos, dist, path, total)) = stack.pop() {
        if path.contains(&pos) {
            continue;
        }

        if &pos == end {
            path_lengths.insert(total + dist);
            continue;
        }

        let mut path = path.clone();
        path.insert(pos);

        let neighbors = graph
            .get(&pos)
            .unwrap()
            .iter()
            .map(|(pos, d)| (*pos, *d, path.clone(), total + dist));
        stack.extend(neighbors);
    }

    path_lengths.into_iter().max().unwrap_or_default()
}

#[aoc::main()]
fn main(input: &str) -> (usize, usize) {
    let (map, start, end) = parse_map(input);
    let p1 = longest_path(&map, &start, &end, false);
    let p2 = longest_path(&map, &start, &end, true);
    (p1, p2)
}
