use itertools::Itertools;

fn solve(data: &Vec<(&usize, &usize)>) -> usize {
    data.iter()
        .map(|(t, d)| {
            (0..**t)
                .map(|i| {
                    let speed = i;
                    let remaining = *t - i;
                    speed * remaining
                })
                .filter(|dis| dis > d)
                .count()
        })
        .product()
}

#[aoc::main()]
fn main(input: &str) -> (usize, usize) {
    let data = input
        .split("\n")
        .map(|line| {
            let (_, vals) = line.trim().split_once(":").unwrap();
            vals.split_whitespace()
                .map(|d| d.parse::<usize>().ok().unwrap())
                .collect::<Vec<usize>>()
        })
        .collect::<Vec<_>>();
    let data = data[0]
        .iter()
        .zip(data[1].iter())
        .map(|(t, d)| (t, d))
        .collect::<Vec<_>>();

    let p1 = solve(&data);

    let data = input
        .split("\n")
        .map(|line| {
            let (_, vals) = line.trim().split_once(":").unwrap();
            let joined = vals.split_whitespace().join("");
            joined.parse::<usize>().ok().unwrap()
        })
        .collect::<Vec<_>>();

    let p2 = solve(&vec![(&data[0], &data[1])]);

    (p1, p2)
}
