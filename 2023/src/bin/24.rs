use itertools::Itertools;

struct Hailstone {
    x: f64,
    y: f64,
    z: f64,
    dx: f64,
    dy: f64,
    dz: f64,
}

impl Hailstone {
    fn from_str(line: &str) -> Hailstone {
        let (pos, velocity) = line.split_once(" @ ").unwrap();
        let (x, y, z) = pos
            .split(", ")
            .map(|w| w.parse::<f64>().unwrap())
            .collect_tuple()
            .unwrap();
        let (dx, dy, dz) = velocity
            .split(", ")
            .map(|w| w.trim().parse::<f64>().unwrap())
            .collect_tuple()
            .unwrap();
        Hailstone {
            x,
            y,
            z,
            dx,
            dy,
            dz,
        }
    }
}

fn intersects(h1: &Hailstone, h2: &Hailstone) -> Option<(f64, f64)> {
    let m1 = h1.dy / h1.dx;
    let m2 = h2.dy / h2.dx;
    if (m2 - m1).abs() < f64::EPSILON {
        return None;
    }
    let x = (m1 * h1.x - m2 * h2.x + h2.y - h1.y) / (m1 - m2);
    let y = (m1 * m2 * (h2.x - h1.x) + m2 * h1.y - m1 * h2.y) / (m2 - m1);
    Some((x, y))
}

fn find_intersections(hailstones: &[Hailstone], start: f64, end: f64) -> usize {
    let mut intersections = 0;
    for (h1, h2) in hailstones.iter().tuple_combinations() {
        if let Some((x, y)) = intersects(&h1, &h2) {
            if (h1.dx < 0.0 && x > h1.x) || (h1.dx > 0.0 && x < h1.x) {
                continue;
            }
            if (h2.dx < 0.0 && x > h2.x) || (h2.dx > 0.0 && x < h2.x) {
                continue;
            }
            if (start..=end).contains(&x) && (start..=end).contains(&y) {
                intersections += 1;
            }
        }
    }
    intersections
}

#[aoc::main()]
fn main(input: &str) -> (usize, usize) {
    let hailstones = input
        .lines()
        .map(|line| Hailstone::from_str(line))
        .collect::<Vec<Hailstone>>();
    let p1 = find_intersections(&hailstones, 200000000000000.0, 400000000000000.0);
    (p1, 0)
}
