use std::cmp::max;
use std::collections::{HashMap, HashSet};

#[derive(Debug, PartialEq, Copy, Clone)]
enum Dir {
    NORTH,
    SOUTH,
    WEST,
    EAST,
}

impl Dir {
    fn opposite(&self) -> Dir {
        match self {
            Dir::NORTH => Dir::SOUTH,
            Dir::SOUTH => Dir::NORTH,
            Dir::WEST => Dir::EAST,
            Dir::EAST => Dir::WEST,
        }
    }

    fn to_delta(&self) -> (isize, isize) {
        match self {
            Dir::NORTH => (0, -1),
            Dir::SOUTH => (0, 1),
            Dir::WEST => (-1, 0),
            Dir::EAST => (1, 0),
        }
    }
}

fn connector_dirs(c: &char, start_dirs: &Vec<Dir>) -> Vec<Dir> {
    match c {
        '|' => vec![Dir::NORTH, Dir::SOUTH],
        '-' => vec![Dir::WEST, Dir::EAST],
        'L' => vec![Dir::NORTH, Dir::EAST],
        'J' => vec![Dir::NORTH, Dir::WEST],
        '7' => vec![Dir::WEST, Dir::SOUTH],
        'F' => vec![Dir::EAST, Dir::SOUTH],
        'S' => start_dirs.clone(),
        _ => vec![],
    }
}

fn parse_pipe_maze(input: &str) -> ((usize, usize), HashMap<(usize, usize), char>) {
    let maze = input
        .lines()
        .enumerate()
        .flat_map(|(j, line)| line.chars().enumerate().map(move |(i, c)| ((i, j), c)))
        .collect::<HashMap<(usize, usize), char>>();

    let start = maze
        .iter()
        .filter_map(|(k, v)| (v == &'S').then(|| *k))
        .last()
        .unwrap();

    (start, maze)
}

// based on the last two points, get the next pos to continue the path
fn get_next_pos(
    last: (usize, usize),
    maze: &HashMap<(usize, usize), char>,
    start_dirs: &Vec<Dir>,
    connected_dir: &Dir,
) -> (Option<(usize, usize)>, Option<Dir>) {
    // get char for the pos
    let l = maze.get(&last).unwrap();
    // and get the associated dirs
    let l_dirs = connector_dirs(l, start_dirs);

    // get the direction of the last point that is not already connected to the previous point
    let free_dir = l_dirs
        .into_iter()
        .filter(|d| d != &connected_dir.opposite())
        .last();

    // get next pos pointed by this dir
    let next_pos = free_dir.and_then(|d| {
        let (dx, dy) = d.to_delta();
        let (x, y) = (
            (last.0 as isize + dx) as usize,
            (last.1 as isize + dy) as usize,
        );
        (x < usize::MAX && y < usize::MAX).then(|| (x, y))
    });

    // ensure that the next pos can connect to the last connector
    (
        next_pos.and_then(|pos| {
            let c = maze.get(&pos).unwrap();
            connector_dirs(c, start_dirs)
                .contains(&free_dir.unwrap().opposite())
                .then(|| (pos))
        }),
        free_dir,
    )
}

fn get_starting_connecting_dir(
    last: (usize, usize),
    maze: &HashMap<(usize, usize), char>,
    start_dirs: &Vec<Dir>,
) -> Dir {
    // get char for the pos
    let l = maze.get(&last).unwrap();
    // and get the associated dirs
    let l_dirs = connector_dirs(l, start_dirs);

    // get the direction of the start that is connected to the next point
    *start_dirs
        .into_iter()
        .filter(|d| l_dirs.contains(&d.opposite()))
        .last()
        .unwrap()
}

fn extract_main_loop(
    start: (usize, usize),
    maze: &HashMap<(usize, usize), char>,
) -> Vec<(usize, usize)> {
    // based on S position, possible neighbors direction they should expose to be able to connect
    let starting_connections = [
        ((-1, 0), Dir::WEST),
        ((0, -1), Dir::NORTH),
        ((1, 0), Dir::EAST),
        ((0, 1), Dir::SOUTH),
    ]
    .iter()
    .map(|((x, y), dir)| {
        (
            (
                (start.0 as isize + x) as usize,
                (start.1 as isize + y) as usize,
            ),
            dir,
        )
    })
    .filter(|((x, y), _dir)| x < &usize::MAX && y < &usize::MAX)
    .collect::<Vec<((usize, usize), &Dir)>>();

    let mut valid_connections = starting_connections
        .clone()
        .into_iter()
        .filter(|(pos, dir)| {
            connector_dirs(maze.get(pos).unwrap(), &vec![]).contains(&dir.opposite())
        })
        .map(|(pos, dir)| (pos, *dir))
        .collect::<Vec<((usize, usize), Dir)>>();

    let starting_opened_conn = valid_connections
        .iter()
        .map(|(_, dir)| *dir)
        .collect::<Vec<Dir>>();

    // We must have at least 2 connections for a loop to be possible
    assert!(valid_connections.len() >= 2);

    // start from a valid connection and follow the path until we find back the start
    let mut pipe_loop: Vec<(usize, usize)> = Vec::new();
    let mut full_loop = false;

    while valid_connections.len() > 0 {
        let (s, _dir) = valid_connections.pop().unwrap();
        pipe_loop = vec![start, s];

        let mut connected_dir = get_starting_connecting_dir(s, maze, &starting_opened_conn);

        // crawl
        loop {
            let last = pipe_loop[pipe_loop.len() - 1];

            match get_next_pos(last, &maze, &starting_opened_conn, &connected_dir) {
                (Some(pos), Some(free_dir)) => {
                    if start == pos {
                        full_loop = true;
                        break;
                    }
                    pipe_loop.push(pos);
                    connected_dir = free_dir;
                }
                (_, _) => break,
            }
        }

        if full_loop {
            break;
        }
    }
    pipe_loop
}

fn is_vertical(c: &char) -> bool {
    ['|', 'L', 'J'].contains(c)
}

fn count_enclosed_tiles(
    full_loop: &HashSet<(usize, usize)>,
    maze: &HashMap<(usize, usize), char>,
) -> usize {
    let maze_shape = maze
        .iter()
        .fold((0, 0), |acc, ((x, y), _)| (max(acc.0, *x), max(acc.1, *y)));
    let mut inside_count = 0;

    for y in 0..=maze_shape.1 {
        let mut inside = false;
        for x in 0..=maze_shape.0 {
            let pos = (x, y);
            // is this a wall ?
            let is_wall = full_loop.contains(&pos);
            if is_wall && is_vertical(&maze.get(&pos).unwrap()) {
                inside = !inside
            } else {
                if inside && !is_wall {
                    inside_count += 1;
                }
            }
        }
    }
    inside_count
}

#[aoc::main()]
fn main(input: &str) -> (usize, usize) {
    let (start, maze) = parse_pipe_maze(input);

    let full_loop = extract_main_loop(start, &maze);

    let p1 = full_loop.len() / 2;
    let p2 = count_enclosed_tiles(&HashSet::from_iter(full_loop), &maze);
    (p1, p2)
}
