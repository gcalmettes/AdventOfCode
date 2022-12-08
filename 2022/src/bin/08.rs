use hashbrown::HashMap;
use itertools::Itertools;

#[derive(Debug)]
struct Map {
    coords: HashMap<Coord, i32>,
    size: (i64, i64),
}

// #[derive(Debug, Hash, Eq, PartialEq)]
type Coord = (i64, i64);

impl Map {
    fn calculate_scenic_score(&self, coord: Coord) -> i64 {
        let height = self.coords.get(&coord).unwrap();

        let (x, y) = coord;
        let mut score = [0, 0, 0, 0];

        // left score
        for i in (0..x).rev() {
            score[0] += 1;
            let other_height = self.coords.get(&(i, y)).unwrap();
            if other_height >= height {
                break;
            }
        }

        // right score
        for i in x + 1..self.size.0 {
            score[1] += 1;
            let other_height = self.coords.get(&(i, y)).unwrap();
            if other_height >= height {
                break;
            }
        }

        // top score
        for j in (0..y).rev() {
            score[2] += 1;
            let other_height = self.coords.get(&(x, j)).unwrap();
            if other_height >= height {
                break;
            }
        }
        // bottom score
        for j in y + 1..self.size.1 {
            score[3] += 1;
            let other_height = self.coords.get(&(x, j)).unwrap();
            if other_height >= height {
                break;
            }
        }

        score.iter().product()
    }

    fn max_scenic_score(&self) -> i64 {
        (0..self.size.0)
            .cartesian_product(0..self.size.1)
            .map(|c| self.calculate_scenic_score(c))
            .max()
            .unwrap()
    }

    fn count_visible(&self) -> i64 {
        // Trees on the borders are all visible.
        // Don't count the corner twice.
        let mut count = (self.size.0 + self.size.1) * 2 - 4;

        // check visibility row-wise
        for (i, j) in (1..self.size.0 - 1).cartesian_product(1..self.size.1 - 1) {
            let c = (i, j);
            count += self.is_visible(c)
        }
        count
    }

    fn is_visible(&self, coord: Coord) -> i64 {
        let (x, y) = coord;

        let height = self.coords.get(&coord).unwrap();

        let mut is_visible: i64 = 0;

        'outer: loop {
            // coord on border
            if x == 0 || x == self.size.0 - 1 || y == 0 || y == self.size.1 - 1 {
                is_visible = 1;
                // visible, we break
                break 'outer;
            }

            let mut is_hidden = false;
            // check left
            for i in 0..x {
                let other_height = self.coords.get(&(i, y)).unwrap();
                if other_height >= height {
                    is_hidden = true;
                    break;
                }
            }
            if !is_hidden {
                is_visible = 1;
                break 'outer;
            }
            // check right
            is_hidden = false;
            for i in x + 1..self.size.0 {
                let other_height = self.coords.get(&(i, y)).unwrap();
                if other_height >= height {
                    is_hidden = true;
                    break;
                }
            }
            if !is_hidden {
                is_visible = 1;
                break 'outer;
            }
            // check top
            is_hidden = false;
            for j in 0..y {
                let other_height = self.coords.get(&(x, j)).unwrap();
                if other_height >= height {
                    is_hidden = true;
                    break;
                }
            }
            if !is_hidden {
                is_visible = 1;
                break 'outer;
            }
            // check bottom
            is_hidden = false;
            for j in y + 1..self.size.1 {
                let other_height = self.coords.get(&(x, j)).unwrap();
                if other_height >= height {
                    is_hidden = true;
                    break;
                }
            }
            if !is_hidden {
                is_visible = 1;
                break 'outer;
            }
            break;
        }
        is_visible
    }
}

fn parse_input(input: &str) -> Map {
    let mut coords = HashMap::new();
    let mut len_x: i64 = 0;
    let mut len_y: i64 = 0;

    input.lines().enumerate().for_each(|(j, line)| {
        line.chars().enumerate().for_each(|(i, v)| {
            len_x = i as i64;
            len_y = j as i64;
            coords.insert((i as i64, j as i64), v.to_digit(10).unwrap() as i32);
        })
    });
    Map {
        coords,
        size: (len_x + 1, len_y + 1),
    }
}

fn part1(data: &Map) -> i64 {
    data.count_visible()
}

fn part2(data: &Map) -> i64 {
    data.max_scenic_score()
}

#[aoc::main()]
fn main(input: &str) -> (i64, i64) {
    let data = parse_input(input);
    let p1 = part1(&data);
    let p2 = part2(&data);
    (p1, p2)
}
