use pathfinding::{directed::dijkstra::dijkstra, matrix::Matrix};

fn possible_move(
    blocks: &Matrix<u32>,
    pos: (usize, usize),
    dir: (isize, isize),
    current_straight_length: usize,
    min_before_turn: usize,
    max_before_turn: usize,
) -> Vec<(((usize, usize), (isize, isize), usize), u32)> {
    // the next possible neighbors
    let mut neighbors: Vec<(((usize, usize), (isize, isize), usize), u32)> = Vec::new();
    // starting point, top left corner, we can go in front or below
    if current_straight_length == 0 {
        [(1, 0), (0, 1)].into_iter().for_each(|dir| {
            if let Some(next_pos) = blocks.move_in_direction(pos, dir) {
                neighbors.push(((next_pos, dir, 1), blocks[next_pos]))
            }
        })
    } else {
        // if less than max straight length, we can continue in same dir
        if current_straight_length < max_before_turn {
            if let Some(next_pos) = blocks.move_in_direction(pos, dir) {
                // continue in same dir
                neighbors.push((
                    (next_pos, dir, current_straight_length + 1),
                    blocks[next_pos],
                ))
            }
        }

        // if more than min straight line, we can turn right and left
        if current_straight_length >= min_before_turn {
            // turn right and left
            let turn_right = (dir.1, -dir.0);
            let turn_left = (-dir.1, dir.0);
            if let Some(next_pos) = blocks.move_in_direction(pos, turn_right) {
                neighbors.push(((next_pos, turn_right, 1), blocks[next_pos]))
            }
            if let Some(next_pos) = blocks.move_in_direction(pos, turn_left) {
                neighbors.push(((next_pos, turn_left, 1), blocks[next_pos]))
            }
        }
    }
    neighbors
}

fn find_path(
    m: &Matrix<u32>,
    start: (usize, usize),
    goal: (usize, usize),
    min_before_turn: usize,
    max_before_turn: usize,
) -> usize {
    if let Some(path) = dijkstra(
        // start, (dx, dy), current length of straight line
        &(start, (0, 0), 0),
        |&(pos, (dx, dy), length)| {
            possible_move(m, pos, (dx, dy), length, min_before_turn, max_before_turn)
        },
        |&(pos, _dir, straight_length)| pos == goal && straight_length >= min_before_turn,
    ) {
        path.1 as usize
    } else {
        0
    }
}

#[aoc::main()]
fn main(input: &str) -> (usize, usize) {
    let blocks = Matrix::from_rows(
        input
            .lines()
            .map(|line| line.chars().filter_map(|c| c.to_digit(10))),
    )
    .unwrap();

    let bottom_right = (blocks.columns - 1, blocks.rows - 1);

    let p1 = find_path(&blocks, (0, 0), bottom_right, 1, 3);
    let p2 = find_path(&blocks, (0, 0), bottom_right, 4, 10);
    (p1, p2)
}
