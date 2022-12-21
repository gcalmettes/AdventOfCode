use hashbrown::HashMap;

#[derive(Debug, Copy, Clone)]
enum Math<'a> {
    Num(u64),
    JobAdd((&'a str, &'a str)),
    JobSub((&'a str, &'a str)),
    JobMul((&'a str, &'a str)),
    JobDiv((&'a str, &'a str)),
}

impl Math<'_> {
    fn is_ready(&self, mapping: &HashMap<&str, Math>) -> bool {
        match self {
            Math::Num(_) => true,
            Math::JobAdd((a, b)) => match (mapping[a], mapping[b]) {
                (Math::Num(_), Math::Num(_)) => true,
                _ => false,
            },
            Math::JobSub((a, b)) => match (mapping[a], mapping[b]) {
                (Math::Num(_), Math::Num(_)) => true,
                _ => false,
            },
            Math::JobMul((a, b)) => match (mapping[a], mapping[b]) {
                (Math::Num(_), Math::Num(_)) => true,
                _ => false,
            },
            Math::JobDiv((a, b)) => match (mapping[a], mapping[b]) {
                (Math::Num(_), Math::Num(_)) => true,
                _ => false,
            },
        }
    }
}

fn parse_input(input: &str) -> HashMap<&str, Math> {
    let mut mapping_maths: HashMap<&str, Math> = HashMap::new();

    input.lines().for_each(|line| {
        let (monkey, maths) = line.split_once(": ").unwrap();
        let mut maths = maths.split_whitespace();
        match (maths.next(), maths.next(), maths.next()) {
            (Some(a), Some("+"), Some(b)) => {
                mapping_maths.insert(monkey, Math::JobAdd((a, b)));
            }
            (Some(a), Some("-"), Some(b)) => {
                mapping_maths.insert(monkey, Math::JobSub((a, b)));
            }
            (Some(a), Some("*"), Some(b)) => {
                mapping_maths.insert(monkey, Math::JobMul((a, b)));
            }
            (Some(a), Some("/"), Some(b)) => {
                mapping_maths.insert(monkey, Math::JobDiv((a, b)));
            }
            (Some(a), None, None) => {
                mapping_maths.insert(monkey, Math::Num(a.parse().unwrap()));
            }
            _ => unreachable!(),
        };
    });
    mapping_maths
}

fn fill_values(data: &HashMap<&str, Math>) -> HashMap<String, u64> {
    let mut maths = data.clone();

    loop {
        let to_compute = maths
            .clone()
            .into_iter()
            .filter(|(_, m)| match (m, m.is_ready(&maths)) {
                (Math::Num(_), _) => false, // already computed
                (_, true) => true,          // ready to be computed
                _ => false,                 // still missing elements
            })
            .collect::<Vec<_>>();

        if to_compute.len() == 0 {
            break;
        }

        for (k, m) in to_compute.into_iter() {
            let value = match m {
                Math::JobAdd((a, b)) => match (maths[a], maths[b]) {
                    (Math::Num(a), Math::Num(b)) => a + b,
                    _ => unreachable!(),
                },
                Math::JobSub((a, b)) => match (maths[a], maths[b]) {
                    (Math::Num(a), Math::Num(b)) => a - b,
                    _ => unreachable!(),
                },
                Math::JobMul((a, b)) => match (maths[a], maths[b]) {
                    (Math::Num(a), Math::Num(b)) => a * b,
                    _ => unreachable!(),
                },
                Math::JobDiv((a, b)) => match (maths[a], maths[b]) {
                    (Math::Num(a), Math::Num(b)) => a / b,
                    _ => unreachable!(),
                },
                _ => unreachable!(),
            };
            *maths.get_mut(k).unwrap() = Math::Num(value);
        }
    }
    let mut res = HashMap::new();

    for (k, m) in maths.into_iter() {
        let v = match m {
            Math::Num(a) => a,
            _ => unreachable!(),
        };
        res.insert(k.to_string(), v);
    }
    res
}

fn part1(data: &HashMap<&str, Math>) -> u64 {
    let values = fill_values(data);
    values["root"]
}

fn part2(data: &HashMap<&str, Math>) -> u64 {
    let mut maths = data.clone();

    let (n1, n2) = match maths["root"] {
        Math::JobAdd((a, b)) => (a, b),
        Math::JobSub((a, b)) => (a, b),
        Math::JobMul((a, b)) => (a, b),
        Math::JobDiv((a, b)) => (a, b),
        _ => unreachable!(),
    };

    let values = fill_values(data);
    let (v1, v2) = (values[n1], values[n2]);
    let mut delta: u64 = match v2 > v1 {
        true => v2 - v1,
        false => v1 - v2,
    };

    let me = "humn";
    let me_value = values[me];

    let mut to_yell = me_value;
    let mut inc: i64 = 100000000000;
    let mut prev_diff = 0;

    loop {
        to_yell = ((to_yell as i64) + inc) as u64;
        *maths.get_mut(me).unwrap() = Math::Num(to_yell);
        let values = fill_values(&maths);
        let (v1, v2) = (values[n1], values[n2]);

        let diff = match v2 > v1 {
            true => v2 - v1,
            false => v1 - v2,
        };

        if diff == 0 {
            break;
        }

        if prev_diff == diff {
            // revert change
            to_yell = ((to_yell as i64) - inc) as u64;
            // change strategy radically
            inc *= -1;
            continue;
        }

        if diff < 200 {
            // decrease granularity
            let sign = inc.signum();

            inc = 1 * sign;
            if inc.abs() == 0 {
                inc = 1;
            }
            delta = diff;
            continue;
        }

        if diff > delta {
            // revert change
            to_yell = ((to_yell as i64) - inc) as u64;
            // decrease step
            inc = inc / 2;
            delta = diff;
            prev_diff = diff;
            continue;
        }

        if inc.abs() == 0 {
            inc = 1;
        }
        delta = diff;
    }
    to_yell
}

#[aoc::main()]
fn main(input: &str) -> (u64, u64) {
    let maths = parse_input(input);
    let p1 = part1(&maths);
    let p2 = part2(&maths);
    (p1, p2)
}
