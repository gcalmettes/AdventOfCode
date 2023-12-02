## Sample file for a day

Create a new `.rs` file in the `src/bin` folder (e.g.: `src/bin/myday.rs`).

Add the `#[aoc::main()]` macro on the `main` function.

Template:
----------

```
fn parse_input(input: &str) -> Vec<usize> {
    let _input = input;
    vec![0, 1, 2]
}

fn part1(data: &Vec<usize>) -> usize {
    println!("data: {:?}", data);
    data.iter()
        .sum()
}

fn part2(data: &Vec<usize>) -> usize {
    println!("data: {:?}", data);
    data.iter()
        .map(|x| x*2)
        .sum()
}

#[aoc::main()]
fn main(input: &str) -> (usize, usize) {
    let data = parse_input(input);
    let p1 = part1(&data);
    let p2 = part2(&data);
    (p1, p2)
}
```

To read sample input, add the suffix of the sample file as argument
of the macro (e.g.: if the file of the day is named `myday-test.in`, then use `#[aoc::main("test")]`)
```
#[aoc::main("test")]
fn main(input: &str) -> (usize, usize) {
    let data = parse_input(input);
    let p1 = part1(data.clone());
    let p2 = part2(data);
    (p1, p2)
}
```

Run the day's solution:

```
cargo run --bin myday
```
