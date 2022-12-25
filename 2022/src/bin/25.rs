fn to_decimal(s: &str) -> isize {
    s.chars().fold(0, |num, c| {
        num * 5
            + match c {
                '2' => 2,
                '1' => 1,
                '0' => 0,
                '-' => -1,
                '=' => -2,
                _ => unreachable!(),
            }
    })
}

fn to_snafu(num: isize) -> String {
    if num == 0 {
        "".to_string()
    } else {
        to_snafu((num + 2) / 5) + ["0", "1", "2", "=", "-"][num as usize % 5]
    }
}

#[aoc::main()]
fn main(input: &str) -> (String, usize) {
    let s = input.lines().map(to_decimal).sum();
    let p1 = to_snafu(s);
    (p1, 0)
}
