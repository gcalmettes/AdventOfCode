#[derive(Debug, Clone)]
struct Monkey<'m> {
    items: Vec<usize>,
    op: (&'m str, &'m str),
    test: usize,
    throw: (usize, usize),
    inspected: usize,
}

fn parse_input(input: &str) -> Vec<Monkey> {
    // fn parse_input(input: &str) -> isize {
    let mut monkeys = vec![];
    input.split("\n\n").for_each(|bloc| {
        let (_, rest) = bloc.split_once("\n").unwrap();
        let mut rest = rest.splitn(5, "\n");
        let items = rest.next().unwrap();
        let op = rest.next().unwrap();
        let test = rest.next().unwrap();
        let t = rest.next().unwrap();
        let f = rest.next().unwrap();

        let items = items
            .split_once(": ")
            .unwrap()
            .1
            .split(", ")
            .map(|item| item.parse::<usize>().unwrap())
            .collect::<Vec<_>>();
        let op = op
            .split_once(" = ")
            .unwrap()
            .1
            .split_once("old ")
            .unwrap()
            .1
            .split_once(" ")
            .unwrap();
        let test = test
            .split_once("divisible by ")
            .unwrap()
            .1
            .parse::<usize>()
            .unwrap();
        let t = t.split_once("monkey ").unwrap().1.parse::<usize>().unwrap();
        let f = f.split_once("monkey ").unwrap().1.parse::<usize>().unwrap();
        monkeys.push(Monkey {
            items,
            op,
            test,
            throw: (t, f),
            inspected: 0,
        });
    });
    monkeys
}

fn compute_worry_level(level: usize, op: &(&str, &str)) -> usize {
    match op {
        ("+", "old") => level + level,
        ("*", "old") => level * level,
        ("+", v) => level + v.parse::<usize>().unwrap(),
        ("*", v) => level * v.parse::<usize>().unwrap(),
        _ => unreachable!(),
    }
}

fn pass_test(level: usize, test: usize) -> bool {
    (level % test) == 0
}

// /// returns the greatest common divisor of n numbers
// fn gcd(nums: &[usize]) -> usize {
//     if nums.len() == 1 {
//         return nums[0];
//     }
//     let a = nums[0];
//     let b = gcd(&nums[1..]);
//     gcd_of_two_numbers(a, b)
// }

// fn gcd_of_two_numbers(a: usize, b: usize) -> usize {
//     if b == 0 {
//         return a;
//     }
//     gcd_of_two_numbers(b, a % b)
// }

fn simulate(
    data: Vec<Monkey>,
    rounds: usize,
    manage_worry_level: impl Fn(usize) -> usize,
) -> usize {
    let n = data.len();
    let mut monkeys = data.clone();
    // each round
    (0..rounds).for_each(|_| {
        // each monkey
        (0..n).for_each(|id| {
            let monkey = &monkeys[id].clone();
            monkey.items.iter().for_each(|item| {
                monkeys[id].inspected += 1;
                let new_worry_level = compute_worry_level(*item, &monkey.op);
                let new_worry_level = manage_worry_level(new_worry_level);
                let val = monkey.test;
                let pass = pass_test(new_worry_level, val);
                match pass {
                    true => monkeys[monkey.throw.0].items.push(new_worry_level),
                    false => monkeys[monkey.throw.1].items.push(new_worry_level),
                };
            });
            monkeys[id].items = vec![];
        });
    });
    monkeys.sort_by(|a, b| b.inspected.cmp(&a.inspected));
    monkeys[..2]
        .iter()
        .map(|monkey| monkey.inspected)
        .product::<usize>()
}

#[aoc::main()]
fn main(input: &str) -> (usize, usize) {
    let data = parse_input(input);
    let lcm = data.iter().map(|monkey| monkey.test).product::<usize>();
    let p1 = simulate(data.clone(), 20, |level| level / 3);
    let p2 = simulate(data.clone(), 10000, |level| level % lcm);
    (p1, p2)
}
