## Sample file for a day

```
fn parse_input(input: &str) -> Vec<usize> {
    vec![0, 1, 2]
}

fn part1(data: Vec<usize>) -> usize {
    println!("data: {:?}", data);
    1
}

fn part2(data: Vec<usize>) -> usize {
    println!("data: {:?}", data);
    2
}

#[aoc::main(05)]
fn main(input: &str) -> (usize, usize) {
    let data = parse_input(input);
    let p1 = part1(data.clone());
    let p2 = part2(data);
    (p1, p2)
}
```

To read sample input, add the suffix of the sample file as argument
of the macro (e.g.: if the file is name `05-test.in`, then use `#[aoc::main(05, "test")]`)
```
#[aoc::main(05, "test")]
fn main(input: &str) -> (usize, usize) {
    let data = parse_input(input);
    let p1 = part1(data.clone());
    let p2 = part2(data);
    (p1, p2)
}
```
