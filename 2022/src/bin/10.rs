use itertools::Itertools;

#[derive(Debug)]
enum Ins {
    NoOp,
    AddX(i32),
}

fn parse_input(input: &str) -> Vec<Ins> {
    input
        .lines()
        .map(|line| match line {
            "noop" => Ins::NoOp,
            add => {
                let (_, v) = add.split_once(" ").unwrap();
                Ins::AddX(v.parse().unwrap())
            }
        })
        .collect()
}

fn part1(data: &Vec<Ins>) -> i32 {
    // we start at cycle 0 with register value x=1
    let mut cycles = vec![(0, 1)];
    data.iter().enumerate().for_each(|(i, ins)| match ins {
        Ins::NoOp => cycles.push((cycles[i].0 + 1, cycles[i].1)),
        Ins::AddX(v) => cycles.push((cycles[i].0 + 2, cycles[i].1 + v)),
    });

    (20..=220)
        .step_by(40)
        .map(|t| {
            let x = cycles
                .iter()
                .filter(|(c, _)| c < &t)
                .sorted_by(|a, b| a.0.cmp(&b.0))
                .last()
                .unwrap()
                .1;
            x * t
        })
        .sum()
}

fn part2(data: &Vec<Ins>) -> String {
    // we start at cycle 0 with register value x=1
    let mut cycles = vec![(0, 1)];
    data.iter().enumerate().for_each(|(i, ins)| match ins {
        Ins::NoOp => cycles.push((cycles[i].0 + 1, cycles[i].1)),
        Ins::AddX(v) => cycles.push((cycles[i].0 + 2, cycles[i].1 + v)),
    });

    let mut screen: [[char; 40]; 6] = [[' '; 40]; 6];
    (1..=240).for_each(|cycle| {
        let sprite_pos = cycles
            .iter()
            .filter(|(c, _)| c < &cycle)
            .sorted_by(|a, b| a.0.cmp(&b.0))
            .last()
            .unwrap()
            .1;

        let cursor_pos = cycle - 1;
        let x = cursor_pos % 40;
        let y = cursor_pos / 40;
        if (sprite_pos - 1..=sprite_pos + 1).contains(&x) {
            screen[y as usize][x as usize] = '█'
        }
    });

    // we start by a newline for display formatting purpose
    let mut display = "\n".to_owned();
    display.push_str(
        screen
            .iter()
            .map(|chars| String::from_iter(chars))
            .join("\n")
            .as_str(),
    );
    display
}

#[aoc::main()]
fn main(input: &str) -> (i32, String) {
    let data = parse_input(input);
    let p1 = part1(&data);
    let p2 = part2(&data);
    (p1, p2)
}
