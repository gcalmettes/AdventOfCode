use hashbrown::HashSet;
use regex::Regex;
use std::ops::Range;

type Pos = (i64, i64);

#[derive(Debug)]
struct Sensor {
    pos: Pos,
    beacon: Pos,
}

impl Sensor {
    fn distance(&self, beacon: Pos) -> i64 {
        (beacon.0 - self.pos.0).abs() + (beacon.1 - self.pos.1).abs()
    }

    fn distance_nearest_beacon(&self) -> i64 {
        self.distance(self.beacon)
    }

    fn covered_positions_on_row(&self, row: i64) -> Range<i64> {
        let distance_nearest_beacon = self.distance_nearest_beacon();
        let delta_y = (row - self.pos.1).abs();
        let extra = distance_nearest_beacon - delta_y;
        match extra >= 0 {
            true => Range {
                start: self.pos.0 - extra,
                end: self.pos.0 + extra + 1,
            },
            false => Range {
                start: self.pos.0,
                end: self.pos.0,
            },
        }
    }
}

fn parse_input(input: &str) -> Vec<Sensor> {
    let re_sensors =
        Regex::new(r"Sensor at x=(-?\d+), y=(-?\d+): closest beacon is at x=(-?\d+), y=(-?\d+)")
            .unwrap();
    re_sensors
        .captures_iter(&input)
        .map(|c| {
            let coords = c
                .iter()
                .filter_map(|m| m.unwrap().as_str().parse::<i64>().ok())
                .collect::<Vec<i64>>();
            Sensor {
                pos: (coords[0], coords[1]),
                beacon: (coords[2], coords[3]),
            }
        })
        .collect::<Vec<Sensor>>()
}

fn covered_positions_for_row(sensors: &Vec<Sensor>, row: i64) -> HashSet<i64> {
    let possible_positions = sensors
        .iter()
        .flat_map(|s| {
            let p = s
                .covered_positions_on_row(row)
                .into_iter()
                .collect::<Vec<i64>>();
            p
        })
        .collect::<HashSet<i64>>();
    possible_positions
}

fn part1(sensors: &Vec<Sensor>, row: i64) -> usize {
    let mut occupied = sensors
        .iter()
        .filter(|s| s.pos.1 == row)
        .map(|s| s.pos.0)
        .collect::<HashSet<i64>>();

    occupied.extend(
        sensors
            .iter()
            .filter(|s| s.beacon.1 == row)
            .map(|s| s.beacon.0)
            .collect::<HashSet<i64>>(),
    );
    let possible_positions = covered_positions_for_row(sensors, row);
    possible_positions
        .iter()
        .filter(|x| !occupied.contains(*x))
        .count()
}

fn part2_brute_force(sensors: &Vec<Sensor>, vmax: i64) -> i64 {
    let mut pos: Pos = (0, 0);
    for row in 0..=vmax {
        let n_possible_positions = covered_positions_for_row(sensors, row)
            .into_iter()
            .filter(|x| x >= &0 && x <= &vmax)
            .count();

        if (n_possible_positions as i64) < vmax + 1 {
            let possible_positions = covered_positions_for_row(sensors, row)
                .into_iter()
                .filter(|x| x >= &0 && x <= &vmax)
                .collect::<Vec<i64>>();
            let empty_x = (0..=vmax)
                .filter(|x| !possible_positions.contains(x))
                .collect::<Vec<i64>>();
            pos = (empty_x[0], row);

            break;
        }
    }
    pos.0 * 4_000_000 + pos.1
}

fn part2(sensors: &Vec<Sensor>, vmax: i64) -> i64 {
    let mut pos: Pos = (0, 0);
    for ((sx, sy), distance) in sensors.iter().map(|s| (s.pos, s.distance_nearest_beacon())) {
        // we check only the points just outside the border of each enveloppe distance zone
        for direction in [(-1, -1), (-1, 1), (1, 1), (1, -1)] {
            for d in 0..distance {
                let border_x = sx + direction.0 * d;
                let border_y = sy + direction.1 * (distance + 1 - d);
                if border_x < 0 || border_y < 0 || border_x > vmax || border_y > vmax {
                    // no need to check if pos is outside of bounds
                    break;
                }
                if sensors
                    .iter()
                    .all(|s| s.distance((border_x, border_y)) > s.distance_nearest_beacon())
                {
                    // println!("FOUND {:?}", (border_x, border_y));
                    pos = (border_x, border_y);
                    break;
                }
            }
        }
    }
    pos.0 * 4_000_000 + pos.1
}

#[aoc::main()]
fn main(input: &str) -> (usize, i64) {
    let data = parse_input(input);
    let p1 = part1(&data, 2_000_000);
    let p2 = part2(&data, 4_000_000);
    (p1, p2)
}
