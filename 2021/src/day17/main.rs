use std::ops::RangeInclusive;

static UPPERXBOUND: i32 = 176;
static LOWERYBOUND: i32 = -141;
static TARGETXRANGE: RangeInclusive<i32> = 119..=UPPERXBOUND;
static TARGETYRANGE: RangeInclusive<i32> = LOWERYBOUND..=-84;

fn simulate_probe_launch(mut vx: i32, mut vy: i32) -> (Option<i32>, (i32, i32)) {
    let (mut x, mut y, mut max) = (0, 0, 0);

    loop {
        x += vx;
        y += vy;
        let dir = vx.signum();
        vx -= dir;
        vy -= 1;
        if y > max {
            max = y;
        }
        match (TARGETXRANGE.contains(&x), TARGETYRANGE.contains(&y)) {
            (true, true) => return (Some(max), (vx, vy)), // in range, success
            (false, _) => {
                if vx == 0 { // not yet in target but no velocity forward
                    return (None, (vx, vy))
                }
            },
            (_, false) => {
                if y < LOWERYBOUND && vy < 0 { // we missed the target
                    return (None, (vx, vy))
                }
            },
        }
    }
}

fn parts() -> (i32, usize) {
    let mut maxs: Vec<i32> = Vec::new();
    for x in 0..=UPPERXBOUND {
        for y in LOWERYBOUND..1000 {
            match simulate_probe_launch(x, y) {
                (Some(m), _) => maxs.push(m),
                _ => {},
            }
        }
    }
    (*maxs.iter().max().unwrap(), maxs.iter().count())
}

fn main() {
    let (p1, p2) = parts();
    println!("{}", p1);
    println!("{}", p2);
}
